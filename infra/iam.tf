# 1. Create the Google Service Account for DNS management
resource "google_service_account" "cert_manager_solver" {
  account_id   = "cert-manager-dns01-solver"
  display_name = "Cert Manager DNS01 Solver"
}

# 2. Grant the Service Account 'DNS Admin' permissions
resource "google_project_iam_member" "dns_admin" {
  project = var.project_id
  role    = "roles/dns.admin"
  member  = "serviceAccount:${google_service_account.cert_manager_solver.email}"
}

# 3. Allow Kubernetes to impersonate this GSA (Workload Identity)
resource "google_service_account_iam_member" "workload_identity_user" {
  service_account_id = google_service_account.cert_manager_solver.name
  role               = "roles/iam.workloadIdentityUser"

  # This links the GSA to the cert-manager KSA in your cluster
  member = "serviceAccount:${var.project_id}.svc.id.goog[cert-manager/cert-manager]"
}

# Allow the Keycloak KSA to impersonate a Google Service Account
resource "google_project_iam_member" "keycloak_sql_client" {
  project = var.project_id
  role    = "roles/cloudsql.client"
  member  = "serviceAccount:${var.project_id}.svc.id.goog[keycloak/keycloak]"
}

# 1. Bind the Kubernetes Service Account to the GCP Service Account
resource "google_service_account" "keycloak_sa" {
  account_id   = "keycloak-k8s-sa"
  display_name = "Keycloak GKE Service Account"
}

# 2. Allow Keycloak to act as a Cloud SQL Client
resource "google_project_iam_member" "keycloak_db_access" {
  project = var.project_id
  role    = "roles/cloudsql.client"
  member  = "serviceAccount:${google_service_account.keycloak_sa.email}"
}

resource "google_service_account_iam_member" "keycloak_workload_identity" {
  service_account_id = "projects/${var.project_id}/serviceAccounts/keycloak-k8s-sa@${var.project_id}.iam.gserviceaccount.com"
  role               = "roles/iam.workloadIdentityUser"
  member             = "serviceAccount:${var.project_id}.svc.id.goog[keycloak/keycloak]"
}
