# Guía de desarrollo

Esta guía explica cómo preparar el proyecto, entender sus componentes, hacer
cambios y validarlos. La publicación y el despliegue se documentan por separado
en [`docs/deployment.md`](docs/deployment.md).

## Alcance del producto

El único flujo soportado es:

```text
Zed Gallery
  -> extensión Rust/WASM
  -> paquete verificado propiedad de Zed
  -> translator-lsp
  -> translator-core
  -> runtime Bergamot/Marian + modelos Mozilla en->es
```

La extensión prepara el paquete automáticamente en el primer uso. Después de
esa preparación, la traducción es local y offline. El resultado se presenta en
un hover de solo lectura: nunca se modifica el buffer ni el archivo original.

La primera release admite únicamente Linux `x86_64`, Markdown, texto plano e
inglés → español.

## Requisitos de desarrollo

| Requisito | Uso |
|---|---|
| Git | Checkout, locks de fuentes y flujo de cambios. |
| GNU Make | Interfaz única de compilación, pruebas y empaquetado. |
| Docker | Toolchain Rust/C++ fijado; no se usa `cargo` o `rustc` del host. |
| Bash y utilidades GNU | Scripts de integración, hashes, archivos y permisos. |
| `jq`, `curl`, `zstd` | Preparación y prueba del paquete real con modelos. |
| Linux `x86_64` | Evidencia nativa y perfil soportado de la primera release. |

El checkout debe estar en almacenamiento persistente. No uses `/tmp`,
`/dev/shm` ni otro `tmpfs`: los builds nativos pueden consumir decenas de GiB.
Para ejecutar toda la validación marketplace conviene reservar aproximadamente
40 GiB libres. El paquete instalado sigue teniendo un límite independiente de
128 MiB.

La red solo es necesaria para descargar la imagen fijada, las fuentes nativas
y los tres recursos públicos del modelo. La compilación nativa y la inferencia
real se prueban con red deshabilitada.

## Preparación inicial

```bash
git clone https://github.com/Theos48/zed-en-es-translator.git
cd zed-en-es-translator
make workspace-storage-check
make pull-rust-base
make rust-image
make test
```

`make rust-image` crea el toolchain fijado en
`docker/rust-toolchain.Dockerfile`. Los caches locales viven en `.cache/` y las
salidas en `target/`; ambos están excluidos de Git.

Consulta siempre la interfaz vigente antes de trabajar:

```bash
make help
```

## Mapa del repositorio

| Ruta | Responsabilidad |
|---|---|
| `zed-extension/` | Extensión WASM, detección de plataforma, adquisición, estado y lanzamiento del LSP. |
| `crates/translator-lsp/` | Sincronización de documentos, code action, comando y hover de preview. |
| `crates/translator-core/` | Validación, límites, Markdown, paths seguros, redacción y proceso embebido. |
| `native/translator-embedded-runtime/` | Adaptador C++ privado que ejecuta Bergamot/Marian. |
| `ops/marketplace/` | Locks exactos de paquete, modelos, fuentes, Zed y licencias. |
| `scripts/marketplace/` | Fetch nativo, build determinista, validación y paquete real. |
| `tests/integration/` | Frontera del repositorio y gates marketplace. |
| `.github/workflows/` | CI normal y publicación del paquete al crear un tag. |
| `docs/` | Arquitectura, decisiones, roadmap y despliegue. |
| `specs/009-*` | Requisitos y evidencia de instalación/publicación. |
| `specs/010-*` | Contrato y evidencia de convergencia del repositorio. |

## Cómo funciona una traducción

1. Zed carga la extensión desde `zed-extension/` y solicita el language server.
2. La extensión comprueba la plataforma y analiza el `package.lock.json`
   compilado dentro del WASM.
3. Si no existe un paquete activo válido, descarga a `staging/` el archivo de
   release y los tres recursos Mozilla de URLs fijas.
4. Verifica HTTPS, tamaño, SHA-256, layout, permisos, licencias y presupuesto;
   después promociona el paquete mediante un rename atómico.
5. Lanza únicamente el `translator-lsp` verificado, sin argumentos ni variables
   de proveedor.
6. El LSP conserva snapshots versionados de Markdown o texto plano y ofrece la
   acción `Translate English to Spanish [offline]`.
7. `translator-core` valida tamaño, tipo, UTF-8 y path; protege Markdown, código
   y enlaces, y entrega al runtime solo los segmentos traducibles.
8. El runtime procesa esos segmentos con Bergamot/Marian y los modelos
   adyacentes. El proceso tiene stdin/stdout/stderr, entorno, hilos y timeout
   acotados.
9. Core reconstruye la estructura y el LSP guarda un preview asociado a la
   versión del documento. Zed lo muestra mediante hover.
10. Cualquier cambio o cierre del documento invalida el preview.

El layout exacto, la adquisición y la secuencia visual están en:

- [`specs/009-zed-marketplace-install/contracts/translation-package.md`](specs/009-zed-marketplace-install/contracts/translation-package.md)
- [`specs/009-zed-marketplace-install/contracts/acquisition.md`](specs/009-zed-marketplace-install/contracts/acquisition.md)
- [`docs/diagrams.md`](docs/diagrams.md)

## Ciclo cotidiano de cambios

Antes de editar:

```bash
make workspace-storage-check
make test-repository-boundary
```

Durante el cambio:

```bash
make format
make test
```

Antes de solicitar revisión:

```bash
make fmt
make clippy
make deny
make test
make test-repository-boundary
git diff --check
```

Los cambios de comportamiento empiezan con una prueba que falle o con un
contrato negativo explícito. No edites `Cargo.lock` manualmente. Para cambios
en dependencias del workspace raíz, actualiza el manifest y ejecuta:

```bash
make workspace-lock
```

Las dependencias de `zed-extension/` se mantienen con su lock independiente y
los PRs de Dependabot; cualquier actualización manual debe hacerse con el mismo
toolchain fijado y terminar con `make test`, `make clippy` y `make deny`.

## Qué modificar según el objetivo

### Reglas de texto, Markdown, seguridad o límites

Modifica `crates/translator-core/`. Añade cobertura adyacente para:

- segmentación y reconstrucción;
- regiones protegidas y contenido ambiguo;
- paths, symlinks, archivos sensibles, binarios y UTF-8;
- límites de entrada, segmento, salida y proceso;
- redacción de errores y diagnósticos.

Los límites públicos viven en `crates/translator-core/src/limits.rs` y también
deben coincidir con los presupuestos del package lock y sus tests.

### Acción, estado documental o preview de Zed

Modifica `crates/translator-lsp/`. Conserva:

- full document sync versionado;
- una sola acción/comando de traducción;
- hover de solo lectura, sin `WorkspaceEdit`;
- invalidación por cambio, cierre o versión obsoleta;
- mensajes sin contenido del documento.

### Descarga, recuperación o plataformas

Modifica `zed-extension/src/acquisition.rs`, `package.rs` y sus tests. Todo
input descargable debe seguir fijado en `ops/marketplace/package.lock.json`.
Una plataforma no soportada debe fallar antes de red o escritura.

### Runtime nativo o modelos

Modifica el adaptador en `native/translator-embedded-runtime/` o los locks de
`ops/marketplace/`. Después ejecuta al menos:

```bash
make test-marketplace-foundation
make test-marketplace-native-supply-chain
make test-marketplace-package
make test-marketplace-offline
```

Un cambio en binarios, modelos, versión o layout obliga a regenerar tamaños,
SHA-256, licencias y evidencia del candidato exacto.

### Versión, nombre, repositorio o fork

La identidad aparece de forma intencional en manifests, locks, scripts y tests.
Antes de publicar una variante, localiza todos los puntos acoplados:

```bash
rg -n 'en-es-translator|0\.1\.0|Theos48|linux-x86_64' \
  --glob '!.cache/**' --glob '!Cargo.lock' --glob '!zed-extension/Cargo.lock'
```

No reutilices el ID público para un producto distinto. Cambiar repositorio
también requiere actualizar la allowlist HTTPS estricta de
`zed-extension/src/package.rs`, el package lock, manifests, tests, licencias y
workflow de release.

## Gates por tipo de cambio

| Cambio | Gates mínimos adicionales |
|---|---|
| Core o LSP | `make test-marketplace-foundation` |
| Extensión/adquisición | `make test-marketplace-contract test-marketplace-acquisition` |
| Runtime/fuentes C++ | `make test-marketplace-native-supply-chain` |
| Lock o paquete | `make test-marketplace-package test-marketplace-release-contents` |
| Modelos, privacidad o recursos | `make test-marketplace-offline` |
| Release/tag | Todos los anteriores y después `make marketplace-release-check` |

Los targets marketplace reutilizan Docker, pero algunos descargan fuentes o
modelos y pueden tardar varios minutos.

## Restricciones que requieren una decisión explícita

Estos cambios no son una refactorización ordinaria:

- escribir, insertar o reemplazar contenido en el editor;
- agregar proveedores remotos o configurables;
- aceptar endpoints, credenciales o binarios arbitrarios;
- añadir otra aplicación, CLI o lifecycle de modelos;
- soportar otra plataforma o par de idiomas sin paquete exacto.

Primero actualiza la constitución/spec correspondiente y registra la decisión
en `docs/decisions.md` o en un ADR. Después sigue el flujo Spec Kit descrito en
`docs/PLAN.md`.

## Limpieza y espacio en disco

```bash
make clean-preview
make clean
```

La limpieza normal elimina targets, WASM y paquetes generados, pero conserva
caches reproducibles. Para revisar el nivel profundo:

```bash
make clean-deep-preview
```

Solo si realmente quieres volver a descargar dependencias y fuentes:

```bash
make clean-deep CONFIRM=remove-reproducible-caches
```

Nunca borres worktrees con `rm`. Usa `git worktree remove` y después
`git worktree prune`.

## Diagnóstico rápido

| Problema | Comprobación |
|---|---|
| Docker no responde | Verifica el daemon y acceso al socket; no instales Rust en el host como atajo. |
| Build rechazado por almacenamiento | Mueve el checkout a disco persistente y ejecuta `make worktree-audit`. |
| Build nativo tarda o ocupa mucho | Es normal en un build limpio; revisa con `make clean-preview` y conserva `.cache/embedded-source`. |
| Falla un hash | No actualices el lock para silenciarlo; identifica primero qué artefacto cambió. |
| Offline falla antes de iniciar | Ejecuta primero el gate de paquete y confirma que `jq`, `curl` y `zstd` existen. |
| Release check dice que falta el tag | Es el resultado esperado hasta publicar el candidato exacto. |
| Zed no carga una dev extension | Revisa `zed: open log`; consulta también `docs/deployment.md`. |

## Pull requests

Describe el comportamiento afectado, los contratos cambiados y los gates
ejecutados. Usa `.github/pull_request_template.md` como checklist. No incluyas
`target/`, `.cache/`, modelos, paquetes generados, logs, `.env` ni evidencia con
contenido privado.
