output "prometheus_url" {
  description = "The URL of the Prometheus server to use for this dashboard."
  value       = local.prometheus_url
}

output "dashboard_definition" {
  description = "The JSON definition of the dashboard."
  value       = data.jsonnet_file.dashboard.rendered
}

output "prometheus_endpoint" {
  description = "The URL of the Prometheus server to use for this dashboard."
  value       = aws_prometheus_workspace.prometheus.prometheus_endpoint
}
