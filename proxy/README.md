## Docker proxy
This is a Caddy proxy for simple users who choose to use the `docker-compose` method of hosting. This will automatically run the API and client through a single port and handle local file storage.

### Environment Variables
- PROXY_FILES  
  Should files be proxied to the API. This should likely be enabled if using both `LOCAL_SERVE` and local `STORAGE_PROVIDER`