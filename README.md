# Codedock Examples

Welcome to the official repository for Codedock starter templates.

This repository contains ready-to-deploy, minimal example applications built for various languages and frameworks. Codedock natively supports deploying subdirectories from Git repositories, making it incredibly easy to start your next project.

## Available Templates

* **[Node.js Express](./node-express)**
* **[Go Fiber](./go-fiber)**
* **[Python FastAPI](./python-fastapi)**
* **[Ruby Sinatra](./ruby-sinatra)**
* **[PHP Basic](./php-basic)**

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
