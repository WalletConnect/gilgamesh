output "dashboard_definition" {
  value = data.jsonnet_file.dashboard.rendered
}
