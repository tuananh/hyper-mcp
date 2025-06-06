output "url" {
  value = format("%s/mcp", google_cloud_run_service.my-app.status[0].url)
}
