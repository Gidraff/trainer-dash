output "kubernetes_cluster_name" {
  value = google_container_cluster.primary.name
}

output "cloud_sql_private_ip" {
  value = google_sql_database_instance.db.private_ip_address
}

output "ingress_static_ip" {
  value = google_compute_global_address.ingress_ip.address
}

output "dns_name_servers" {
  value = google_dns_managed_zone.primary.name_servers
}


