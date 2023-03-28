config {
  format = "default"
  module = true
}

plugin "terraform" {
  enabled = true
  preset  = "all"
}

rule "terraform_workspace_remote" {
  enabled = false
}
