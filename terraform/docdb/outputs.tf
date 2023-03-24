output "endpoint" {
  description = "The connection endpoint"
  value       = aws_docdb_cluster.docdb_primary.endpoint
}

output "username" {
  description = "The master username for the cluster"
  value       = aws_docdb_cluster.docdb_primary.master_username
}

output "password" {
  description = "The master password for the cluster"
  value       = aws_docdb_cluster.docdb_primary.master_password
}

output "port" {
  description = "The port on which the cluster accepts connections"
  value       = aws_docdb_cluster.docdb_primary.port
}

output "connection_url" {
  description = "The connection url for the cluster"
  value       = "mongodb://${aws_docdb_cluster.docdb_primary.master_username}:${aws_docdb_cluster.docdb_primary.master_password}@${aws_docdb_cluster.docdb_primary.endpoint}:${aws_docdb_cluster.docdb_primary.port}/${var.default_database}?tls=true&tlsCaFile=rds-combined-ca-bundle.pem&tlsAllowInvalidCertificates=true&replicaSet=rs0&readPreference=secondaryPreferred&retryWrites=false&minPoolSize=32&maxPoolSize=256&maxIdleTimeMS=30000&connectTimeoutMS=30000"
}

output "cluster_id" {
  description = "The DocDB Cluster ID"
  value       = aws_docdb_cluster.docdb_primary.cluster_identifier
}
