variable "name" {
  description = "Name prefix."
}

variable "project_id" {
  type        = string
  description = "GCP project id."
}

variable "region" {
  type        = string
  default     = "asia-southeast1"
  description = "The GCP region where the Cloud Run service will be deployed."
}
