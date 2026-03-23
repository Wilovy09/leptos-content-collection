---
title: "Hello Tera"
date: "2026-03-23"
draft: false
summary: "Post de ejemplo para Actix-web + Tera + leptos-content-collection"
---

# Actix-web + Tera + leptos-content-collection

Este ejemplo combina tres piezas:

- **leptos-content-collection** — carga Markdown tipado con frontmatter YAML
- **Actix-web** — servidor HTTP asíncrono
- **Tera** — motor de plantillas estilo Jinja2

## ¿Cómo funciona?

El contenido se embebe en el binario durante `cargo build` usando `build.rs`.
En runtime, Actix-web sirve los posts sin ningún acceso al filesystem para el contenido.

Las plantillas Tera **sí** se cargan del filesystem en runtime, lo que permite
modificarlas sin recompilar el binario.

## Ejemplo de código

```rust
let mut ctx = Context::new();
ctx.insert("post", &post);
let html = tera.render("post.html", &ctx)?;
```
