resource "google_sql_database_instance" "db" {
  name                = "cordiafit-db"
  database_version    = "POSTGRES_15"
  region              = var.region
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

# Create the actual 'keycloak' database inside your instance
resource "google_sql_database" "keycloak_db" {
  name     = "keycloak"
  instance = google_sql_database_instance.db.name

  # Optional but recommended for Postgres 12+
  charset   = "UTF8"
  collation = "en_US.UTF8"
}

resource "google_sql_user" "db_user" {
  name     = var.db_user
  instance = google_sql_database_instance.db.name
  password = var.db_password
}
resource "google_sql_database" "api_db" {
  name     = "fitflow_db"
  instance = google_sql_database_instance.db.name
}
resource "google_sql_user" "api_user" {
  name     = "fitflow_admin"
  instance = google_sql_database_instance.db.name
  password = var.db_password
}
