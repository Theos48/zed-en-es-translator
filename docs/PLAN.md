# Plan de desarrollo

## Fuentes de verdad

La documentacion se divide por autoridad para evitar deriva:

1. `.specify/memory/constitution.md` gobierna principios no negociables.
2. `specs/<feature>/` gobierna la implementacion de la feature activa:
   requisitos, plan tecnico, contratos, tareas y quickstart.
3. `docs/decisions.md` y `docs/adr/` registran decisiones estables de producto,
   arquitectura y proceso.
4. `docs/research/`, `docs/product.md`, `docs/diagrams.md` y este plan son
   contexto estrategico; no duplican el detalle operativo de una feature activa.

Si una tarea cambia limites, alcance, seguridad, tecnologias o arquitectura,
se actualiza la feature activa y tambien `docs/decisions.md` o un ADR. Si solo
avanza la implementacion esperada, se actualiza `specs/<feature>/`.

## Estado actual

Spec Kit esta inicializado. La primera feature formal fue implementada y
validada:

```text
specs/001-translation-core-contract/
```

El primer ciclo formal entrega una base tecnica offline: core Rust, CLI Rust,
`MockProvider`, contrato JSON, limites, seguridad de lectura de archivos
Markdown/texto y pruebas negativas. La validacion registrada pasa con
`make test` dentro del contenedor Rust fijado por el proyecto.

La segunda feature formal fue implementada y validada:

```text
specs/002-mcp-server/
```

Esta iteracion expone el core de traduccion como servidor MCP con herramientas
`translate_text` y `translate_file`, sin proveedor real, sin red, sin wrapper
Zed y sin modificar buffers. La decision tecnica vigente es implementar el
servidor MCP en Rust con `rmcp`, llamando a `translator-core` directamente.

Queda fuera de esta feature:

- proveedor real;
- red;
- servidor MCP por HTTP/red o publicacion en registro;
- wrapper Zed;
- edicion automatica de buffers;
- soporte de archivo completo para codigo fuente.

La tercera feature formal tiene validacion automatizada completa y smoke manual
interactivo en Zed aprobado. Sus criterios de diagnostico y entorno quedaron
re-especificados segun las limitaciones confirmadas del runtime de extension de
Zed; el detalle operativo vive en `specs/003-zed-wrapper/` y las decisiones
estables en D063/D064:

```text
specs/003-zed-wrapper/
```

Esta iteracion empaqueta el servidor MCP existente como extension local de
desarrollo de Zed en `zed-extension/` y cubre startup local, diagnosticos
redaccionados, preparacion reproducible y no-mutacion. No incluye proveedor
real, red, publicacion, UX avanzada ni edicion automatica de buffers.

## Flujo por feature

Para cada iteracion:

1. Elegir el siguiente bloque desde `docs/feature-map.md`.
2. Usar esa entrada como contexto para `speckit-specify`.
3. Ejecutar `speckit-clarify` solo si quedan decisiones ambiguas.
4. Usar `speckit-plan` para fijar tecnologias, estructura, contratos y limites
   concretos de esa feature.
5. Usar `speckit-tasks` para generar trabajo ejecutable.
6. Usar `speckit-analyze` antes de implementar cuando haya varios artefactos.
7. Usar `speckit-implement` para ejecutar `tasks.md` con TDD.

Durante este flujo, `docs/feature-map.md` conserva el backlog. La feature
formal vive en `specs/<feature>/`.

## Roadmap

### 1. Nucleo de traduccion

Completado en la primera feature formal de Spec Kit con TDD.

Entregables:

- workspace Rust;
- `translator-core`;
- `translator-cli`;
- contratos request/result/error;
- `MockProvider`;
- preservacion Markdown basica;
- lectura workspace-only para `.md`, `.markdown` y `.txt`;
- redaccion de errores/logs;
- pruebas buenas, malas y adversariales.

### 2. Servidor MCP

Completado en la segunda feature formal de Spec Kit con TDD. Expone el core
como herramientas MCP:

- `translate_text`;
- `translate_file`;
- validacion de parametros;
- errores MCP con `isError: true`;
- tests de contrato MCP.
- transporte stdio y crate Rust `crates/translator-mcp/`.

### 3. Wrapper Zed

Completado en la tercera feature formal de Spec Kit con TDD. Empaqueta el
servidor como extension local de desarrollo:

- `zed-extension/extension.toml`;
- crate Rust/WASM aislada con `zed_extension_api = "0.7.0"`;
- `context_server_command` para arrancar `translator-mcp` como comando directo;
- preparacion reproducible mediante `make zed-extension-prepare`;
- validacion con `make test-zed-extension`;
- diagnosticos redaccionados y revision de no-mutacion.

### 4. UX de lectura

Reducir friccion dentro de Zed:

- flujo Agent Panel;
- seleccion si Zed la expone de forma fiable;
- resultado legible sin modificar buffers.

### 5. Proveedor real y publicacion

Solo despues de tener la base segura:

- seleccionar proveedor local o remoto;
- mantener remoto default deny;
- confirmar cada envio fuera del equipo;
- auditar dependencias;
- preparar licencia y publicacion.
