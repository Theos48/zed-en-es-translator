# zed-en-es-translator

Proyecto para explorar y desarrollar una integracion de traduccion ingles -> espanol dentro de Zed.

## Objetivo inicial

Permitir que una persona traduzca texto en ingles a espanol sin salir del editor Zed, con un flujo que se sienta integrado y que pueda evolucionar por features pequenas guiadas por SDD y TDD.

## Objetivo de producto

Construir una extension nativa de traduccion para Zed, disenada como producto
propio para trabajo tecnico: seleccion de texto, accion desde menu
contextual/comando/boton, preview legible, preservacion de Markdown/codigo,
control explicito para copiar/insertar/aplicar, proveedores locales o
gratuitos/no pago, remoto siempre confirmado, privacidad por defecto y sin
depender de Agent Panel como experiencia principal.

## Estado

Estado de las features formales:

```text
specs/001-translation-core-contract/  completada formal
specs/002-mcp-server/                 completada formal
specs/003-zed-wrapper/                completada formal
specs/004-zed-ux-flow/                completada formal
specs/005-real-provider-config/        completada formal
```

La primera feature entrega un MVP tecnico offline: core Rust, `MockProvider`,
contrato CLI JSON por stdin/stdout, limites explicitos, lectura segura de
Markdown/texto y pruebas negativas de seguridad.

La segunda feature agrega un servidor MCP Rust en `crates/translator-mcp/` con
transporte stdio y dos tools: `translate_text` y `translate_file`. Mantiene el
modo offline/mock, no agrega proveedor real, no abre red, no modifica buffers y
delega lectura/seguridad de archivos al core existente.

La tercera feature agrega una extension local de desarrollo para Zed en
`zed-extension/`. La extension declara el context server `translator-en-es`,
devuelve un comando controlado para arrancar el binario release
`translator-mcp`, no agrega entorno arbitrario propio, rechaza configuracion de
provider/remoto/args/env arbitrarios y emite diagnosticos redaccionados. La
validacion de filesystem y el aislamiento total del entorno del proceso lanzado
quedan acotados por limitaciones confirmadas del runtime de Zed; ver
`specs/003-zed-wrapper/` y `docs/decisions.md` D063/D064. No agrega provider
real, red, publicacion ni edicion automatica de buffers.

La cuarta feature documenta y valida el flujo de lectura dentro de Zed
sobre la extension local ya fusionada. Cubre el uso del Agent Panel con
`translate_text` y `translate_file`, los limites entre el modelo del Agent y el
servidor MCP local, la evidencia manual redaccionada, la decision explicita de
soporte de seleccion y la no-mutacion de archivos o buffers. La guia operativa
vive en `docs/zed-ux-flow.md`. Este flujo Agent Panel es un puente de
validacion, no la UX final del producto: la meta final es una accion propia de
la extension de Zed para traducir desde menu contextual, comando o boton sin
configurar Agent.

La quinta feature implementa el primer proveedor real configurable en
`specs/005-real-provider-config/`. Mantiene `MockProvider` como default,
selecciona un proveedor local/self-hosted compatible con LibreTranslate como
primer camino real, modela remoto como default-deny con confirmacion por
solicitud, conserva no-mutacion, limites, redaccion y host limpio, y expone la
configuracion controlada a CLI, MCP y la extension Zed.

Rust se ejecuta mediante la imagen Docker oficial fijada en `Makefile`; no se
instala `rustc` ni `cargo` globalmente para este proyecto por defecto.

Validacion principal:

```bash
make zed-extension-prepare
make test-zed-extension
make test-zed-ux-flow
make test-core
make test-mcp
make test-real-provider-config
make test
make fmt
make clippy
```

Resultado registrado para `specs/001-translation-core-contract/` y
`specs/002-mcp-server/`: `make test`, `make test-mcp`, `make fmt` y
`make clippy` pasan dentro del contenedor Rust fijado por el proyecto.

Resultado registrado para `specs/003-zed-wrapper/`: `make
zed-extension-prepare`, `make test-zed-extension`, `make test`, `make fmt` y
`make clippy` pasan. El smoke manual interactivo en Zed pasa con la modal de
configuracion de la extension. Los limites de diagnostico rapido y aislamiento
de entorno quedaron documentados en el spec y en D063/D064.

Resultado registrado para `specs/004-zed-ux-flow/`: `make test-zed-ux-flow`
pasa y la validacion manual interactiva en Zed quedo registrada con evidencia
sintetica/redaccionada.

Resultado registrado para `specs/005-real-provider-config/`: `make test-core`,
`make test-mcp`, `make test-zed-extension`, `make test-real-provider-config`,
`make test`, `make fmt` y `make clippy` pasan dentro del contenedor Rust del
proyecto. La evidencia automatizada usa stubs locales de loopback; la plantilla
para smoke manual contra un proveedor local externo vive en
`specs/005-real-provider-config/manual-validation.md`.

## Documentos

- [Plan de desarrollo](docs/PLAN.md)
- [Producto y roadmap funcional](docs/product.md)
- [Mapa detallado de features](docs/feature-map.md)
- [Matriz de decisiones](docs/decisions.md)
- [Guia de flujo UX en Zed](docs/zed-ux-flow.md)
- [Diagramas](docs/diagrams.md)
- [ADR 0001: alcance tecnico inicial](docs/adr/0001-zed-extension-scope.md)
- [ADR 0002: arquitectura y tecnologia inicial](docs/adr/0002-architecture-and-technology.md)
- [ADR 0003: servidor MCP Rust con rmcp](docs/adr/0003-mcp-server-rust-rmcp.md)
- [Investigacion: estructura Zed y Spec Kit](docs/research/zed-spec-kit-repo-structure.md)
- [Investigacion: contrato de traduccion y Provider](docs/research/provider-contract.md)
- [Investigacion: archivos y comentarios](docs/research/supported-files-and-comments.md)
- [Investigacion: seguridad y runtime](docs/research/security-runtime-controls.md)

## Como usar la documentacion

`docs/` es la planeacion estrategica. Sirve para decidir y preparar futuras
features de Spec Kit.

`specs/<feature>/` es la fuente operativa de la feature activa. Ahi viven el
`spec.md`, `plan.md`, `tasks.md`, contratos, quickstart y checklists que se
implementan.

Para una nueva iteracion:

1. Elegir una entrada de `docs/feature-map.md`.
2. Crear/refinar la feature con Spec Kit.
3. Dejar el detalle operativo en `specs/<feature>/`.
4. Actualizar `docs/decisions.md` o ADRs solo si cambia una decision estable.

## Nota sobre ubicacion

El proyecto vive en la raiz del repositorio. Usa rutas relativas al checkout en
lugar de rutas absolutas del host.
