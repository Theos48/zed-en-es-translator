# zed-en-es-translator

Proyecto para explorar y desarrollar una integracion de traduccion ingles -> espanol dentro de Zed.

## Objetivo inicial

Permitir que una persona traduzca texto en ingles a espanol sin salir del editor Zed, con un flujo que se sienta integrado y que pueda evolucionar por features pequenas guiadas por SDD y TDD.

## Estado

Primera feature formal implementada:

```text
specs/001-translation-core-contract/
```

La primera feature entrega un MVP tecnico offline: core Rust, `MockProvider`,
contrato CLI JSON por stdin/stdout, limites explicitos, lectura segura de
Markdown/texto y pruebas negativas de seguridad. El proveedor real, MCP/Zed y
el soporte completo de archivos de codigo quedan para ciclos posteriores.

Rust se ejecuta mediante la imagen Docker oficial fijada en `Makefile`; no se
instala `rustc` ni `cargo` globalmente para este proyecto por defecto.

Validacion principal:

```bash
make test
```

Resultado registrado: `make test` pasa dentro del contenedor Rust fijado por el
proyecto.

## Documentos

- [Plan de desarrollo](docs/PLAN.md)
- [Producto y roadmap funcional](docs/product.md)
- [Mapa detallado de features](docs/feature-map.md)
- [Matriz de decisiones](docs/decisions.md)
- [Diagramas](docs/diagrams.md)
- [ADR 0001: alcance tecnico inicial](docs/adr/0001-zed-extension-scope.md)
- [ADR 0002: arquitectura y tecnologia inicial](docs/adr/0002-architecture-and-technology.md)
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

El proyecto vive en `/home/theos/dev/zed-en-es-translator`. No se usa `/dev` porque es un filesystem especial del sistema para dispositivos.
