# ADR 0007: convergencia en una sola arquitectura Gallery

## Estado

Aceptado. Reemplaza las superficies operativas definidas en ADR 0001, 0002,
0003 y 0005, y reemplaza parcialmente ADR 0004. ADR 0006 permanece vigente
como definicion del paquete publicado.

## Contexto

La feature 009 ya implementa la experiencia objetivo: la extension de Zed
adquiere un paquete exacto, lanza `translator-lsp` y traduce localmente mediante
el runtime embebido. El CLI, MCP/Agent, LibreTranslate, Azure y la seleccion
manual de proveedores no tienen consumidores en ese paquete. Mantenerlos
duplica dependencias, pruebas, automatizacion y documentacion, y contradice la
instalacion plug-and-play que se pretende publicar.

## Decision

El repositorio conserva una sola cadena de producto:

```text
zed-extension -> translator-lsp -> translator-core -> embedded runtime
```

Se eliminan las superficies CLI, MCP/Agent y proveedores configurables, junto
con sus dependencias, scripts, fixtures, pruebas e instrucciones operativas.
`MockProvider` queda exclusivamente como doble de prueba. Git conserva la
historia completa; los ADRs conservados explican decisiones anteriores con
estado de supersesion explicito. La limpieza de salidas generadas usa una
allowlist normal y una limpieza profunda separada.

La constitucion 2.0.0 formaliza esta frontera y conserva como obligaciones la
no-mutacion, operacion local/offline, limites, seguridad de paths, procesos
acotados, redaccion, supply chain y politica de host limpio.

## Consecuencias

- El workspace raiz contiene solo `translator-core` y `translator-lsp`.
- El LSP construye directamente el proveedor embebido adyacente y no acepta
  provider, URL, credencial, confirmacion remota ni binario arbitrario.
- Las pruebas se concentran en invariantes del producto y del paquete vigente.
- Las features 001 a 007 salen del working tree cuando sus restricciones vivas
  han migrado a 009, 010, la constitucion o este registro de decisiones.
- Cambios de core/LSP obligan a regenerar la identidad del paquete antes de
  crear `v0.1.0` o continuar la submission a la Gallery.

## Criterio de revision

Revisar solo si una API estable de Zed permite una experiencia mas directa con
garantias equivalentes, si se agrega otra plataforma mediante un paquete
reproducible propio, o si se propone reintroducir una frontera publica. Esto
ultimo requiere nueva feature, evidencia de consumidor y enmienda constitucional.
