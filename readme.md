### lancer le projet

```
docker build -t api .
docker run -it -p 80:8080 --name api_container api
```

pour verifier l'api

```
curl localhost:80/ping
```

scout pour verifier les possibles vulns

```abel@abel ~/Documents/devops/api $ d scout cves api
    i New version 1.17.0 available (installed version is 1.16.1) at https://github.com/docker/scout-cli
    ✓ Image stored for indexing
    ✓ Indexed 19 packages
    ✓ No vulnerable package detected


## Overview

                    │       Analyzed Image
────────────────────┼──────────────────────────────
  Target            │  api:latest
    digest          │  b4b1c05c128e
    platform        │ linux/amd64
    vulnerabilities │    0C     0H     0M     0L
    size            │ 4.3 MB
    packages        │ 19


## Packages and Vulnerabilities

  No vulnerable packages detected
```

### docker-compose

```
docker compose build
docker compose up
```

maintenant sur un curl

```
api-4    | lets go
nginx-1  | 172.20.0.1 - - [12/May/2025:11:18:26 +0000] "GET / HTTP/1.1" 404 0 "-" "curl/8.13.0"
api-2    | lets go
nginx-1  | 172.20.0.1 - - [12/May/2025:11:18:27 +0000] "GET / HTTP/1.1" 404 0 "-" "curl/8.13.0"
api-3    | lets go
nginx-1  | 172.20.0.1 - - [12/May/2025:11:18:30 +0000] "GET / HTTP/1.1" 404 0 "-" "curl/8.13.0"
api-1    | lets go
nginx-1  | 172.20.0.1 - - [12/May/2025:11:18:30 +0000] "GET / HTTP/1.1" 404 0 "-" "curl/8.13.0"
api-4    | lets go
nginx-1  | 172.20.0.1 - - [12/May/2025:11:18:35 +0000] "GET / HTTP/1.1" 404 0 "-" "curl/8.13.0"
```

load balancing sur les differents containers
