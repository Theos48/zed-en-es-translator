# ADR 0004: flujo directo Zed mediante LSP

## Estado

Aceptado. Implementacion automatizada y validacion manual interactiva completas
en `specs/006-direct-zed-translation/manual-validation.md`.

## Contexto

F010 debe traducir seleccion o contenido permitido del documento abierto sin
Agent Panel, perfiles Agent, prompts ni un modelo intermediario. El buffer no
puede modificarse y el proveedor remoto conserva allowlist, confirmacion por
solicitud y bloqueo de secretos.

La API estable `zed_extension_api = 0.7.0` permite lanzar language servers,
pero no expone acciones genericas de extension, seleccion del editor,
clipboard, webviews o paneles propios.

## Decision

La extension registra `en-es-translator` para Markdown y Plain Text y lanza un
nuevo binario Rust `translator-lsp`:

- `textDocument/codeAction` recibe el rango UTF-16 y devuelve una accion sin
  `WorkspaceEdit`;
- `workspace/executeCommand` transporta URI, version, rango y tipo, nunca texto;
- `window/showMessageRequest` confirma cada solicitud remota;
- `textDocument/hover` muestra el preview versionado de solo lectura;
- `translator-core` conserva limites, segmentacion, seguridad de archivos,
  provider, secretos y redaccion;
- `translator-mcp` y Agent Panel quedan como compatibilidad historica.

## Razon

LSP es la unica superficie estable verificada que entrega seleccion y version,
ofrece una accion nativa y puede mostrar contenido dentro de Zed sin editar el
documento. Separar protocolo editor en `translator-lsp` evita mezclar su ciclo
de vida con MCP y mantiene una sola autoridad de contenido en `translator-core`.

## Consecuencias

- El workspace agrega `crates/translator-lsp/` y dependencias bloqueadas
  `lsp-server` 0.7.9 y `lsp-types` 0.97.0.
- La accion muestra localidad segura: offline, local o remoto con confirmacion.
- Un cambio o cierre de documento invalida preview y solicitud pendiente.
- No existe copy, insert, replace, apply ni panel propio en esta feature.
- La extension requiere una ruta absoluta preparada a `translator-lsp` en la
  configuracion LSP local.
- La seleccion de proveedor del LSP usa unicamente cuatro entradas validadas en
  `lsp.en-es-translator.binary.env`; Zed 1.10.3 no propago la configuracion
  anidada del proveedor al entorno del proceso durante el smoke real.
- El artefacto y las pruebas Rust siguen ejecutandose dentro del contenedor del
  proyecto; no se instala toolchain en el host.

## Criterio de revision

Revisar si una futura API estable de Zed ofrece acciones directas, clipboard o
preview propio con contratos equivalentes de seleccion, version, privacidad y
no-mutacion. Insertar o aplicar texto sigue requiriendo primero una enmienda de
la constitucion.
