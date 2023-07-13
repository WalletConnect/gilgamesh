resource "aws_prometheus_workspace" "prometheus" {
  alias = "prometheus-${module.this.id}"
}
