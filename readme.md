### lancer le projet

```
docker build -t api .
docker run -it -p 80:8080 --name api_container api
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
