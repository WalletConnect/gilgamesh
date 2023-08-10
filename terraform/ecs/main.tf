locals {
  service_name               = "${module.this.id}-service"
  file_descriptor_soft_limit = pow(2, 18)
  file_descriptor_hard_limit = local.file_descriptor_soft_limit * 2
}

# Log Group for our App
resource "aws_cloudwatch_log_group" "cluster_logs" {
  name              = "${module.this.id}_logs"
  retention_in_days = 14
}

# ECS Cluster
resource "aws_ecs_cluster" "app_cluster" {
  name = module.this.id

  configuration {
    execute_command_configuration {
      logging = "OVERRIDE"

      log_configuration {
        cloud_watch_encryption_enabled = false
        cloud_watch_log_group_name     = aws_cloudwatch_log_group.cluster_logs.name
      }
    }
  }
}

## Task Definition
resource "aws_ecs_task_definition" "app_task_definition" {
  family = module.this.id
  cpu    = var.cpu
  memory = var.memory
  requires_compatibilities = [
    "FARGATE"
  ]
  network_mode       = "awsvpc" # Required because of fargate
  execution_role_arn = aws_iam_role.ecs_task_execution_role.arn
  task_role_arn      = aws_iam_role.ecs_task_execution_role.arn
  container_definitions = jsonencode([
    {
      name  = module.this.id,
      image = var.image,
      cpu   = var.cpu - 128, # Remove sidecar memory/cpu so rest is assigned to primary container
      ulimits = [{
        name : "nofile",
        softLimit : local.file_descriptor_soft_limit,
        hardLimit : local.file_descriptor_hard_limit
      }],
      memory    = var.memory - 128,
      essential = true,
      portMappings = [
        {
          containerPort = 8080,
          hostPort      = 8080
        },
        {
          containerPort = 8081,
          hostPort      = 8081
        }
      ],
      environment = [
        { name = "PORT", value = "8080" },
        { name = "PUBLIC_URL", value = "http://localhost:8080" }, // TODO: Change this to the actual public URL
        { name = "LOG_LEVEL", value = var.log_level },
        { name = "MONGO_ADDRESS", value = var.docdb-connection_url },
        { name = "TELEMETRY_PROMETHEUS_PORT", value = "8081" }
      ],
      dependsOn = [
        { containerName = "aws-otel-collector", condition = "START" }
      ],
      logConfiguration = {
        logDriver = "awslogs",
        options = {
          awslogs-group         = aws_cloudwatch_log_group.cluster_logs.name,
          awslogs-region        = var.log_region,
          awslogs-stream-prefix = "ecs"
        }
      }
    },
    {
      name   = "aws-otel-collector",
      image  = "public.ecr.aws/aws-observability/aws-otel-collector:latest",
      cpu    = 128,
      memory = 128,
      environment = [
        { "name" : "AWS_PROMETHEUS_SCRAPING_ENDPOINT", "value" : "0.0.0.0:8081" },
        { name = "AWS_PROMETHEUS_ENDPOINT", value = "${var.prometheus_endpoint}api/v1/remote_write" },
        { name = "AWS_REGION", value = "eu-central-1" }
      ],
      essential = true,
      command = [
        "--config=/etc/ecs/ecs-amp-xray-prometheus.yaml"
      ],
      logConfiguration = {
        logDriver = "awslogs",
        options = {
          awslogs-create-group  = "True",
          awslogs-group         = "/ecs/${module.this.id}-ecs-aws-otel-sidecar-collector",
          awslogs-region        = var.log_region,
          awslogs-stream-prefix = "ecs"
        }
      }
    }
  ])

  runtime_platform {
    operating_system_family = "LINUX"
  }
}

## Service
resource "aws_ecs_service" "app_service" {
  name            = local.service_name
  cluster         = aws_ecs_cluster.app_cluster.id
  task_definition = aws_ecs_task_definition.app_task_definition.arn
  launch_type     = "FARGATE"
  desired_count   = 1
  propagate_tags  = "TASK_DEFINITION"

  # Wait for the service deployment to succeed
  wait_for_steady_state = true

  # Allow external changes without Terraform plan difference
  lifecycle {
    ignore_changes = [desired_count]
  }

  network_configuration {
    subnets          = var.private_subnets
    assign_public_ip = true                                # We do public ingress through the LB
    security_groups  = [aws_security_group.app_ingress.id] # Setting the security group
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.target_group.arn # Referencing our target group
    container_name   = module.this.id
    container_port   = 8080 # Specifying the container port
  }
}

# Autoscaling
# We can scale by
# ECSServiceAverageCPUUtilization, ECSServiceAverageMemoryUtilization, and ALBRequestCountPerTarget
# out of the box or use custom metrics
resource "aws_appautoscaling_target" "ecs_target" {
  min_capacity       = 1
  max_capacity       = 3
  resource_id        = "service/${aws_ecs_cluster.app_cluster.name}/${aws_ecs_service.app_service.name}"
  scalable_dimension = "ecs:service:DesiredCount"
  service_namespace  = "ecs"
}

resource "aws_appautoscaling_policy" "cpu_scaling" {
  name               = "${module.this.id}-application-scaling-policy-cpu"
  policy_type        = "TargetTrackingScaling"
  resource_id        = aws_appautoscaling_target.ecs_target.resource_id
  scalable_dimension = aws_appautoscaling_target.ecs_target.scalable_dimension
  service_namespace  = aws_appautoscaling_target.ecs_target.service_namespace

  target_tracking_scaling_policy_configuration {
    predefined_metric_specification {
      predefined_metric_type = "ECSServiceAverageCPUUtilization"
    }
    target_value       = 30
    scale_in_cooldown  = 180
    scale_out_cooldown = 90
  }
  depends_on = [aws_appautoscaling_target.ecs_target]
}

resource "aws_appautoscaling_policy" "memory_scaling" {
  name               = "${module.this.id}-application-scaling-policy-memory"
  policy_type        = "TargetTrackingScaling"
  resource_id        = aws_appautoscaling_target.ecs_target.resource_id
  scalable_dimension = aws_appautoscaling_target.ecs_target.scalable_dimension
  service_namespace  = aws_appautoscaling_target.ecs_target.service_namespace

  target_tracking_scaling_policy_configuration {
    predefined_metric_specification {
      predefined_metric_type = "ECSServiceAverageMemoryUtilization"
    }
    target_value       = 30
    scale_in_cooldown  = 180
    scale_out_cooldown = 90
  }
  depends_on = [aws_appautoscaling_target.ecs_target]
}


# Load Balancers & Networking
resource "aws_lb" "application_load_balancer" {
  name               = module.this.id
  load_balancer_type = "application"
  subnets            = var.public_subnets

  security_groups = [aws_security_group.lb_ingress.id]
}

resource "aws_lb_target_group" "target_group" {
  name        = module.this.id
  port        = 8080
  protocol    = "HTTP"
  target_type = "ip"
  vpc_id      = var.vpc_id # Referencing the default VPC
  slow_start  = 30         # Give a 30 second delay to allow the service to startup

  health_check {
    protocol            = "HTTP"
    path                = "/health" # Echo Server's health path
    port                = 8080
    interval            = 15
    timeout             = 10
    healthy_threshold   = 3
    unhealthy_threshold = 3
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_lb_listener" "listener" {
  load_balancer_arn = aws_lb.application_load_balancer.arn # Referencing our load balancer
  port              = "443"
  protocol          = "HTTPS"
  certificate_arn   = var.acm_certificate_arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.target_group.arn # Referencing our target group
  }
}

resource "aws_lb_listener" "listener-http" {
  load_balancer_arn = aws_lb.application_load_balancer.arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type = "redirect"

    redirect {
      port        = "443"
      protocol    = "HTTPS"
      status_code = "HTTP_301"
    }
  }
}

# DNS Records
resource "aws_route53_record" "dns_load_balancer" {
  zone_id = var.route53-zone_id
  name    = var.route53-fqdn
  type    = "A"

  alias {
    name                   = aws_lb.application_load_balancer.dns_name
    zone_id                = aws_lb.application_load_balancer.zone_id
    evaluate_target_health = true
  }
}

# IAM
resource "aws_iam_role" "ecs_task_execution_role" {
  name               = "${module.this.id}-ecs-task-execution-role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json
}

data "aws_iam_policy_document" "assume_role_policy" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["ecs-tasks.amazonaws.com"]
    }
  }
}

resource "aws_iam_role_policy_attachment" "ecs_task_execution_role_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

resource "aws_iam_role_policy_attachment" "prometheus_write_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonPrometheusRemoteWriteAccess"
}

resource "aws_iam_role_policy_attachment" "cloudwatch_write_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/CloudWatchLogsFullAccess"
}

resource "aws_iam_role_policy_attachment" "ssm_read_only_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMReadOnlyAccess"
}

resource "aws_iam_role_policy_attachment" "ecs_task_execution_role_xray_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/AWSXRayDaemonWriteAccess"
}

data "aws_iam_policy_document" "otel" {
  statement {
    actions = [
      "logs:PutLogEvents",
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:DescribeLogStreams",
      "logs:DescribeLogGroups",
      "xray:PutTraceSegments",
      "xray:PutTelemetryRecords",
      "xray:GetSamplingRules",
      "xray:GetSamplingTargets",
      "xray:GetSamplingStatisticSummaries",
      "ssm:GetParameters",
    ]
    resources = [
      "*"
    ]
  }
}
resource "aws_iam_policy" "otel" {
  name   = "${module.this.id}-otel"
  path   = "/"
  policy = data.aws_iam_policy_document.otel.json
}
resource "aws_iam_role_policy_attachment" "ecs_task_execution_fetch_ghcr_secret_policy" {
  role       = aws_iam_role.ecs_task_execution_role.name
  policy_arn = aws_iam_policy.otel.arn
}

# Security Groups
resource "aws_security_group" "app_ingress" {
  name        = "${module.this.id}-ingress-to-app"
  description = "Allow app port ingress"
  vpc_id      = var.vpc_id

  ingress {
    from_port       = 0
    to_port         = 0
    protocol        = "-1"
    security_groups = [aws_security_group.lb_ingress.id]
  }

  ingress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = [var.allowed_app_ingress_cidr_blocks]
  }

  egress {
    from_port   = 0             # Allowing any incoming port
    to_port     = 0             # Allowing any outgoing port
    protocol    = "-1"          # Allowing any outgoing protocol
    cidr_blocks = ["0.0.0.0/0"] # Allowing traffic out to all IP addresses
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_security_group" "lb_ingress" {
  name        = "${module.this.id}-lb-ingress"
  description = "Allow app port ingress from vpc"
  vpc_id      = var.vpc_id

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"] # Allowing traffic in from all sources
  }

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"] # Allowing traffic in from all sources
  }

  egress {
    from_port   = 0                                    # Allowing any incoming port
    to_port     = 0                                    # Allowing any outgoing port
    protocol    = "-1"                                 # Allowing any outgoing protocol
    cidr_blocks = [var.allowed_lb_ingress_cidr_blocks] # Allowing traffic out to all VPC IP addresses
  }

  lifecycle {
    create_before_destroy = true
  }
}
