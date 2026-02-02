################################################################################
# 1. NAMESPACES
################################################################################
resource "kubernetes_namespace" "argocd" {
  metadata { name = "argocd" }
}
resource "kubernetes_namespace" "cert_manager" {
  metadata { name = "cert-manager" }
}
resource "kubernetes_namespace" "keycloak" {
  metadata { name = "keycloak" }
}
resource "kubernetes_namespace" "observability" {
  metadata { name = "observability" }
}

################################################################################
# 2. KEYCLOAK APP & NETWORKING CONFIG
################################################################################

resource "kubernetes_manifest" "keycloak_backend_config" {
  manifest = {
    apiVersion = "cloud.google.com/v1"
    kind       = "BackendConfig"
    metadata = {
      name      = "keycloak-backend-config"
      namespace = kubernetes_namespace.keycloak.metadata[0].name
    }
    spec = {
      sessionAffinity = {
        affinityType         = "GENERATED_COOKIE"
        affinityCookieTtlSec = 3600
      }
      healthCheck = {
        checkIntervalSec = 30
        timeoutSec       = 5
        type             = "HTTP"
        requestPath      = "/auth/health/live"
        port             = 9000
      }
    }
  }
}

resource "kubernetes_service_account" "keycloak" {
  metadata {
    name      = "keycloak"
    namespace = kubernetes_namespace.keycloak.metadata[0].name
    annotations = {
      "iam.gke.io/gcp-service-account" = "keycloak-k8s-sa@${var.project_id}.iam.gserviceaccount.com"
    }
  }
}

resource "kubernetes_manifest" "keycloak" {
  manifest = {
    apiVersion = "apps/v1"
    kind       = "StatefulSet"
    metadata = {
      name      = "keycloak-v2"
      namespace = kubernetes_namespace.keycloak.metadata[0].name
      labels    = { app = "keycloak" }
    }
    spec = {
      replicas    = 1
      serviceName = "keycloak"
      selector    = { matchLabels = { app = "keycloak" } }
      template = {
        metadata = { labels = { app = "keycloak" } }
        spec = {
          serviceAccountName = "keycloak"
          securityContext    = { fsGroup = 1000 }
          containers = [
            {
              name  = "keycloak"
              image = "quay.io/keycloak/keycloak:26.0.0"
              args = [
                "start",
                "--hostname-strict=false",
                "--proxy-headers=xforwarded",
                "--http-enabled=true",
                "--db=postgres",
                "--http-relative-path=/auth",
                "--health-enabled=true",
                "--http-management-port=9000"
              ]
              readinessProbe = {
                httpGet = {
                  path   = "/health/ready"
                  port   = 9000
                  scheme = "HTTP"
                }
                initialDelaySeconds = 30
                periodSeconds       = 10
                timeoutSeconds      = 1
                successThreshold    = 1
                failureThreshold    = 3
              }
              env = [
                { name = "KC_HTTP_RELATIVE_PATH", value = "/auth" },
                { name = "KC_DB", value = "postgres" },
                { name = "KC_DB_URL", value = "jdbc:postgresql://127.0.0.1:5432/keycloak" },
                { name = "KC_DB_USERNAME", value = var.db_user },
                { name = "KC_DB_PASSWORD", value = var.db_password },
                { name = "KC_BOOTSTRAP_ADMIN_USERNAME", value = "admin" },
                { name = "KC_BOOTSTRAP_ADMIN_PASSWORD", value = var.db_password }
              ]
              ports = [
                { name = "http", containerPort = 8080 },
                { name = "management", containerPort = 9000 }
              ]
            },
            {
              name  = "cloud-sql-proxy"
              image = "gcr.io/cloud-sql-connectors/cloud-sql-proxy:2.8.1"
              args = [
                "--private-ip",
                "${var.project_id}:${var.region}:${google_sql_database_instance.db.name}"
              ]
              securityContext = { runAsNonRoot = true }
            }
          ]
        }
      }
    }
  }
}

resource "kubernetes_service" "keycloak" {
  metadata {
    name      = "keycloak-v2"
    namespace = kubernetes_namespace.keycloak.metadata[0].name
    annotations = {
      # Use 'default' instead of 'ports' to eliminate any ambiguity for the controller
      "cloud.google.com/backend-config" = jsonencode({ "default" = "keycloak-backend-config" })
      "cloud.google.com/neg"            = jsonencode({ "ingress" = true })
    }
  }
  spec {
    selector = { app = "keycloak" }
    port {
      name        = "http"
      port        = 80
      target_port = 8080
    }
    # Keep this port, but GKE will now ignore its native 9000 probe 
    # and use the BackendConfig settings because of the 'default' annotation above.
    port {
      name        = "management"
      port        = 9000
      target_port = 9000
    }
    type = "NodePort"
  }
}


resource "helm_release" "argocd" {
  name       = "argocd"
  repository = "https://argoproj.github.io/argo-helm"
  chart      = "argo-cd"
  version    = "7.7.11" # Use the latest stable version
  namespace  = kubernetes_namespace.argocd.metadata[0].name

  values = [
    yamlencode({
      configs = {
        params = {
          "server.insecure" = "true"
        }
      }
      global = {
        domain = "argocd.${var.domain}"
      }
      server = {
        service = {
          type = "NodePort"
        }
      }
    })
  ]
}

################################################################################
# 3. THE UNIFIED INGRESS
################################################################################
resource "kubernetes_ingress_v1" "main_ingress" {
  metadata {
    name      = "main-ingress"
    namespace = kubernetes_namespace.keycloak.metadata[0].name
    annotations = {
      "kubernetes.io/ingress.class"                 = "gce"
      "kubernetes.io/ingress.global-static-ip-name" = google_compute_global_address.ingress_ip.name
      "ingress.gcp.kubernetes.io/pre-shared-cert"   = google_compute_managed_ssl_certificate.default.name
      "kubernetes.io/ingress.allow-http"            = "false"
    }
  }

  spec {
    rule {
      host = "auth.${var.domain}"
      http {
        path {
          path      = "/"
          path_type = "Prefix"
          backend {
            service {
              name = "keycloak-v2"
              port { name = "http" } # Referenced by name for consistency
            }
          }
        }
      }
    }

    rule {
      host = var.domain
      http {
        path {
          path      = "/"
          path_type = "Prefix"
          backend {
            service {
              name = "keycloak-v2"
              port { name = "http" }
            }
          }
        }
      }
    }
  }
}

resource "kubernetes_ingress_v1" "api_ingress" {
  metadata {
    name      = "api-ingress"
    namespace = "default"
    annotations = {
      "kubernetes.io/ingress.class"                 = "gce"
      "kubernetes.io/ingress.global-static-ip-name" = google_compute_global_address.ingress_ip.name
      "ingress.gcp.kubernetes.io/pre-shared-cert"   = google_compute_managed_ssl_certificate.default.name
      "kubernetes.io/ingress.allow-http"            = "false"
    }
  }

  spec {
    rule {
      host = "api.${var.domain}"
      http {
        path {
          path      = "/"
          path_type = "Prefix"
          backend {
            service {
              name = "fitflow-api-service" # Make sure this matches your svc name
              port { number = 80 }
            }
          }
        }
      }
    }
  }
}

resource "kubernetes_ingress_v1" "argocd_ingress" {
  metadata {
    name      = "argocd-ingress"
    namespace = kubernetes_namespace.argocd.metadata[0].name
    annotations = {
      "kubernetes.io/ingress.class"                 = "gce"
      "kubernetes.io/ingress.global-static-ip-name" = google_compute_global_address.ingress_ip.name
      "ingress.gcp.kubernetes.io/pre-shared-cert"   = google_compute_managed_ssl_certificate.default.name
      "kubernetes.io/ingress.allow-http"            = "false"
    }
  }

  spec {
    rule {
      host = "argocd.${var.domain}"
      http {
        path {
          path      = "/"
          path_type = "Prefix"
          backend {
            service {
              name = "argocd-server" # Standard service name created by the Helm chart
              port { number = 80 }   # Matches the 'server.insecure=true' setting
            }
          }
        }
      }
    }
  }

  depends_on = [helm_release.argocd]
}

################################################################################
# 4. HELM RELEASES & 5. ISSUERS (Remains Unchanged)
################################################################################
# [Existing Helm and Null Resource blocks continue here...]


resource "kubernetes_config_map" "fitflow_api_config" {
  metadata {
    name = "fitflow-api-config"
  }

  data = {
    # Keycloak
    KEYCLOAK_ISSUER_URL   = "https://${var.keycloak_domain}/realms/trainer-app"
    KEYCLOAK_INTERNAL_URL = "http://keycloak-v2.keycloak.svc.cluster.local:8080/auth/realms/trainer-app"
    
    # Database Networking
    DB_HOST               = "127.0.0.1"
    DB_PORT               = "5432"
    DB_NAME               = var.db_name
    
    # Application Settings
    RUST_LOG              = var.rust_log
    APP_HOST              = var.app_host
    APP_PORT              = var.app_port
  }
}

# Secret remains the same as before...
# Secret for sensitive credentials
resource "kubernetes_secret" "fitflow_api_secrets" {
  metadata {
    name = "fitflow-api-secrets"
  }

  type = "Opaque"

  # data allows you to provide plain text in TF, 
  # which K8s then encodes to base64 automatically.
  data = {
    DB_USER = var.db_user
    DB_PASS = var.db_password
  }
}