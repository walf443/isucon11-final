# What is this

Writing a sample of [actix-web](https://actix.rs/) application base on [isucon/isucon11-final](https://github.com/isucon/isucon11-final) for my study purpose.

# How to run test

```
$ docker-compose up -d
```

# Architecture Overview

```mermaid

flowchart LR
  http-app --> http-core
  http-app --> infra
  http-app --> core
  http-core --> core
  infra --> core
  infra --> infra-storage-file
  infra-storage-file --> core
```

## http-app
This crate handles http request using actix-web.
combine infra API.

## http-core
This crate does not depend on infra layer.

```mermaid

flowchart LR
  http -- requests --> routes
  routes --> core
  core -- modles --> routes
  routes -- responses --> http
```

## infra
This crate handles DB or storage codes. don't handle HTTP

```mermaid

flowchart LR
  ServiceInfra --> RepositoryInfra
  ServiceInfra --> StorageInfra
  RepositoryInfra --> DB[(MySQL)]
  RepositoryInfra -- models --> ServiceInfra
  StorageInfra --> ObjectStorage[(LocalFile)]
```

## infra-storage-file
This crate implement file storage code.

## core
core application API. don't handle HTTP

