# 1. Static Global IP for the Load Balancer
resource "google_compute_global_address" "ingress_ip" {
  name = "cordiafit-ingress-ip"
}

# 2. Cloud DNS Managed Zone
resource "google_dns_managed_zone" "primary" {
  name     = "cordiafit-zone"
  dns_name = "${var.domain}."
}

# 3. DNS Records (A-Records for all domains)

# Root Domain (cordiafit.fit)
resource "google_dns_record_set" "root" {
  name         = "${var.domain}."
  type         = "A"
  ttl          = 300
  managed_zone = google_dns_managed_zone.primary.name
  rrdatas      = [google_compute_global_address.ingress_ip.address]
}

# Auth Subdomain (auth.cordiafit.fit)
resource "google_dns_record_set" "auth" {
  name         = "auth.${var.domain}."
  type         = "A"
  ttl          = 300
  managed_zone = google_dns_managed_zone.primary.name
  rrdatas      = [google_compute_global_address.ingress_ip.address]
}

# WWW Subdomain (www.cordiafit.fit)
resource "google_dns_record_set" "www" {
  name         = "www.${var.domain}."
  type         = "A"
  ttl          = 300
  managed_zone = google_dns_managed_zone.primary.name
  rrdatas      = [google_compute_global_address.ingress_ip.address]
}

# 4. Google-Managed SSL Certificate
resource "random_id" "cert_suffix" {
  byte_length = 4
}
resource "google_compute_managed_ssl_certificate" "default" {
  name = "cordiafit-cert-${random_id.cert_suffix.hex}"
  managed {
    domains = [
      var.domain,
      "www.${var.domain}",
      "auth.${var.domain}",
      "api.${var.domain}",
      "argocd.${var.domain}",
    ]
  }
  lifecycle {
    create_before_destroy = true
  }
}

output "ingress_ip_address" {
  value = google_compute_global_address.ingress_ip.address
}

# VYfEW7uxYbzcKmA6%
