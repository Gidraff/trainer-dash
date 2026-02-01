resource "google_sql_database_instance" "db" {
  name             = "cordiafit-db"
  database_version = "POSTGRES_15"
  region           = var.region
  deletion_protection = false

  depends_on = [google_service_networking_connection.private_vpc_connection]

  settings {
    tier = "db-f1-micro"
    ip_configuration {
      ipv4_enabled    = false
      private_network = google_compute_network.main.id
    }
  }
}