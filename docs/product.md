# Producto y roadmap funcional

## Problema

Traducir texto tecnico de ingles a espanol durante la edicion rompe el flujo
cuando obliga a cambiar de aplicacion, copiar texto a una pagina externa o
perder formato.

## Usuario objetivo

Persona que trabaja en Zed con documentacion, comentarios, README, issues,
prompts o notas tecnicas en ingles y necesita una traduccion rapida al espanol
dentro del editor.

## Propuesta

Una herramienta integrada en Zed que reciba texto en ingles y devuelva espanol
claro, manteniendo estructura y formato cuando sea posible.

La experiencia final esperada no depende del Agent Panel. El usuario debe poder
seleccionar texto en un documento abierto, usar una accion de la extension
desde menu contextual, comando o boton, y ver la traduccion dentro de Zed sin
configurar un Agent ni un modelo intermediario. El flujo Agent Panel se acepta
solo como puente de validacion mientras se aprovechan las tools MCP existentes.

## Objetivo actual

El flujo directo de extension ya fue implementado y validado. El siguiente
objetivo de producto es configurar y comprobar dos caminos de traduccion
reales desde esa extension: un proveedor local/offline y un proveedor
remoto/online, sin confundir el adaptador ya implementado con un servicio real
en funcionamiento.

El servidor MCP, el flujo Agent Panel y las validaciones de F007 quedan como
infraestructura y evidencia historica. No son la superficie principal que debe
guiar nuevas features de producto.

Regla de direccion: desde F006, cualquier feature que toque Zed debe empujar la
extension hacia una experiencia propia. Agent Panel solo puede aparecer como
puente tecnico, prueba de compatibilidad o workaround documentado cuando la API
vigente de Zed impida una accion directa. No se debe acumular producto sobre
Agent Panel para migrarlo al final.

## Vision de producto final

El producto final debe sentirse como una extension nativa de traduccion para
Zed, disenada como producto propio, pulido y ambicioso para trabajo tecnico.

Objetivos:

- traducir seleccion desde menu contextual, comando o boton;
- mostrar preview legible antes de cualquier cambio;
- permitir copiar, insertar o aplicar la traduccion solo por accion explicita
  del usuario;
- conservar Markdown, listas, enlaces, bloques de codigo e inline code;
- soportar documentos permitidos sin mutar el archivo original por defecto;
- evolucionar hacia comentarios/docstrings en codigo con segmentacion segura;
- ofrecer proveedor local o gratuito/no pago como ruta principal;
- mantener remoto default deny con confirmacion por solicitud;
- evitar que el usuario tenga que configurar Agent, perfiles Agent o modelos
  intermediarios para traducir.

## Primer ciclo tecnico

Antes del MVP usable con proveedor real, el primer ciclo formal construye una
base segura:

- core de traduccion independiente de Zed;
- `MockProvider` determinista para TDD;
- contrato CLI Rust para validar el core como frontera publica;
- soporte inicial de `translate_file` para `.md`, `.markdown` y `.txt`;
- limites, validaciones de ruta y pruebas negativas de seguridad;
- privacidad remota default deny.

## Roadmap funcional

El backlog detallado vive en `docs/feature-map.md`. Resumen:

1. Contrato de traduccion.
2. Preservacion segura de Markdown/texto.
3. Proveedor mock.
4. Proveedor real configurable.
5. Servidor MCP.
6. Fundacion de extension Zed.
7. UX de lectura dentro de Zed.
8. Privacidad y configuracion.
9. Flujo directo de extension Zed sin Agent.
10. Configuracion operativa de un proveedor real local/offline y otro
    remoto/online.
11. Empaquetado y publicacion.

## Fuera de alcance inicial

- Traduccion multiidioma.
- Correccion gramatical general.
- Interfaz grafica compleja.
- Entrenamiento de modelos.
- Instalacion global de runtimes o servicios en el host.
- Proveedor real en el primer ciclo tecnico.
- Reemplazo automatico de seleccion.
- Soporte de archivo completo para codigo fuente antes de tener
  segmentador/parser y pruebas suficientes.
