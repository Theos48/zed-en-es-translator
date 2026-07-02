# ADR 0001: alcance tecnico inicial

## Estado

Aceptado.

## Contexto

La documentacion oficial de Zed describe las extensiones como repositorios Git con `extension.toml`. Las capacidades publicas documentadas se centran en lenguajes, temas, depuradores, snippets y servidores MCP.

Para una herramienta de traduccion, el camino documentado mas cercano es exponer un servidor MCP utilizable desde el Agent Panel. La primera version no debe modificar ni reemplazar texto en el buffer; solo debe mostrar una traduccion para lectura.

Zed documenta MCP como via de integracion con el Agent Panel. Aun asi, el
servidor MCP debe poder existir como pieza reutilizable y la extension de Zed
debe tratarse como wrapper de integracion, no como unico canal posible.

## Decision

El alcance inicial sera:

1. Core de traduccion testeable fuera de Zed.
2. Servidor MCP con una herramienta de traduccion ingles -> espanol.
3. Extension de Zed como wrapper para ejecutar el servidor en modo dev.
4. Flujo de lectura desde Agent Panel sin edicion automatica del buffer.

El primer ciclo formal de implementacion no intentara resolver traduccion real ni soporte completo de codigo. El alcance inicial aceptado es un MVP tecnico con core, mock determinista, contrato CLI, limites, controles de privacidad y pruebas negativas.

## Consecuencias

- Podemos avanzar con TDD sin depender de Zed desde el primer dia.
- El MVP queda integrado en Zed por Agent Panel.
- Evitamos riesgo de cambios destructivos en el buffer.
- La accion directa de editor queda fuera del alcance inicial mientras el producto busca lectura asistida, no edicion automatica.
- La extension Zed queda desacoplada del servidor MCP para reducir riesgo si cambia la via recomendada por Zed.
- La traduccion usable con proveedor real queda fuera del primer ciclo tecnico.

## Criterio de revision

Este ADR debe revisarse cuando exista evidencia practica de:

- Limitaciones reales de MCP en Zed.
- Camino recomendado por Zed para herramientas no relacionadas con lenguajes.
- Cambio de alcance hacia edicion automatica o reemplazo de seleccion.
