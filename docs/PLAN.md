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
Zed; fue fusionada a `main`. El detalle operativo vive en
`specs/003-zed-wrapper/` y las decisiones estables en D063/D064:

```text
specs/003-zed-wrapper/
```

Esta iteracion empaqueta el servidor MCP existente como extension local de
desarrollo de Zed en `zed-extension/` y cubre startup local, diagnosticos
redaccionados, preparacion reproducible y no-mutacion. No incluye proveedor
real, red, publicacion, UX avanzada ni edicion automatica de buffers.

La cuarta feature formal fue implementada y validada:

```text
specs/004-zed-ux-flow/
```

Esta iteracion define y valida el flujo de lectura dentro de Zed sobre la
extension local ya fusionada. Documenta un flujo Agent Panel de baja friccion,
mantiene los contratos `translate_text` y `translate_file`, preserva la
no-mutacion y registra la decision de soporte de seleccion con evidencia manual
real. No incluye proveedor real, red, publicacion, API keys ni reemplazo
automatico de contenido. La guia operativa vive en `docs/zed-ux-flow.md`;
`make test-zed-ux-flow` valida los contratos documentales y de evidencia. Este
flujo Agent Panel es un puente de validacion, no la experiencia final de
producto; la experiencia final queda registrada en D065/F010 como una accion
propia de la extension que no requiere configurar Agent.

La quinta feature formal fue implementada y validada:

```text
specs/005-real-provider-config/
```

Esta iteracion promueve F004 como hito de proveedor real configurable sin
debilitar la privacidad por defecto. El detalle operativo, gates y evidencia de
validacion viven en `specs/005-real-provider-config/`. El hito implemento el
adaptador LibreTranslate-compatible y sus controles, pero no desplego ni dejo
configurado un proveedor real de uso cotidiano; la evidencia automatizada uso
servicios loopback simulados.

La sexta feature formal fue implementada y validada, incluida la validacion
manual interactiva:

```text
specs/006-direct-zed-translation/
```

Esta iteracion promueve F010 mediante un language server Rust. Zed ofrece una
code action con localidad segura, entrega rango/version sin Agent, ejecuta la
traduccion por comando y muestra el resultado en hover. No hay ediciones,
clipboard ni panel propio. El proveedor directo se configura por una allowlist
en `binary.env` tras la brecha detectada en el smoke real. La arquitectura
estable vive en D073-D075 y ADR 0004; el estado manual real vive en
`manual-validation.md`.

La septima feature formal fue implementada y validada en
`specs/007-operational-providers/`. Demostro el camino local real y offline,
pero su ciclo Docker/LibreTranslate queda como superficie de desarrollo y
compatibilidad, no como instalacion del producto.

La feature activa convierte F012 y F009 en una sola entrega orientada a la
Gallery:

```text
specs/009-zed-marketplace-install/
```

La extension prepara automaticamente un paquete local Bergamot/Marian y tres
recursos Mozilla `en -> es` de identidad fija, lo ejecuta offline y conserva
todo dentro del work directory de Zed. La implementacion y validacion
automatizada estan completas; tag/release, submission upstream y aceptacion
limpia desde Gallery siguen siendo gates externos obligatorios.

Direccion actual:

- la base tecnica existe: core, CLI, MCP, fundacion de extension Zed local,
  flujo Agent Panel validado y proveedor real configurable;
- el Agent Panel queda como puente de validacion e infraestructura historica,
  no como experiencia final de producto;
- F006 debe entenderse como la fundacion de la extension Zed, no como permiso
  para construir el producto alrededor de Agent Panel;
- desde F006, toda feature que toque Zed debe avanzar la extension directa o
  documentar una limitacion concreta de la API de Zed que obligue a un puente;
- F010 esta completada como flujo directo LSP con smoke manual aprobado;
- F011/configuracion operativa de proveedores reales esta completada en
  `specs/007-operational-providers/`;
- F011 selecciona LibreTranslate 1.9.6 fijado por digest como camino soportado
  sin cuenta ni API key; CLI/Zed directo/offline/fallo de update/rollback y
  limpieza project-scoped pasan;
- Azure AI Translator Text v3 permanece como adaptador avanzado opcional bajo
  target fijo, consentimiento por solicitud y pruebas controladas; no requiere
  evidencia real ni bloquea F011;
- el camino publicado no usa el modelo Argos ni el ciclo Docker; usa recursos
  Mozilla exactos con avisos y conclusiones de licencia registrados;
- F012 y F009 convergen en `specs/009-zed-marketplace-install/`: Gallery,
  preparacion automatica, traduccion offline y remocion por Zed son una sola
  experiencia plug-and-play;
- una PR local o dev extension no cierra la feature: tag/asset, submission al
  registro y 3/3 instalaciones limpias posteriores al merge siguen abiertos.

## Flujo por feature

Para cada iteracion:

1. Elegir el siguiente bloque desde `docs/feature-map.md`.
2. Usar esa entrada como contexto para `speckit-specify`.
3. Ejecutar `speckit-clarify` y registrar si no hay ambiguedades criticas.
4. Ejecutar `speckit-checklist` cuando la feature tenga riesgos especificos
   (seguridad, privacidad, UX, proveedor, publicacion, datos) o cuando el
   usuario lo pida.
5. Usar `speckit-plan` para fijar tecnologias, estructura, contratos y limites
   concretos de esa feature.
6. Usar `speckit-tasks` para generar trabajo ejecutable.
7. Usar `speckit-analyze` despues de `tasks.md` y antes de implementar; si no
   aplica, registrar el prerequisito que lo bloquea.
8. Usar `speckit-implement` para ejecutar `tasks.md` con TDD.
9. Usar `speckit-converge` despues de implementar para append-only de trabajo
   restante; si no aplica, registrar el prerequisito que lo bloquea.

No cerrar una iteracion diciendo que el flujo esta completo sin reportar el
estado de `checklist`, `analyze` y `converge` como ejecutado, no aplicable o
bloqueado.

Durante este flujo, `docs/feature-map.md` conserva el backlog. La feature
formal vive en `specs/<feature>/`.

Los worktrees temporales se crean bajo
`~/dev/.worktrees/zed-en-es-translator/`, no bajo `/tmp` ni otro
`tmpfs`/`ramfs`. Antes de compilar se ejecuta `make workspace-storage-check` y
para detectar cualquier checkout registrado fuera de esta regla se usa `make
worktree-audit`. La retirada se hace con `git worktree remove` y `git worktree
prune`, despues de comprobar que el checkout este limpio y sin procesos o
montajes activos.

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

### 3. Fundacion de extension Zed

Completado en la tercera feature formal de Spec Kit con TDD. Aunque la
implementacion concreta empaqueta el servidor MCP como context server local,
esta etapa se interpreta como la fundacion de la extension Zed, no como el
destino de producto:

- `zed-extension/extension.toml`;
- crate Rust/WASM aislada con `zed_extension_api = "0.7.0"`;
- `context_server_command` para arrancar `translator-mcp` como comando directo;
- preparacion reproducible mediante `make zed-extension-prepare`;
- validacion con `make test-zed-extension`;
- diagnosticos redaccionados y revision de no-mutacion.
- frontera documentada para que las siguientes features muevan la UX hacia una
  accion propia de la extension cuando la API de Zed lo permita.

### 4. UX de lectura

Completado como cuarta feature formal de Spec Kit:

```text
specs/004-zed-ux-flow/
```

Reducir friccion dentro de Zed:

- flujo Agent Panel solo como puente de validacion F007;
- seleccion si Zed la expone de forma fiable;
- resultado legible sin modificar buffers.
- guia de revision en `docs/zed-ux-flow.md`;
- plantilla de evidencia manual redaccionada en
  `specs/004-zed-ux-flow/manual-validation-template.md`;
- checks documentales con `make test-zed-ux-flow`.

Este bloque no debe repetirse como direccion de producto en nuevas features. Si
una feature futura toca UX de Zed, debe partir de la extension directa; Agent
Panel solo puede quedar como fallback justificado.

### 5. Proveedor real configurable

Completado como quinta feature formal de Spec Kit:

```text
specs/005-real-provider-config/
```

Integra el primer proveedor real configurable sin cambiar el default offline ni
el limite de privacidad. El detalle operativo vive en
`specs/005-real-provider-config/`. Su alcance fue el adaptador y la frontera de
configuracion, no desplegar o mantener una instancia real.

### 6. Flujo directo sin Agent

Completado como sexta feature formal, incluida validacion manual en Zed:

- accion propia de la extension desde menu contextual, comando o boton;
- traducir texto seleccionado o contenido permitido del documento abierto;
- mostrar la traduccion dentro de Zed sin configurar Agent Panel;
- mantener no-mutacion automatica del buffer;
- usar proveedor local o gratuito/no pago, con remoto siempre explicitamente
  configurado y confirmado por solicitud.
- tratar el servidor MCP y el flujo Agent Panel como infraestructura o puente
  de compatibilidad, no como superficie primaria de usuario.
- usar code action, execute command y hover de LSP porque la API estable 0.7.0
  no ofrece accion generica, clipboard o panel propio.

### 7. Configuracion operativa de proveedores reales

Completada como septima feature formal:

```text
specs/007-operational-providers/
```

- usar LibreTranslate 1.9.6 fijado por digest como proveedor local/offline;
- conservar Azure AI Translator Text v3 como adaptador remoto avanzado
  opcional, nunca como requisito del camino base;
- preparar el proveedor local como servicio reproducible y aislado del
  proyecto, sin instalar runtimes o servicios globales en Fedora;
- mantener LibreTranslate solo en red interna y publicar loopback mediante un
  relay project-scoped de destino fijo, sin logs de contenido;
- documentar inicio, parada, actualizacion, persistencia, verificacion y
  rollback del camino local;
- mantener el adaptador remoto por HTTPS, host allowlisted, secretos fuera
  del repositorio y confirmacion por cada solicitud;
- validar el proveedor local con traducciones sinteticas reales desde CLI y el
  flujo directo de Zed; validar la seguridad remota con dobles controlados;
- conservar `MockProvider` como default, no-mutacion, redaccion, limites y
  bloqueo de secretos;
- exigir evidencia manual contra el servicio local real ademas de los stubs
  automatizados, sin cuenta ni API key.
- no redistribuir el modelo Argos `en-es` mientras su licencia siga sin
  declarar en upstream; este gate debe resolverse antes de publicar un bundle.

### 8. Paquete automatico y publicacion en Gallery

Activa en `specs/009-zed-marketplace-install/`, combinando F012 y F009:

- instalar desde la Gallery sin terminal, checkout, Docker, cuenta, key, ruta
  o provider manual;
- preparar automaticamente el paquete Linux `x86_64` con fuentes y hashes
  fijos;
- traducir localmente y offline con limites, redaccion y no-mutacion;
- conservar active/previous y recuperar fallos sin limpieza manual;
- dejar runtime/modelos/estado bajo propiedad de Zed para disable/uninstall;
- publicar tag/asset exacto, abrir PR del proyecto y submission upstream;
- cerrar solo tras tres instalaciones limpias desde la Gallery ya fusionada.
