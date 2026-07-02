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

## Primer ciclo tecnico

Antes del MVP usable con proveedor real, el primer ciclo formal construye una
base segura:

- core de traduccion independiente de Zed;
- `MockProvider` determinista para TDD;
- contrato CLI entre servidor MCP futuro y core Rust;
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
6. Wrapper Zed.
7. UX de lectura dentro de Zed.
8. Privacidad y configuracion.
9. Empaquetado y publicacion.

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
