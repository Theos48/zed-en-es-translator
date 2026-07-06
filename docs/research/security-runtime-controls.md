# Investigacion: controles de seguridad y runtime

## Objetivo

Definir controles verificables antes de ejecutar el flujo formal de Spec Kit:

1. Frontera de lectura de `translate_file`.
2. Consentimiento remoto.
3. Logs y secretos.
4. Ejecucion de proveedores CLI/HTTP futuros.
5. Supply chain minima.

## Fuentes oficiales

### MCP

La especificacion de tools MCP define que los servidores exponen tools con `inputSchema` y resultados con `content`. Tambien trata los errores de tool con `isError: true` e indica que las entradas deben validarse.

Fuente: <https://modelcontextprotocol.io/specification/2025-06-18/server/tools>

### Zed

Zed documenta que las extensiones MCP registran servidores en `extension.toml` y que el wrapper implementa `context_server_command`, devolviendo comando, argumentos y entorno.

Fuente: <https://zed.dev/docs/extensions/mcp-extensions>

Zed documenta que las extensiones son repositorios Git con `extension.toml` y requisitos de publicacion como licencia.

Fuente: <https://zed.dev/docs/extensions/developing-extensions>

### Spec Kit

Spec Kit separa `specify`, `plan`, `tasks` e `implement`. El plan tecnico debe capturar decisiones de arquitectura, contratos y criterios verificables antes de generar tareas.

Fuente: <https://github.com/github/spec-kit>

## Contexto propio

- El MVP no modifica buffers.
- No se usaran APIs de pago.
- Todo envio fuera del equipo requiere confirmacion.
- El primer ciclo formal sera mock/offline.
- La politica del host exige evitar runtimes globales de proyecto y decidir dependencias caso por caso.

## Decision A: lectura de archivos

`translate_file` solo puede leer dentro del workspace autorizado por Zed.

Reglas:

- canonicalizar workspace y path antes de leer;
- rechazar paths absolutos que no esten dentro del workspace canonicalizado;
- rechazar `..` si el resultado canonicalizado escapa del workspace;
- rechazar symlinks que apunten fuera del workspace;
- rechazar contenido no UTF-8;
- rechazar contenido binario;
- rechazar archivos ocultos sensibles por defecto, especialmente `.env`, `.env.*`, llaves, certificados y credenciales;
- validar extension permitida despues de normalizar casing;
- limitar lectura por bytes.
- rechazar bytes NUL;
- abrir el archivo validado una sola vez y hacer las comprobaciones de tamano,
  tipo y contenido sobre ese mismo handle;
- verificar que el handle abierto corresponde al archivo validado cuando la
  plataforma permita comparar identidad de archivo, para cerrar ventanas
  TOCTOU.

Errores relacionados:

- `PATH_NOT_ALLOWED`;
- `UNSUPPORTED_FILE_TYPE`;
- `FILE_TOO_LARGE`;
- `NON_UTF8_INPUT`;
- `FILE_NOT_FOUND`.

## Decision B: remoto default deny

Los proveedores remotos estan deshabilitados por defecto.

Para habilitar un remoto futuro se requiere:

- configuracion explicita del proveedor;
- confirmacion por cada traduccion;
- preview local de proveedor/host, tamano aproximado y tipo de entrada;
- bloqueo si no hay confirmacion;
- no enviar `file_path`;
- no enviar segmentos con secretos obvios.

La confirmacion debe validarse en servidor/core. La UI o cliente MCP pueden ayudar, pero no son la unica barrera.

Errores relacionados:

- `REMOTE_CONFIRMATION_REQUIRED`;
- `PROVIDER_NOT_CONFIGURED`;
- `SECRET_DETECTED`.

## Decision C: logs

Logs permitidos:

- request id local;
- codigo de error;
- proveedor logico;
- tamanos;
- duracion;
- estado redaccionado.

Logs prohibidos:

- `source_text`;
- segmentos traducibles;
- traduccion completa;
- tokens;
- headers;
- secretos;
- rutas sensibles sin redaccion.

## Decision D: configuracion y secretos

En el MVP tecnico:

- no se versionan `.env` reales;
- no se requiere proveedor real;
- el MCP/CLI usa allowlist de variables cuando la plataforma de lanzamiento
  permita limpiar el entorno heredado;
- para el context server de Zed en F006, D064 documenta que el proceso lanzado
  hereda el entorno del proceso Zed por limitacion de plataforma;
- errores y logs redaccionan valores sensibles.

La ubicacion final de configuracion de proveedor se decidira cuando se implemente proveedor real.

## Decision E: proveedores CLI y HTTP futuros

CLI local futuro:

- ejecutar sin shell;
- usar argumentos estructurados;
- pasar texto por stdin, no argv;
- cwd controlado;
- entorno minimo;
- timeout;
- limite de stdout/stderr;
- allowlist de binarios.

HTTP local futuro:

- solo loopback;
- timeout;
- limite de respuesta;
- sin redirects peligrosos;
- sin acceso a hosts no configurados.

HTTP remoto futuro:

- HTTPS;
- host allowlist;
- confirmacion por request;
- evaluacion de privacidad, no solo costo cero.

## Decision F: supply chain

Antes de publicar o habilitar proveedores reales:

- usar lockfiles;
- revisar versiones y origen de dependencias;
- auditar dependencias Rust y TypeScript;
- revisar scripts de instalacion;
- documentar binarios externos si se usan.

## Pruebas negativas requeridas

- path traversal con `../`;
- path absoluto fuera del workspace;
- symlink dentro del workspace que apunta fuera;
- archivo no UTF-8;
- archivo binario con extension permitida;
- archivo mayor a 20 KiB;
- `translate_text` mayor a 20 KiB;
- remoto sin confirmacion;
- secreto obvio antes de remoto;
- logs sin contenido fuente ni secretos;
- timeout de provider;
- stdout CLI invalido.

## Revision futura

Revisar este documento cuando:

- se valide el flujo real de seleccion en Zed;
- se agregue un proveedor local o remoto real;
- se habilite soporte de archivo completo para codigo fuente;
- se prepare publicacion en Zed o registro MCP.
