resource "google_container_cluster" "primary" {
  name       = "cordiafit-cluster"
  location   = var.region
  network    = google_compute_network.main.id
  subnetwork = google_compute_subnetwork.gke_subnet.id

  remove_default_node_pool = true
  initial_node_count       = 1
  deletion_protection      = false

  workload_identity_config {
    workload_pool = "${var.project_id}.svc.id.goog"
  }

  ip_allocation_policy {
    cluster_secondary_range_name  = "pods"
    services_secondary_range_name = "services"
  }
}

resource "google_container_node_pool" "primary_nodes" {
  name       = "primary-pool"
  cluster    = google_container_cluster.primary.name
  location   = var.region
  node_count = var.node_count

  node_config {
    machine_type = "e2-standard-2" # Sufficient for Keycloak + Rust API
    oauth_scopes = ["https://www.googleapis.com/auth/cloud-platform"]
  }
}
