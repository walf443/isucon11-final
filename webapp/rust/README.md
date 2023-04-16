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
This crate handles DB code. don't handle HTTP

```mermaid

flowchart LR
  ServiceInfra --> RepositoryImpl
  RepositoryImpl --> DB[(MySQL)]
  RepositoryImpl -- models --> ServiceInfra
```

## core
core application API. don't handle HTTP

