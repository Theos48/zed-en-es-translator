# ADR 0002: arquitectura y tecnologia inicial

## Estado

Superseded by [ADR 0007](./0007-repository-convergence.md). Las decisiones que
originaron el core seguro permanecen vigentes a través de la constitución
2.0.0; la arquitectura multipista descrita aquí es solo historia.

Todo el contenido siguiente describe alternativas y fronteras iniciales. La
única arquitectura vigente está definida por ADR 0006, ADR 0007 y la
constitución 2.0.0.

## Contexto

El proyecto busca una extension para Zed que permita leer traducciones de ingles a espanol sin salir del editor. El usuario no quiere modificar el texto original ni reemplazar la seleccion automaticamente. La traduccion debe servir para Markdown, documentacion, comentarios y docstrings, evitando traducir codigo siempre que sea posible.

No se usaran APIs de pago. El desarrollo debe poder iniciar con un proveedor mock para TDD, pero la arquitectura debe admitir un motor local futuro y, si se decide despues, proveedores gratuitos/remotos con confirmacion explicita antes de enviar texto fuera del equipo.

Zed documenta extensiones con `extension.toml`, logic custom en Rust/WASM cuando aplique, y servidores MCP como via de integracion con el Agent Panel. Para el MVP tecnico el Agent Panel fue suficiente; para el producto final ya no debe ser requisito de uso.

## Decision

La arquitectura inicial separara responsabilidades en capas:

1. Core de traduccion independiente de Zed.
2. Segmentador/protector de contenido para preservar codigo, Markdown y simbolos sensibles.
3. Contrato de proveedor de traduccion intercambiable.
4. `MockProvider` determinista para TDD.
5. Adaptador MCP para exponer la traduccion al Agent Panel.
6. Wrapper de extension Zed en Rust para registrar o arrancar el servidor cuando corresponda.

La particion tecnologica inicial sera:

1. Rust para wrapper Zed y core de traduccion.
2. El servidor MCP de F005 se implementa en Rust con `rmcp`, segun ADR 0003.
3. Contratos explicitos entre fronteras para evitar acoplamiento accidental.
4. El CLI Rust se conserva como frontera publica y de pruebas del core.

El MVP tecnico inicial:

1. Implementara ingles -> espanol.
2. Mostrara la traduccion en el Agent Panel como puente de validacion.
3. No modificara buffers ni reemplazara selecciones.
4. Devolvera una traduccion limpia por defecto.
5. Permitira traducir seleccion o archivo completo con limites claros.
6. Preservara bloques de codigo, inline code y estructura basica de Markdown.
7. Permitira traducir comentarios y docstrings cuando el contenido venga de codigo.
8. Usara `translate_text` y `translate_file` como tools MCP iniciales.
9. Tratara seleccion como flujo UX o alias sobre `translate_text`.
10. Limitara traduccion de archivo completo a 20 KiB por defecto.
11. Usara salida limpia en caso exitoso y errores claros cuando algo impida traducir.

El primer ciclo formal de Spec Kit sera mas estrecho que el MVP usable:

1. Core Rust testeable.
2. `MockProvider` determinista.
3. Contrato CLI Rust.
4. Limites de entrada/salida.
5. Seguridad de lectura para `translate_file`.
6. Privacidad remota default deny.
7. Pruebas negativas de seguridad.

En ese primer ciclo, `translate_file` aceptara solo `.md`, `.markdown` y `.txt`. El soporte de archivos de codigo completo queda pospuesto hasta tener segmentador/parser confiable y pruebas de preservacion para comentarios/docstrings.

El contrato `Provider` debera admitir estas familias de implementacion:

1. Mock determinista.
2. CLI local.
3. HTTP local.
4. HTTP remoto configurado manualmente y sin API de pago.

No se elegira motor local real en esta fase.

Los proveedores remotos estaran deshabilitados por defecto. Cualquier proveedor remoto futuro requerira configuracion explicita, confirmacion por cada traduccion y validacion en servidor/core. La confirmacion no se delegara solamente al cliente MCP.

`translate_file` solo podra leer archivos dentro del workspace autorizado por Zed, despues de canonicalizar la ruta. Debe rechazar traversal con `..`, symlinks que escapen del workspace, contenido binario, entradas no UTF-8 y archivos ocultos sensibles.

El contrato CLI Rust usa JSON UTF-8 por stdin/stdout, una request por proceso,
exit code 0 para exito, exit code no cero para error, stderr redaccionado y
timeout externo. En F005, el servidor MCP Rust llama a `translator-core`
directamente y mapea errores del core a resultados MCP con `isError: true`.

Los logs no deben contener texto fuente, segmentos, traducciones completas, secretos, headers, tokens ni rutas sensibles sin redaccion.

Spec Kit ya estaba inicializado. La planeación estratégica vive en `docs/` y
la implementación activa en `specs/<feature>/`; Git conserva el ciclo formal
que originó esta decisión.

## Consecuencias

- El core puede probarse sin Zed, sin red y sin proveedor real.
- La integracion inicial se mantiene alineada con capacidades documentadas de
  Zed, pero la UX final se mueve a una accion propia de extension.
- La falta de reemplazo automatico reduce riesgo de ediciones destructivas.
- La arquitectura evita acoplar el proyecto a un unico proveedor de traduccion.
- El servidor MCP puede evolucionar separado del wrapper Zed.
- Las fronteras publicas requieren contratos claros y pruebas de integracion.
- El contrato CLI se conserva aunque F005 no lo use como puente interno del
  servidor MCP.
- Las decisiones de instalacion se pueden tomar caso por caso siguiendo la politica del sistema.
- La traduccion real se retrasa deliberadamente hasta que existan controles verificables.
- El soporte de codigo se implementara con criterio conservador: si no se puede distinguir comentario/docstring de codigo con confianza, se preserva sin traducir.
- `docs/` no debe duplicar el detalle operativo de `specs/<feature>/`; registra
  decisiones estables, ADRs, investigacion y roadmap.

## Criterio de revision

Este ADR debe revisarse cuando:

- Exista evidencia practica sobre los limites reales de MCP en Zed.
- Se seleccione un motor local o proveedor remoto real.
- Cambie la estructura formal de Spec Kit o la jerarquia de fuentes de verdad.
- El costo de mantener Rust + TypeScript supere los beneficios de separacion.
- Se valide que Zed entregue seleccion directamente al Agent Panel con baja friccion.
- Se agregue soporte de archivo completo para codigo fuente.

Revision aplicada: D065/D066/D071 establecen que la siguiente evolucion debe
quitar Agent Panel de la superficie principal de usuario antes de publicar.
D072 agrega que esa orientacion aplica desde F006 en adelante.
