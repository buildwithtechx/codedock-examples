# Vessl Examples

Welcome to the official repository for Vessl Starter Templates. 

This repository contains ready-to-deploy, minimal example applications built for various languages and frameworks. Vessl natively supports deploying subdirectories from Git repositories, making it incredibly easy to start your next project.

## Available Templates

* **[Node.js Express](./node-express)**
* **[Go Fiber](./go-fiber)**
* **[Python FastAPI](./python-fastapi)**
* **[Ruby Sinatra](./ruby-sinatra)**
* **[PHP Basic](./php-basic)**

## How to Deploy via Dashboard

1. Navigate to the **Marketplace** in your Vessl Dashboard.
2. Select your preferred template.
3. Click **Deploy Template**.

## How to Deploy via CLI

Vessl supports deploying directly from the command line using the `--dir` flag to target a specific subdirectory within a repository.

```bash
# Example: Deploying the Go Fiber template
vessld deploy https://github.com/vesslhq/vessl-examples.git --dir go-fiber
```

## Contributing

We welcome pull requests! If you have a framework or language you'd like to see here, feel free to add a minimal boilerplate into its own directory and submit a PR.
