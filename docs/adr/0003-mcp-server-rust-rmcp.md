# ADR 0003: servidor MCP Rust con rmcp

## Estado

Superseded by [ADR 0007](./0007-repository-convergence.md). Se conserva como
registro de una integración ya retirada y no como interfaz soportada.

Todo el contenido siguiente describe una implementación retirada. No existe
una compatibilidad soportada; ADR 0007 es la decisión de reemplazo.

## Contexto

La primera feature formal entrego `translator-core` y `translator-cli` en Rust,
con contratos, limites, preservacion Markdown/texto, privacidad, redaccion y
proveedor mock offline.

La planeacion inicial contemplaba un servidor MCP TypeScript que llamara al CLI
Rust. Esa decision tenia sentido antes de validar el core y antes de revisar el
estado actual del ecosistema MCP Rust.

Para la segunda feature formal, el objetivo es exponer el core como servidor MCP
con `translate_text` y `translate_file`, sin proveedor real, sin red, sin wrapper
Zed y sin modificar buffers.

## Decision

F005 implementara el servidor MCP como crate Rust nuevo:

```text
crates/translator-mcp/
```

El servidor usara `rmcp` como SDK Rust MCP y transporte stdio. El servidor
llamara a `translator-core` directamente, no a `translator-cli` como subprocess.

F005 no agregara:

- runtime Node o TypeScript;
- Vitest;
- HTTP/Streamable HTTP;
- proveedor real;
- wrapper Zed;
- `extension.toml`;
- publicacion en registry MCP o Zed.

El wrapper Zed quedo como F006 y arranca el binario MCP por comando para el
flujo local validado.

## Razon

- `translator-core` ya concentra seguridad, limites, validacion de archivos,
  redaccion y proveedor mock.
- Llamar al core directamente evita una frontera extra TypeScript -> CLI ->
  core y reduce lugares donde podrian filtrarse texto, paths o secretos.
- `rmcp` ya ofrece soporte Rust para tools y stdio.
- Stdio encaja con el modelo de Zed para extensiones MCP que arrancan un comando
  de servidor.
- Evitar Node mantiene el host y el proyecto mas simples para esta iteracion.

## Consecuencias

- Las decisiones previas sobre servidor MCP TypeScript quedan reemplazadas para
  F005.
- El workspace gana un tercer crate Rust.
- Las pruebas MCP se escribiran en Rust dentro del contenedor del proyecto.
- El CLI Rust se conserva como frontera de usuario/contrato de feature 001, pero
  ya no es el puente interno del servidor MCP.
- Si en una iteracion futura se requiere interoperabilidad TypeScript o un
  wrapper complejo, se evaluara en F006 o posteriores.

## Criterio de revision

Revisar esta decision si:

- `rmcp` deja de ser mantenible o compatible con Zed;
- Zed cambia su modelo recomendado para servidores MCP;
- se decide publicar por registry MCP antes del wrapper Zed;
- se habilita un transporte HTTP o proveedor remoto;
- el servidor MCP necesita compartir codigo con clientes no Rust.
