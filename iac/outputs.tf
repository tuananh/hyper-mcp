output "url" {
  value = google_cloud_run_service.my-app.status[0].url
}