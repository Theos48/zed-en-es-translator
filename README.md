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
specs/006-direct-zed-translation/      completada formal
specs/007-operational-providers/       completada formal
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
configuracion controlada a CLI, MCP y la extension Zed. Esta entrega implementa
el adaptador y sus controles; no instala, despliega ni deja configurado un
servicio real, y su evidencia automatizada usa servicios loopback simulados.

La sexta feature implementa F010 como flujo directo sin Agent. La extension
registra el language server `en-es-translator` para Markdown y Plain Text; su
code action usa la seleccion o el documento permitido, ejecuta
`translator-lsp`, muestra un preview de solo lectura por hover y conserva la
confirmacion remota por solicitud. No devuelve edits ni modifica archivos o
buffers. La API de Zed 0.7.0 no permite clipboard o panel propio, por lo que
copy/insert/apply quedan fuera. La configuracion de proveedor directa usa solo
la allowlist `lsp.en-es-translator.binary.env` validada en Zed real; estas
decisiones viven en D073-D075 y ADR 0004.

La septima feature implementa el camino automatico y operativo de F011 en
`specs/007-operational-providers/`. Selecciona LibreTranslate 1.9.6 fijado
por digest como camino soportado local/offline sin cuenta ni API key.
`MockProvider` sigue siendo default. El adaptador Azure AI Translator Text v3
permanece como opcion avanzada: usa host HTTPS fijo, key por referencia y
confirmacion nueva por solicitud, pero no es requisito de uso ni aceptacion.
Estan implementados la configuracion exacta, los adaptadores, las pruebas
controladas y el ciclo local candidate/current/previous. La validacion real
local por CLI y Zed directo, sin egress, con fallo de update aislado y rollback
paso; la limpieza project-scoped y la evidencia final tambien pasan. MCP/Agent
Panel conserva solo cobertura de compatibilidad y no
es una superficie de aceptacion F011. El modelo Argos `en-es` no se
redistribuira mientras upstream no declare su licencia; ese gate legal sigue
siendo independiente para F009/empaquetado y publicacion.

El proveedor local se administra solo mediante la interfaz versionada del
proyecto:

```bash
make provider-local-prepare
make provider-local-start
make provider-local-status
make provider-local-verify
make provider-local-stop
make provider-local-update
make provider-local-rollback
make provider-local-clean CONFIRM=remove-provider-data
```

Preparar/actualizar requiere red y al menos 4 GiB libres; el contenedor queda
limitado a 4 CPU y 4 GiB RAM. Start, status, verify, stop y rollback usan el
artefacto ya preparado sin descarga. Stop y `make clean` conservan sus datos;
la eliminacion completa exige el token exacto y se limita al proyecto Compose.
LibreTranslate permanece solo en la red interna y sin puerto publicado; un
relay minimo del mismo proyecto expone `127.0.0.1:5000`, reenvia unicamente al
destino interno fijo y no registra contenido. No se instala Python en Fedora:
el relay se ejecuta dentro de la imagen fijada del proveedor.
El camino soportado no requiere cuenta, suscripcion ni key. Si alguien opta de
forma independiente por el adaptador remoto avanzado, sus credenciales siguen
fuera de settings, archivos versionados y evidencia. La guia completa de
privacidad, validacion, rollback, licencia y remocion esta en
`specs/007-operational-providers/quickstart.md`.

Rust se ejecuta mediante la imagen Docker oficial fijada en `Makefile`; no se
instala `rustc` ni `cargo` globalmente para este proyecto por defecto.

Los worktrees y sus artefactos de compilacion deben vivir en almacenamiento
persistente, nunca bajo `/tmp`, `/dev/shm` u otro `tmpfs`/`ramfs`. Para
revisiones temporales se usa
`~/dev/.worktrees/zed-en-es-translator/<nombre>`. `make
workspace-storage-check` valida el checkout actual antes de cualquier build
Rust y `make worktree-audit` revisa todos los worktrees registrados. La guarda
se prueba sin compilar con `make test-worktree-storage`.

La calidad obligatoria se valida localmente y en cada pull request con los
mismos targets del `Makefile`: formato, Clippy, pruebas y `cargo-deny`. La
auditoria cubre vulnerabilidades publicadas, licencias, dependencias prohibidas
y fuentes no autorizadas. Dependabot revisa semanalmente las dependencias Cargo
de ambos workspaces y las acciones de GitHub.

Validacion principal:

```bash
make workspace-storage-check
make worktree-audit
make test-worktree-storage
make zed-extension-prepare
make zed-direct-prepare
make test-direct-zed-translation
make test-zed-extension
make test-zed-ux-flow
make test-core
make test-mcp
make test-operational-providers
make test-real-provider-config
make test
make fmt
make clippy
make deny
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

Resultado automatizado registrado para
`specs/006-direct-zed-translation/`: `make test-direct-zed-translation`, `make
test-zed-extension`, `make test-real-provider-config`, `make fmt` y `make
clippy` pasan. Los tres escenarios interactivos en Zed tambien pasan con
fuentes sin cambios, Agent Panel ausente y denegacion remota por secreto antes
del proveedor; la evidencia redactada vive en
`specs/006-direct-zed-translation/manual-validation.md`.

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
- [ADR 0004: flujo directo Zed mediante LSP](docs/adr/0004-direct-zed-lsp-workflow.md)
- [ADR 0005: pareja operativa de proveedores reales](docs/adr/0005-operational-provider-pair.md)
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
