# ADR 0006: proveedor embebido Bergamot con runner aislado

## Estado

Aceptado como arquitectura de implementacion condicionada para F012. La
seleccion no se considera promovida, soportada ni publicable hasta que pasen
los gates reales de build reproducible, CPU portable, licencia/procedencia por
artefacto, recursos, latencia, red cero, ciclo de vida, CLI y Zed definidos en
`specs/008-embedded-local-provider/`.

## Contexto

F011 cerro un camino local real con LibreTranslate, pero la experiencia normal
todavia requiere un servicio Docker administrado por el proyecto. F012 busca
traduccion local `en -> es` sin cuenta, key, endpoint remoto, contenedor de
proveedor ni daemon visible, conservando Mock como default y todos los limites
y fronteras de seguridad existentes.

La decision debe cubrir mas que la calidad del modelo: fuente mantenida,
licencia y procedencia de cada artefacto, integridad antes y despues de
descompresion, CPU y memoria, timeout terminable, consentimiento, almacenamiento,
updates atomicos, rollback offline y limites de la API de extensiones de Zed.

## Decision

### Candidato y entrega

Tomar Mozilla Translations/Bergamot y los recursos oficiales Firefox
Translations `en -> es` `base-memory` como el unico candidato que pasa a
implementacion. El lock inicial parte del snapshot exacto de la fuente
mantenida `mozilla/translations`
`f31423c7c2c6ed8ae57d71a3d19a9db6f156060e`, fija todas las dependencias
nativas recursivas y fija modelo, vocabulario y shortlist por record, URL,
version, tamano y SHA-256 comprimido/descomprimido.

El runner se construye con el entorno reproducible del proyecto y los recursos
del modelo se descargan desde fuentes oficiales exactas solo despues de
consentimiento. F012 no rehostea ni empaqueta esos recursos y no afirma
derechos de publicacion; F009 debe revisar el artefacto de entrega real.
La activacion local exige aprobacion humana registrada del mantenedor, ligada
al digest exacto y al scope de adquisicion local. Bundle, redistribucion o
publicacion exigen otra decision humana explicita dentro de F009; los reportes
automaticos apoyan la revision pero no sustituyen ninguna aprobacion.

### Frontera de proceso

Ejecutar inferencia mediante un helper C++17 minimo
`translator-embedded-runtime`, propiedad del proyecto, como proceso one-shot
detras del trait Rust `Provider`:

- un batch JSON versionado por stdin y una respuesta ordenada por stdout;
- binario, argumentos, cwd y manifiesto exactos, sin shell;
- entorno vaciado y pipes concurrentes acotados;
- timeout total existente de 15 segundos, con kill y reap;
- sin retry, socket, servicio, updater, downloader ni logging de contenido.

Se rechaza FFI C++ dentro del proceso como primera opcion porque una inferencia
colgada no puede cancelarse de forma segura. Un hijo persistente solo puede
considerarse si la medicion one-shot falla y una revision nueva cierra
lifecyle, memoria, concurrencia, crash y shutdown.

### Configuracion y Zed

Agregar solo `TRANSLATOR_PROVIDER=embedded_local`. Para ese modo, URL,
referencia de key y permiso remoto deben estar ausentes. No se agrega variable
para ruta de ejecutable/modelo, URL, argumentos o entorno arbitrario. CLI y LSP
resuelven internamente un profile ID y root XDG fijos.

La extension WASM de Zed sigue iniciando `translator-lsp`; no ejecuta el modelo
ni descarga silenciosamente desde `language_server_command`. La preparacion
con consentimiento es un flujo explicito anterior. La accion directa se marca
`[offline]` y mantiene preview de solo lectura.

### Almacenamiento y lifecycle

Usar un store XDG user-scoped, content-addressed y product-owned con objetos y
sets inmutables, staging y referencias logicas candidate/current/previous en
un state atomico. No se aceptan roots elegidos por el workspace, symlinks que
escapen, permisos/owner inseguros ni filesystem volatil.

La preparacion se liga al SHA-256 del manifiesto mostrado:

```text
make provider-embedded-disclose
make provider-embedded-prepare CONSENT=<manifest-sha256>
```

El manager deriva ese SHA-256 usando el dominio
`translator-provider-manifest-v1\0` y un payload JSON tipado de orden fijo que
cubre identidad, runner, artefactos ordenados, conclusiones, presupuestos y
estado de publicacion. Solo omite el propio digest y los registros de
aprobacion para evitar autorreferencia. Una URL, hash, licencia, permiso,
presupuesto o estado modificado invalida el digest y exige consentimiento y
aprobaciones nuevos.

Update requiere consentimiento nuevo. Status, verify y rollback son offline.
La limpieza completa requiere token exacto, lease exclusivo y enumeracion
probada de propiedad; `make clean` no toca los artefactos.

### Gate de promocion

Antes de habilitar el path deben medirse 20 fixtures publicos y pasar, entre
otros, estos limites: transferencia <=64 MiB, set activo <=128 MiB, lifecycle
<=384 MiB, RSS <=1 GiB, <=4 threads, cold readiness <=10 s, p95 warm mixto
<=5 s y toda solicitud <15 s. CLI y Zed deben traducir realmente con red
deshabilitada, cero contactos externos y cero mutacion.

## Razon

- Bergamot tiene uso on-device real, soporte directo `en -> es`, artefactos
  oficiales pequeños e identidad fuerte publicada por Mozilla.
- Un proceso one-shot mantiene C++/excepciones fuera del core Rust y hace
  exigible el timeout constitucional.
- Un root XDG fijo sirve a CLI y Zed sin permitir que un checkout controle un
  ejecutable nativo ni duplicar el modelo por workspace.
- Consentimiento por digest y sets inmutables convierten descarga/update en
  decisiones revisadas y recuperables, no en seguimiento automatico de
  `latest`.
- Separar manager con red del runner sin red reduce la superficie normal.

## Alternativas

- CTranslate2 + OPUS-MT base: fallback tecnico por licencias permisivas, pero
  tiene peor footprint, conversion derivada y ninguna frontera Rust oficial.
- Candle Marian: diferido por depender hoy de artefactos `en-es` derivados o
  de terceros para safetensors/tokenizer.
- WASM Firefox: diferido porque es Emscripten/JS, no WASI standalone.
- Argos: rechazado mientras su modelo `en-es` no declare licencia.
- FFI directo o daemon persistente: rechazados como default por cancelacion,
  ABI/unsafe o mayor lifecycle.
- Bundling/rehosting: fuera de F012 hasta revision de publicacion F009.

## Consecuencias

- El proyecto incorporara un pequeno boundary nativo C++ y debera auditar
  fuentes recursivas, ELF, CPU, SBOM/licencias y obligaciones MPL fuera de
  `cargo-deny`.
- Se agregara un manager Rust de lifecycle y un store user-scoped no
  privilegiado; ninguna dependencia se instala globalmente.
- D075 conserva sus cuatro keys, pero admite un valor de provider adicional.
- La preparacion inicial necesita red y consentimiento; uso normal y rollback
  no.
- Mock y LibreTranslate permanecen hasta que la evidencia promueva el path.
- Publicacion sigue bloqueada y se decide separadamente en F009.

## Criterio de revision

Revisar esta decision si:

- Bergamot no cumple timeout, RAM, CPU, calidad, portabilidad o build
  reproducible;
- cambia la fuente mantenida, la identidad/licencia de runtime/modelo o la API
  Remote Settings;
- Zed ofrece una API de consentimiento/instalacion que permita una entrega
  equivalente sin debilitar los gates;
- el store XDG no puede asegurar propietario, permisos, leases y limpieza;
- se propone FFI, proceso persistente, otra plataforma o redistribucion;
- otro candidato supera los mismos gates con menor superficie.
