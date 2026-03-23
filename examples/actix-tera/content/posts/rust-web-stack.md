---
title: "Stack web con Rust"
date: "2026-03-20"
draft: false
summary: "Actix + Tera es una combinación poderosa para SSR tradicional"
---

# Stack web con Rust: Actix + Tera

Para proyectos donde no necesitas interactividad en el cliente, el SSR tradicional
con plantillas es una excelente opción.

## Ventajas

- Sin JavaScript en el cliente
- HTML semántico listo para SEO
- Binario único con el contenido embebido

## Estructura del proyecto

```
src/main.rs        — rutas y handlers de Actix-web
templates/         — plantillas Tera (base, index, post, 404)
content/posts/     — archivos Markdown con frontmatter
build.rs           — genera el código embebido en compilación
```

## Tera: herencia de plantillas

Tera soporta herencia via `{% extends %}` y bloques `{% block %}`,
lo que permite compartir el layout base entre todas las páginas sin repetición.
