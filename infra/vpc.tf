resource "google_compute_network" "main" {
  name                    = "cordiafit-vpc"
  auto_create_subnetworks = false
}

resource "google_compute_subnetwork" "gke_subnet" {
  name          = "gke-subnet"
  ip_cidr_range = "10.0.0.0/20"
  network       = google_compute_network.main.id
  region        = var.region

  secondary_ip_range {
    range_name    = "pods"
    ip_cidr_range = "10.4.0.0/14"
  }
  secondary_ip_range {
    range_name    = "services"
    ip_cidr_range = "10.8.0.0/20"
  }
}

# Private Service Access for Cloud SQL
resource "google_compute_global_address" "private_ip_alloc" {
  name          = "sql-private-ip-alloc"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 16
  network       = google_compute_network.main.id
}

resource "google_service_networking_connection" "private_vpc_connection" {
  network                 = google_compute_network.main.id
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_ip_alloc.name]
  deletion_policy         = "ABANDON" # Prevents the 'Service Producer' lock on destroy
}

resource "google_compute_network_peering_routes_config" "peering_routes" {
  project = var.project_id
  network = google_compute_network.main.name
  peering = google_service_networking_connection.private_vpc_connection.peering

  # This is the "magic" that allows Pod ranges to reach Cloud SQL
  export_custom_routes = true
  import_custom_routes = true
}

resource "google_compute_router" "router" {
  name    = "cordiafit-router"
  region  = var.region
  network = google_compute_network.main.id
}

resource "google_compute_router_nat" "nat" {
  name                               = "cordiafit-nat"
  router                             = google_compute_router.router.name
  region                             = var.region
  nat_ip_allocate_option             = "AUTO_ONLY"
  source_subnetwork_ip_ranges_to_nat = "ALL_SUBNETWORKS_ALL_IP_RANGES"

  log_config {
    enable = true
    filter = "ERRORS_ONLY"
  }
}

resource "google_compute_firewall" "allow_keycloak_health_9000" {
  name          = "allow-keycloak-health-9000"
  direction     = "INGRESS"
  network       = "cordiafit-vpc" # Replace with your actual VPC resource/name
  priority      = 1000
  source_ranges = ["130.211.0.0/22", "35.191.0.0/16"] # Google Health Check Ranges

  allow {
    protocol = "tcp"
    ports    = ["9000"]
  }

  # This ensures it applies to your GKE nodes
  target_tags = ["gke-cordiafit-cluster"]
}
