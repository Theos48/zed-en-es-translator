# Investigacion: contrato de traduccion y Provider

> Nota de vigencia: esta investigacion conserva contexto de la planeacion
> inicial. Las decisiones sobre el servidor MCP TypeScript y el puente
> TypeScript -> CLI Rust fueron reemplazadas para F005 por ADR 0003 y
> `specs/002-mcp-server/`: servidor MCP Rust con `rmcp`, stdio y llamada directa
> a `translator-core`.

## Objetivo

Definir el contrato minimo entre:

1. Servidor MCP TypeScript.
2. CLI/core Rust.
3. Proveedores de traduccion mock, CLI local, HTTP local y HTTP remoto configurado.

## Fuentes oficiales

### MCP tools

La especificacion MCP define tools con:

- `name`;
- `description`;
- `inputSchema`;
- resultado con `content`;
- errores de ejecucion de tool representados con `isError: true`.

Tambien indica que los servidores deben validar entradas y que los clientes deberian pedir confirmacion para operaciones sensibles o con riesgo de exfiltracion de datos.

Fuente: <https://modelcontextprotocol.io/specification/2025-06-18/server/tools>

### Zed MCP extensions

Zed expone servidores MCP al Agent Panel mediante extensiones. La extension registra servidores en `extension.toml` y el wrapper Rust implementa `context_server_command`, devolviendo comando, argumentos y entorno para arrancar el servidor.

Fuente: <https://zed.dev/docs/extensions/mcp-extensions>

### Spec Kit

Spec Kit separa especificacion, plan, contratos y tareas. Sus plantillas esperan que el plan documente contexto tecnico, estructura real y contratos cuando aplique.

Fuente: <https://github.com/github/spec-kit>

## Contexto propio

Decisiones relevantes:

- MVP solo lectura, no modifica buffer.
- Tools MCP iniciales: `translate_text` y `translate_file`.
- Ingles -> espanol como primera implementacion.
- Espanol neutro tecnico.
- `MockProvider` primero para TDD.
- Provider futuro compatible con CLI local, HTTP local y HTTP remoto configurado.
- Confirmacion explicita antes de enviar texto fuera del equipo.
- Archivo completo limitado a 20 KiB por defecto.
- Core Rust llamado por CLI desde TypeScript.
- Primer ciclo Spec Kit limitado a core, mock, contrato CLI, limites y pruebas negativas.
- Remoto default deny y confirmado por servidor/core.

## Decision A: TranslateRequest

```text
TranslateRequest
- source_language: "en"
- target_language: "es"
- tone: "technical_neutral"
- preserve_formatting: true
- input_kind: "text" | "markdown"

Direct text variant:
- source_text: string

File variant:
- workspace_root: string
- file_path: string
- source_text: omitted from public request and populated internally after a
  validated file read
```

Razon:

- `source_text` cubre texto pegado o seleccion directa.
- `source_language`, `target_language` y `tone` dejan el contrato extensible sin implementar multiidioma todavia.
- `preserve_formatting` explicita el requisito de no romper Markdown/codigo; en el MVP sera obligatorio.
- `input_kind` permite que el core elija estrategia de segmentacion.
- `workspace_root` y `file_path` habilitan lectura segura de archivo dentro del
  workspace autorizado; el caller no suministra `source_text` en esa variante.
- `file_path` es contexto local para errores de archivo, pero no debe enviarse al proveedor salvo decision futura.

No incluir configuracion de proveedor en `TranslateRequest`.

## Decision B: salida publica

```text
TranslateSuccess
- translated_text: string

TranslateFailure
- code: ErrorCode
- message: string
```

Razon:

- La salida exitosa cumple la decision de mostrar traduccion limpia.
- Los errores se mantienen accionables cuando algo impida traducir.
- El servidor MCP puede mapear exito a `content: [{ type: "text", text: translated_text }]`.
- El servidor MCP puede mapear fallo a resultado de tool con `isError: true`.

Metadata, advertencias y segmentos protegidos podran existir internamente para pruebas, pero no son parte de la salida normal del MVP.

## Decision C: ErrorCode inicial

```text
INVALID_INPUT
UNSUPPORTED_LANGUAGE_PAIR
UNSUPPORTED_FILE_TYPE
FILE_TOO_LARGE
FILE_NOT_FOUND
PATH_NOT_ALLOWED
NON_UTF8_INPUT
NO_TRANSLATABLE_SEGMENTS
SECRET_DETECTED
PROVIDER_NOT_CONFIGURED
REMOTE_CONFIRMATION_REQUIRED
PROVIDER_FAILED
PROVIDER_TIMEOUT
INTERNAL_ERROR
```

Razon:

- Cubre errores de validacion de input.
- Cubre alcance de idiomas.
- Cubre lectura de archivo y limite de 20 KiB.
- Cubre frontera de workspace y entradas no UTF-8.
- Cubre ausencia de segmentos traducibles.
- Cubre deteccion basica de secretos antes de proveedor remoto.
- Cubre privacidad para proveedores remotos.
- Cubre timeouts y fallos de proveedor sin filtrar detalles sensibles.

## Decision D: Provider por segmentos

```text
ProviderRequest
- segments: string[]
- source_language: "en"
- target_language: "es"
- tone: "technical_neutral"

ProviderResponse
- translated_segments: string[]
```

Razon:

- El core mantiene responsabilidad de segmentar, proteger codigo/formato y reconstruir salida.
- El proveedor solo traduce texto permitido.
- El mock puede ser determinista segmento por segmento.
- Proveedores futuros CLI/HTTP no necesitan entender Markdown ni codigo fuente.

## Decision E: wire contract TS -> CLI Rust

El servidor MCP TypeScript invocara el CLI Rust con:

```text
stdin: JSON UTF-8 con un TranslateRequest serializado
stdout exito: JSON UTF-8 con TranslateSuccess
stdout error: JSON UTF-8 con TranslateFailure
stderr: diagnostico redaccionado, nunca contenido fuente
exit 0: stdout contiene TranslateSuccess
exit != 0: stdout contiene TranslateFailure si es posible
timeout: 15 s por request en el primer ciclo
```

Reglas:

- una request por proceso CLI;
- no pasar texto traducible por argv;
- no imprimir `source_text`, segmentos, traduccion completa, tokens ni headers en stderr;
- si el proceso excede timeout, el adaptador MCP devuelve `PROVIDER_TIMEOUT`;
- si stdout no es JSON valido, el adaptador MCP devuelve `INTERNAL_ERROR` con mensaje redaccionado;
- el servidor MCP mapea errores a tool result con `isError: true`;
- el servidor MCP mapea exito a `content: [{ type: "text", text: translated_text }]`.

Razon:

- MCP define schema y resultados de tools, pero el puente TS -> Rust es responsabilidad del proyecto.
- JSON por stdin/stdout evita quoting inseguro de shell y permite fixtures simples.
- Una request por proceso reduce estado compartido y simplifica timeouts para el MVP.

## Decision F: limites del contrato

Limites iniciales:

```text
max_input_bytes: 20480
max_segment_bytes: 4096
max_segments: 256
max_output_bytes: 40960
provider_timeout_ms: 15000
```

Los limites aplican a `translate_text` y `translate_file`. Para `translate_file`, el limite se valida por bytes antes de cargar todo el contenido cuando la plataforma lo permita.

## Decision G: datos permitidos hacia Provider

El `Provider` recibira solo:

- segmentos traducibles;
- idiomas;
- tono.

No recibira:

- `file_path`;
- workspace root;
- texto protegido;
- bloques de codigo;
- variables de entorno;
- secretos detectados.

Si se detecta un secreto obvio y el provider no es local/offline, el MVP tecnico debe devolver `SECRET_DETECTED`.

## Criterios de revision

Revisar este contrato cuando:

- Se implemente el primer proveedor real.
- Se agreguen idiomas adicionales.
- Se decida exponer metadata o advertencias al usuario.
- Se cambie de CLI Rust a WASM u otro mecanismo TS-Rust.
