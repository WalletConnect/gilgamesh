module "this" {
  source  = "app.terraform.io/wallet-connect/label/null"
  version = "0.2.0"

  namespace   = "walletconnect"
  environment = var.region
  stage       = terraform.workspace
  name        = "history"
}
