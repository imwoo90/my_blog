---
title: "Hosting Dioxus on GitHub Pages: A Step-by-Step Guide"
date: "2024-03-21"
author: "Antigravity"
description: "Learn how to build and deploy your Dioxus web application to GitHub Pages, featuring automatic routing and base path configuration."
image_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuAI1Cr-l7Wo5cgHt_zfG7BK8WvHPYWk5oEm-9oEQ6_MylTr-gO7ZrldH3pUrirQ5dYe4yZDhwcV-arSM4h3WA2urB99awKGkr9SLyWeteJEQExthZdm_fK0mqi3c95QNudHSRPVSZIyywMm-LADZqGhXreY55_EqUksKyzJXbGh43v6TWyfjFjAPn1a4OPM0KYZ-1joKgoI6uEnbR-6-cn4GYPzcL8ari8x_9XuWah3PJcoY7eqIyd4R-RNA0bwyrgkJRXbMCMI7Qeu"
tags: ["dioxus", "web", "github-pages"]
---

Dioxus is a powerful framework for building cross-platform user interfaces with Rust. One of its most common use cases is building web applications that can be hosted for free on GitHub Pages. In this guide, we'll walk through the essential configuration steps to get your Dioxus app live.

## 1. Setting the Base Path

When hosting on GitHub Pages, your site is often served at a subpath (e.g., `https://username.github.io/repo-name/`). For Dioxus to resolve assets and routes correctly, you must specify this in your `Dioxus.toml`.

```toml
[web.app]
base_path = "repo-name"
```

## 2. Using the Dioxus CLI

The `dx` tool simplifies the build process. To create a production-ready bundle for the web, run:

```bash
dx build --release
```

This will generate a `dist` (or `docs`) folder containing your `index.html`, compiled WebAssembly, and assets.

## 3. GitHub Actions for Automatic Deployment

You can automate the deployment process using GitHub Actions. Create a file at `.github/workflows/deploy.yml`:

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: ["main"]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Install Dioxus CLI
        run: cargo install dioxus-cli
      - name: Build Web App
        run: dx build --release
      - name: Deploy
        uses: JamesIves/github-pages-deploy-action@v4
        with:
          folder: dist
```

## 4. Handling 404s on Refresh

GitHub Pages is a static host, which means direct navigation to routes like `/blog/post-1` might result in a 404 error if you refresh the page. A common workaround is to duplicate `index.html` as `404.html` in your distribution folder.

## Conclusion

Hosting with Dioxus on GitHub Pages is efficient and cost-effective. By combining Rust's performance with GitHub's hosting, you can create fast, modern web experiences with minimal overhead.

Happy coding!
