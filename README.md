# English to Spanish Translator for Zed

Extensión de Zed que traduce inglés → español de forma local para Markdown y
texto plano. Muestra la traducción como un preview de solo lectura: el buffer y
el archivo originales nunca se modifican.

> **Estado de publicación:** la implementación y el paquete local están
> validados, pero la extensión todavía no está disponible en la Extension
> Gallery. El tag y el asset público `v0.1.0` siguen pendientes.

## Qué ofrece

- Instalación plug-and-play desde la Gallery cuando concluya la publicación.
- Preparación automática del runtime y de los modelos en el primer uso.
- Traducción offline después de la adquisición pública inicial.
- Preservación de estructura Markdown, enlaces, código y contenido ambiguo.
- Preview mediante hover, sin `WorkspaceEdit`, inserciones ni reemplazos.
- Paquete y modelos verificados por tamaño, SHA-256, layout y permisos.
- Recuperación segura ante descarga interrumpida, corrupción o actualización
  fallida.
- Sin endpoints, credenciales, motores o rutas de binario configurables.

## Instalación y uso

Cuando la publicación en la Gallery esté disponible:

1. Abre la **Extension Gallery** de Zed.
2. Instala **English to Spanish Translator**.
3. Abre un archivo Markdown o de texto plano.
4. Ejecuta **Translate English to Spanish [offline]** sobre una selección o el
   documento permitido.
5. Coloca el cursor sobre el rango fuente para leer el preview traducido.

En la primera activación, la extensión descarga un archivo de release y tres
recursos públicos Mozilla `en → es`. Todo se valida y almacena dentro del
directorio de trabajo que Zed administra para la extensión. Los siguientes usos
no requieren red.

No existe un procedimiento de instalación manual para usuarios: no se necesita
terminal, Docker, checkout, servicio, cuenta, API key ni configuración de LSP.

## Soporte de la primera release

| Área | Soporte |
|---|---|
| Editor | Zed |
| Plataforma | Linux `x86_64` |
| Dirección | Inglés → español |
| Documentos | Markdown y texto plano |
| Resultado | Hover de solo lectura |
| Inferencia | Bergamot/Marian local |
| Configuración de proveedor | Ninguna |

Las plataformas no soportadas fallan antes de descargar o crear estado y
muestran el límite dentro de Zed.

Linux `x86_64` es el soporte de la primera release, no el límite final del
producto. El roadmap prioriza completar Linux `aarch64`, macOS Intel/Apple
Silicon y Windows x64/ARM64 sin cambiar la experiencia ni agregar
configuración. Consulta [F013 en el mapa de features](docs/feature-map.md#f013-paridad-con-las-plataformas-oficiales-de-zed).

## Cómo funciona

```text
Zed Gallery
  -> extensión Rust/WASM
  -> paquete local verificado y propiedad de Zed
  -> translator-lsp
  -> translator-core
  -> runtime Bergamot/Marian + modelos Mozilla en->es
  -> preview de solo lectura en Zed
```

La extensión es la única responsable de adquirir y activar el paquete. El LSP
resuelve el runtime y los modelos adyacentes a su propio ejecutable; no acepta
configuración del usuario. Core valida el documento, protege sus estructuras,
envía únicamente segmentos traducibles al runtime y reconstruye el resultado.

Consulta [los diagramas de arquitectura](docs/diagrams.md) y la
[guía de desarrollo](CONTRIBUTING.md) para el recorrido detallado.

## Privacidad y seguridad

- El contenido y la traducción se procesan localmente.
- Las descargas públicas nunca incluyen texto, traducciones, rutas del
  workspace, credenciales ni secretos.
- El runtime se inicia sin shell y con entorno, argumentos, I/O, hilos y timeout
  acotados.
- Los paths inseguros, symlinks fuera del workspace, archivos sensibles,
  binarios y contenido no UTF-8 se rechazan antes de traducir.
- Los diagnósticos no incluyen contenido, tokens, headers, output crudo del
  proceso ni rutas sensibles.
- Un paquete inválido nunca se vuelve activo; una actualización fallida conserva
  el último paquete verificado.
- Deshabilitar evita nuevos arranques. Desinstalar desde Zed elimina el estado
  propiedad de la extensión, sin instalación global paralela.

Límites vigentes:

| Límite | Valor |
|---|---:|
| Entrada | 20 KiB |
| Segmento traducible | 4 KiB |
| Segmentos por solicitud | 256 |
| Salida reconstruida | 40 KiB |
| Tiempo de traducción | 15 segundos |
| Paquete activo instalado | 128 MiB |
| Memoria máxima de inferencia | 1 GiB |
| Hilos de inferencia | 4 |

## Estado del proyecto

Ya están completas y validadas:

- adquisición automática, integridad y promoción atómica;
- traducción Bergamot real con red deshabilitada;
- previews, no-mutación, Markdown, límites y redacción;
- recuperación, concurrencia y último paquete válido;
- supply chain, licencias, presupuesto y remoción;
- convergencia del repositorio a una sola arquitectura.

Quedan únicamente gates externos:

1. publicar el tag y asset exactos;
2. ejecutar la aceptación interactiva contra ese asset;
3. enviar la extensión al registro oficial de Zed;
4. después del merge upstream, completar 3/3 instalaciones limpias desde la
   Gallery.

Una dev extension o un binario del checkout no sustituye esos gates.

## Desarrollo rápido

El proyecto usa GNU Make y Docker para no instalar toolchains Rust/C++ en el
host. El checkout debe vivir en almacenamiento persistente, nunca en `/tmp` u
otro `tmpfs`.

Requisitos básicos: Git, GNU Make, Docker, Bash y utilidades GNU. Los gates del
paquete real también usan `jq`, `curl` y `zstd`.

```bash
git clone https://github.com/Theos48/zed-en-es-translator.git
cd zed-en-es-translator
make workspace-storage-check
make pull-rust-base
make rust-image
make test
```

Validación habitual antes de revisión:

```bash
make fmt
make clippy
make deny
make test
make test-repository-boundary
git diff --check
```

Usa `make help` para consultar todos los targets. El build nativo completo puede
usar decenas de GiB; `make clean-preview` y `make clean` eliminan salidas
reproducibles sin borrar los caches fijados.

## Documentación

### Para desarrollar y desplegar

- [Guía de desarrollo y contribución](CONTRIBUTING.md)
- [Dev extension, release, forks y publicación en Zed](docs/deployment.md)
- [Arquitectura y flujos](docs/diagrams.md)
- [Mantenimiento del paquete marketplace](ops/marketplace/README.md)

### Producto, decisiones y roadmap

- [Plan y secuencia de publicación](docs/PLAN.md)
- [Mapa de features futuras](docs/feature-map.md)
- [Matriz de decisiones](docs/decisions.md)
- [ADR 0006: paquete automático para la Gallery](docs/adr/0006-zed-marketplace-package.md)
- [ADR 0007: convergencia del repositorio](docs/adr/0007-repository-convergence.md)

### Contratos y evidencia activa

- [Feature 009: instalación y publicación](specs/009-zed-marketplace-install/spec.md)
- [Feature 009: evidencia validada](specs/009-zed-marketplace-install/validation.md)
- [Feature 010: convergencia y limpieza](specs/010-repository-convergence/spec.md)
- [Feature 010: evidencia validada](specs/010-repository-convergence/validation.md)

Las guías son la entrada operativa. Las specs conservan requisitos, contratos y
evidencia; los ADRs registran decisiones estables y Git conserva la historia
retirada.

## Licencias

El código original del proyecto se distribuye bajo MIT; el texto de licencia de
la extensión está en [`zed-extension/LICENSE`](zed-extension/LICENSE). El
runtime Bergamot/Marian y los modelos Mozilla se distribuyen bajo sus términos
MPL-2.0 y avisos correspondientes, registrados en
[`ops/marketplace/licenses/`](ops/marketplace/licenses/).
