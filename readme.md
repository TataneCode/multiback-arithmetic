# Modular arithmetics calculation
The aim of this project is to configure and run an angular/.net/rust project

- Angular as front
- .Net as api
- Rust as grpc server to proceed with calculations

## Roadmap

### Angular
- Implement HttpClient to query the backoffice
- Implement Material usage

### .Net
- Implement postgres
- Implement remote grafana logging/metrics/tracing
- Implement grpc connection to the rust api
- Implement background service management

### Rust
- Implement calculation endpoints :
  - Euclidian theorem
  - Elgamal calculation
  - ...

### Deployment
- Write k8s deployment manifest
