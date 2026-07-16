# ADR 0006: paquete local automatico para la Gallery de Zed

## Estado

Aceptado e implementado en `specs/009-zed-marketplace-install/`; publicacion y
aceptacion final en la Gallery pendientes de los gates externos registrados en
la validacion de la feature.

## Contexto

El flujo directo LSP ya permitía traducir dentro de Zed, pero exigía preparar
un binario y ajustes de desarrollo. Los prototipos locales demostraron
traducción real y offline, aunque su ciclo separado tampoco era una experiencia
de extensión. El producto confirmado debe funcionar al instalarlo desde la
Gallery, sin terminal, checkout, servicio, cuenta, key, ruta o setting manual.

## Decision

Publicar una extension Rust/WASM delgada que usa el flujo de language server
administrado por Zed. En Linux `x86_64`, su primera activacion:

1. valida un lock compilado con fuentes, tamanos, SHA-256, presupuestos y
   conclusiones de licencia exactos;
2. descarga un release fijo del LSP/runner/avisos y tres recursos Mozilla
   `en -> es` fijos;
3. prepara un staging no ejecutable, decodifica Zstandard con Rust puro y
   valida cada identidad instalada;
4. promueve atómica e inmutablemente el paquete y devuelve `translator-lsp`
   sin selección o configuración de runtime;
5. ejecuta Bergamot mediante un proceso de un solo request, entorno limpio,
   cuatro hilos y limites de entrada, salida y tiempo.

El LSP resuelve exclusivamente el runner y modelos adyacentes verificados. No
reutiliza el manager ni los comandos de ciclo de vida del prototipo F012. Todo
el estado vive en el directorio de trabajo de la extension; Zed es responsable
de descarga, disable y uninstall. Otros sistemas/arquitecturas abortan antes de
crear estado o hacer red.

## Razon

Este diseno convierte la traduccion local probada en una extension ordinaria:
la complejidad de supply chain y modelos pertenece al release, no al usuario.
Los recursos Mozilla tienen identidad y atribucion publicas; el paquete activo
queda bajo 128 MiB y el runner no enlaza networking. La promocion atomica y un
ultimo paquete verificado cubren interrupciones y updates sin exponer una UI de
administracion.

## Consecuencias

- La primera version soporta solo Linux `x86_64`.
- El primer uso necesita red para cuatro descargas publicas; el uso posterior
  es offline.
- El release del proyecto debe existir antes de que la extension compilada
  pueda completar una instalacion limpia.
- La aceptacion 3/3 desde Gallery depende de merge upstream y no puede
  sustituirse por una dev extension.
- ADR 0007 retira las superficies de compatibilidad sin consumidor antes del
  primer tag; el paquete publicado conserva una única ruta local.

## Criterio de revision

Revisar si Zed cambia su API de descarga/almacenamiento/desinstalacion, si los
artefactos o licencias upstream cambian, si los presupuestos reales dejan de
cumplirse o antes de agregar otra plataforma/modelo/idioma.
