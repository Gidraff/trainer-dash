variable "project_id" {
  type        = string
  description = "The GCP Project ID where resources will be built."
}

variable "region" {
  type        = string
  description = "The GCP region for the cluster and database."
  default     = "us-central1"
}

variable "domain" {
  type        = string
  description = "The primary domain name."
  default     = "cordiafit.fit"
}

variable "node_count" {
  type        = number
  description = "Number of nodes per zone in the GKE cluster."
  default     = 2
}

# --- Database Variables ---
variable "db_name" {
  type    = string
  default = "trainerdb"
}

variable "db_user" {
  type    = string
  default = "trainer"
}

variable "db_password" {
  type        = string
  description = "Password for the Cloud SQL user."
  sensitive   = true
}

variable "rust_log" {
  description = "Logging level for the Rust application (error, warn, info, debug, trace)"
  type        = string
  default     = "info"
}

variable "app_host" {
  description = "The host address the API binds to"
  type        = string
  default     = "0.0.0.0"
}

variable "app_port" {
  description = "The port the API listens on"
  type        = string
  default     = "8080"
}
variable "keycloak_domain" {
  description = "The Keycloak domain"
  type        = string
  default     = "auth.cordiafit.fit"
}
