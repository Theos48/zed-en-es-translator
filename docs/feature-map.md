# Mapa de features

Este mapa es backlog estrategico para preparar futuros ciclos de Spec Kit. No
es la especificacion operativa de la feature activa; cuando una feature se
formaliza, su fuente de verdad pasa a `specs/<feature>/`.

## Uso con Spec Kit

Cada entrada F### debe conservar suficiente detalle para iniciar `specify` sin
volver a redescubrir alcance, restricciones y criterios. Al promover una entrada
a feature formal:

1. Copiar su objetivo, criterios y restricciones relevantes al prompt de
   `speckit-specify`.
2. Separar lo que pertenece a la feature actual de lo que queda para ciclos
   futuros.
3. Mantener en `docs/feature-map.md` el backlog estrategico, ajustando solo el
   estado o aprendizaje que sirva para futuras features.
4. Registrar en `docs/decisions.md` o un ADR cualquier cambio estable de
   arquitectura, seguridad, tecnologia o proceso.

Estado actual:

- Completado formal: F001, F002 parcial, F003 y F008 parcial mediante
  `specs/001-translation-core-contract/`; F005 mediante
  `specs/002-mcp-server/`; F006 mediante `specs/003-zed-wrapper/` tras merge
  en `main`; F007 mediante `specs/004-zed-ux-flow/`; F004 mediante
  `specs/005-real-provider-config/`.
- Completado formal: F010 mediante
  `specs/006-direct-zed-translation/`, incluidas tres validaciones manuales en
  Zed.
- Activo formal: ninguno; F010 esta cerrada.
- Siguiente candidata: F011.

Prioridad actual: configurar y validar proveedores reales mediante F011 antes de
promover F009. No conviene publicar una extension cuyo adaptador de proveedor
existe, pero que aun no tiene evidencia de traduccion extremo a extremo con un
proveedor local/offline real y otro remoto/online real.

Regla de direccion desde F006: las features que toquen Zed deben ser
extension-first. El Agent Panel puede usarse para validar integracion o cubrir
una limitacion concreta de la API, pero no debe ser el destino de producto ni
una deuda acumulada para migrar al final.

## F001: contrato de traduccion

Objetivo: definir una interfaz independiente de Zed y del proveedor.

Entradas:

- texto fuente;
- idioma origen fijo: ingles;
- idioma destino fijo: espanol;
- opciones de tono;
- preservacion de formato obligatoria en MVP;
- tipo de entrada: texto, Markdown o codigo.

Salidas:

- texto traducido;
- error claro con codigo normalizado cuando algo impida traducir.

Restricciones:

- sin metadata en la salida normal del MVP;
- metadata y segmentos protegidos solo pueden existir internamente o para
  pruebas.

## F002: preservacion de formato

Objetivo: evitar traducciones destructivas en Markdown y codigo.

Criterios:

- no traducir contenido dentro de bloques de codigo;
- mantener listas, headings y enlaces;
- traducir texto visible cuando sea seguro;
- si no hay confianza para distinguir comentario/docstring de codigo, preservar
  sin traducir;
- archivo completo de codigo queda fuera del primer ciclo Spec Kit.

## F003: proveedor mock

Objetivo: permitir TDD sin red ni API keys.

Criterios:

- respuestas deterministas;
- simulacion de errores;
- simulacion de latencia opcional.

## F004: proveedor real configurable

Objetivo: conectar el contrato a un backend real sin acoplar el core.

Estado actual: feature formal completada en
`specs/005-real-provider-config/`. Implementa un proveedor local/self-hosted
compatible con LibreTranslate como primer camino real, mantiene mock/offline
como default, modela remoto como default-deny con confirmacion por solicitud y
mantiene la configuracion fuera del texto traducido. Esta feature implemento el
adaptador y sus fronteras de configuracion; sus pruebas usaron servicios
loopback simulados y no desplego ni dejo configurada una instancia real para el
uso cotidiano.

Criterios:

- proveedor elegido de forma explicita y configurable;
- secretos fuera del repositorio y sin valores reales en ejemplos versionados;
- errores normalizados compatibles con el contrato vigente;
- remoto deshabilitado por defecto hasta que exista consentimiento verificable;
- evaluacion de privacidad antes de habilitar cualquier backend externo;
- no enviar contexto local innecesario al proveedor.

## F005: servidor MCP

Objetivo: exponer la traduccion como herramienta invocable desde Zed.

Criterios:

- tools basadas en los contratos de traduccion vigentes;
- validacion de parametros en la frontera MCP;
- salida limpia en caso exitoso y errores accionables en fallo;
- lectura de archivos delegada al core/CLI sin duplicar reglas operativas;
- mapeo de errores del core/CLI a resultados MCP compatibles con Zed.

## F006: fundacion de extension Zed

Objetivo: crear la base de extension Zed desde la que el producto pueda crecer
hacia acciones propias, sin depender de Agent Panel como destino final.

Estado actual: completada en `specs/003-zed-wrapper/`. La implementacion
valida el arranque de `translator-mcp` como context server local porque era la
capacidad documentada y verificable de Zed para esta etapa. Esa decision no
convierte Agent Panel en la UX objetivo.

Criterios:

- manifest `extension.toml`;
- build reproducible;
- logs utiles y redaccionados conforme a la constitucion;
- entorno agregado por el wrapper limitado por allowlist, con la limitacion de
  herencia del proceso Zed documentada en D064;
- arranque del servidor MCP con comando, argumentos y variables controladas por
  el wrapper.
- documentar limites reales de la API de Zed que impidan accion directa;
- dejar preparada la estructura para evolucionar hacia comandos, menus,
  botones o preview propios de la extension cuando sean viables.

## F007: flujo UX dentro de Zed

Objetivo: validar lectura dentro de Zed sin salir del editor, usando el puente
disponible en ese momento.

Criterios:

- flujo documentado;
- friccion baja;
- resultado facil de leer en Agent Panel;
- el buffer no se modifica automaticamente;
- entradas permitidas definidas por el contrato activo;
- validacion manual del flujo real de seleccion antes de ampliar alcance.

Estado actual: feature formal completada en `specs/004-zed-ux-flow/`.

Nota de producto: esta feature valida el camino Agent Panel como puente
intermedio sobre las tools MCP existentes. No representa la experiencia final
del producto; no debe repetirse como patron base para nuevas features de UX.
Ver D065, D071, D072 y F010.

## F008: privacidad y configuracion

Objetivo: que el usuario entienda y controle que texto se envia.

Criterios:

- proveedor y modo local/remoto visibles para el usuario;
- secretos fuera del repositorio;
- remoto default deny;
- controles de privacidad antes de cualquier llamada remota;
- entorno heredado minimo entre Zed, MCP y CLI cuando la plataforma lo permita;
  para el context server Zed actual, considerar la limitacion D064;
- pruebas negativas de privacidad proporcionales al proveedor habilitado.

## F009: empaquetado y publicacion

Objetivo: preparar el proyecto para distribucion.

Estado: futuro posterior a F011. La publicacion debe esperar a que exista una
experiencia directa de extension sin Agent y que los caminos local/offline y
remoto/online hayan sido configurados y validados con proveedores reales.

Criterios:

- licencia compatible con Zed;
- README final;
- checklist de publicacion;
- lockfiles y auditoria de dependencias antes de publicar.
- guia de uso centrada en la accion propia de la extension, no en Agent Panel.

## F010: flujo directo de extension Zed sin Agent

Objetivo: ofrecer la experiencia final de producto dentro de Zed sin requerir
que el usuario configure o use Agent Panel. Debe sentirse como una extension
nativa de traduccion para Zed, disenada como producto propio con control fuerte
para trabajo tecnico, privacidad y preservacion de formato.

Estado: completada en `specs/006-direct-zed-translation/`, incluidas tres
validaciones manuales en Zed. La integracion usa code action, execute command y
hover LSP; las limitaciones de clipboard/panel propio en API 0.7.0 y el canal
real de configuracion `binary.env` quedan registradas en D073-D075 y ADR 0004.

Criterios iniciales para `speckit-specify`:

- exponer una accion propia de la extension desde menu contextual, comando o
  boton, segun lo que permita la API vigente de Zed;
- no requerir Agent Panel, perfiles Agent, prompts manuales ni modelo
  intermediario para ejecutar la traduccion;
- aceptar texto seleccionado cuando Zed lo exponga de forma confiable;
- aceptar contenido permitido del documento abierto solo con las mismas
  validaciones de archivo, tamano, UTF-8, secretos y workspace ya existentes;
- mostrar preview legible dentro de Zed antes de cualquier accion destructiva;
- mantener no-mutacion automatica del buffer como default;
- permitir copiar, insertar o aplicar la traduccion solo por accion explicita
  del usuario, si la API de Zed permite esas salidas;
- reutilizar `translator-core` y la configuracion de proveedor existente;
- tratar `translator-mcp`/Agent Panel como compatibilidad o puente, no como
  superficie primaria del producto;
- conservar remoto default deny y confirmacion por solicitud antes de enviar
  texto fuera del equipo;
- documentar cualquier limitacion real de la API de Zed antes de recortar
  alcance.

## F011: configuracion operativa de proveedores reales

Objetivo: pasar de un adaptador configurable probado con dobles de prueba a dos
caminos de traduccion reales y verificables desde la extension directa de Zed:
uno local/offline y otro remoto/online.

Estado: backlog; siguiente candidata formal antes de F009/publicacion. Los
proveedores concretos se seleccionaran durante `speckit-clarify` y
`speckit-plan` despues de evaluar privacidad, licencia, mantenimiento, calidad
ingles-espanol y costo. El camino base no debe exigir una cuenta de pago.

Criterios iniciales para `speckit-specify`:

- configurar un proveedor local real, ejecutado con aislamiento y alcance de
  proyecto, que pueda traducir sin Internet despues de preparar sus artefactos;
- documentar inicio, parada, actualizacion, datos persistentes, verificacion y
  rollback del proveedor local sin instalar runtimes o servicios globales en
  Fedora;
- configurar un proveedor remoto real mediante HTTPS y host allowlisted, sin
  convertirlo en default ni enviar texto sin confirmacion por solicitud;
- mantener `MockProvider` como default determinista cuando no haya
  configuracion explicita, conforme a la constitucion;
- mantener secretos reales fuera del repositorio y documentar solo nombres de
  variables o referencias seguras;
- mostrar en Zed si la traduccion usara modo offline/local o remoto/online antes
  de ejecutar la solicitud;
- validar traduccion inglesa a espanola con contenido sintetico tanto por CLI
  como por el flujo directo de Zed para ambos proveedores reales;
- demostrar que no se modifican buffers o archivos y que logs, errores y
  evidencias no contienen texto fuente, traducciones, URLs sensibles, tokens o
  secretos;
- comprobar indisponibilidad, timeout, respuesta invalida, rechazo remoto sin
  confirmacion y bloqueo de secretos antes del contacto remoto;
- conservar limites, segmentacion, preservacion Markdown y errores
  normalizados existentes;
- registrar evidencia manual redaccionada contra los servicios reales; los
  stubs siguen siendo validos para automatizacion, pero no cierran por si solos
  esta feature;
- no incluir publicacion, proveedor de pago obligatorio, instalacion global en
  el host ni mutacion automatica del buffer.
