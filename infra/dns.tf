# Static IP for the Ingress Load Balancer
resource "google_compute_global_address" "frontend_ip" {
  name = "cordiafit-static-ip"
}

# Cloud DNS Managed Zone
resource "google_dns_managed_zone" "primary" {
  name     = "cordiafit-zone"
  dns_name = "${var.domain}."
}

# A Record pointing cordiafit.fit to the static IP
resource "google_dns_record_set" "root" {
  name         = "${var.domain}."
  type         = "A"
  ttl          = 300
  managed_zone = google_dns_managed_zone.primary.name
  rrdatas      = [google_compute_global_address.frontend_ip.address]
}