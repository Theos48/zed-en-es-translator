# Investigacion: estructura de repositorio compatible con Zed y Spec Kit

> Nota de vigencia: esta investigacion conserva alternativas de estructura
> previas a F005. La estructura vigente para el servidor MCP es
> `crates/translator-mcp/`, definida en ADR 0003 y `specs/002-mcp-server/`. El
> wrapper Zed ya fue implementado en `zed-extension/` durante F006. La siguiente
> direccion de producto es F010: accion directa de extension Zed sin Agent; MCP
> y Agent Panel quedan como puente/infraestructura, no como superficie final.

## Objetivo

Definir una estructura de repositorio que:

1. Sea compatible con desarrollo y publicacion de extension Zed.
2. No interfiera con la estructura que Spec Kit genera para features formales.
3. Permita separar core Rust, servidor MCP Rust, CLI y extension Zed.
4. Mantenga documentacion de planeacion separada de specs formales.

## Evidencia revisada

### Zed

- Una extension Zed es un repositorio Git con `extension.toml`.
- Para desarrollo local, Zed instala como dev extension el directorio que contiene la extension.
- El ejemplo de estructura Zed coloca `extension.toml`, `Cargo.toml` y `src/lib.rs` en el directorio de la extension.
- Las partes procedurales de extensiones Zed se escriben en Rust y se compilan a WebAssembly.
- Las extensiones MCP registran servidores en `extension.toml` y el wrapper Rust implementa `context_server_command`.
- Zed permite publicar una extension ubicada en un subdirectorio del repositorio mediante el campo `path` al registrar la extension en `zed-industries/extensions`.
- Para MCP, Zed indica que el servidor puede estar publicado como binario o via NPM, y que el wrapper debe devolver comando, argumentos y entorno para arrancarlo.

### Spec Kit

- Spec Kit inicializa tooling y comandos del proyecto sin exigir una estructura de codigo unica.
- Su flujo crea specs formales bajo `specs/[###-feature-name]/`.
- El plan generado por Spec Kit documenta la estructura real de codigo dentro de `plan.md`.
- Sus plantillas incluyen ejemplos como `src/`, `tests/`, `backend/` y `frontend/`, pero permiten ajustar la estructura segun el tipo de proyecto.
- En proyectos existentes, Spec Kit recomienda separar actualizaciones del tooling de cambios a artefactos de features.

## Implicaciones

1. No conviene poner la extension Zed en la raiz si el repo tambien contendra
   core Rust, CLI, MCP Rust, docs de planeacion y `specs/` de Spec Kit.
2. El directorio que Zed instale como dev extension debe poder apuntarse directamente a un subdirectorio con `extension.toml`.
3. La raiz reserva `specs/` para features formales de Spec Kit.
4. La documentacion estrategica vive en `docs/`; la implementacion activa vive
   en `specs/<feature>/`.
5. Conviene separar el core reusable de sus adaptadores: CLI, MCP y extension
   Zed.

## Estructura candidata

```text
zed-en-es-translator/
├── docs/
│   ├── adr/
│   ├── research/
│   ├── decisions.md
│   ├── diagrams.md
│   └── PLAN.md
├── crates/
│   ├── translator-core/
│   ├── translator-cli/
│   └── translator-mcp/
├── zed-extension/
│   ├── extension.toml
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── tests/
│   ├── fixtures/
│   └── integration/
├── specs/
│   └── [features formales de Spec Kit]
├── Makefile
└── Cargo.toml
```

## Decision

Usar `zed-extension/` como directorio de extension Zed, no la raiz del repositorio.

Razon:

- Es compatible con `zed: install dev extension` apuntando a `zed-extension/`.
- Es compatible con publicacion Zed usando `path = "zed-extension"`.
- Deja la raiz libre para workspace Rust, CLI, MCP Rust, docs y `specs/`.
- Evita que la estructura generada por Spec Kit compita con los archivos obligatorios de Zed.

Estado: aceptada en `D029`.
