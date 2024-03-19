# HTMX - Todo List

<img src="./public/static/favicon.svg" height="60px">

A simple todo list web application built using [htmx](https://htmx.org/) on a Rust [Axum](https://github.com/tokio-rs/axum) REST API server that is styled using [Tailwind CSS](https://tailwindcss.com/) and generating server-side HTML with [Askama](https://github.com/djc/askama) templating with real-time updates (only the counter at the top right).

This is my first application using htmx and it was interesting and differs from the React mentality.

## Getting started

If you just want to run the application, just use the associated Dockerfile, build it and run it. The application should be available at port `3000`:

```
docker build -t htmx-todo-list:latest .
docker run -p 3000:3000 htmx-todo-list:latest
```

If you want to develop the application, you need Rust installed and I recommend [pnpm](https://pnpm.io/) alongside [Node.js](https://nodejs.org/) to build the styles. Run the following to have the application ready:

```
cargo run
```

### Building Tailwind CSS styles

You'll need Node.js installed and they can be built using the following pnpm command:

```
pnpm dlx tailwindcss -i tailwind.css -o public/static/main.css --watch --minify
```
