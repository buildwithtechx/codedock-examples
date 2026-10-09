# Codedock Examples

Welcome to the official repository for Codedock starter templates.

This repository contains ready-to-deploy, minimal example applications built for various languages and frameworks. Codedock natively supports deploying subdirectories from Git repositories, making it incredibly easy to start your next project.

## Available Templates

* **[Deno HTTP](./deno-http)**
* **[Docker Node](./docker-node)**
* **[Go Fiber](./go-fiber)**
* **[Go Gin](./go-gin)**
* **[Go Standard Library](./go-stdlib)**
* **[Java Spring](./java-spring)**
* **[Node.js Express](./node-express)**
* **[Node.js Fastify](./node-fastify)**
* **[Node.js Next.js](./node-nextjs)**
* **[PHP Basic](./php-basic)**
* **[PHP Slim](./php-slim)**
* **[Python Django](./python-django)**
* **[Python FastAPI](./python-fastapi)**
* **[Python Flask](./python-flask)**
* **[Ruby Sinatra](./ruby-sinatra)**
* **[Rust Axum](./rust-axum)**
* **[Static HTML](./static-html)**

Each template ships a `templates.json` entry with its display name, description, and logo, which the Codedock dashboard reads to render the examples catalogue.

## How to Deploy via Dashboard

1. Open a project in your Codedock dashboard and go to **Add New Resource**.
2. Select the **Example Projects** tab.
3. Click **Deploy** on your preferred template and review the setup.

## How to Deploy via CLI

Codedock supports deploying official templates directly from the command line using the `--template` shorthand.

```bash
# Example: Deploying the Go Fiber template
codedockd deploy --template go-fiber
```

## Contributing

We welcome pull requests! If you have a framework or language you'd like to see here, add a minimal boilerplate into its own directory, add a matching `templates.json` entry with a logo under `logos/`, and submit a PR.