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
  `specs/002-mcp-server/`.
- Activo formal: F006 mediante `specs/003-zed-wrapper/`.
- Futuro: F004, F007 y F009.

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

## F006: wrapper Zed

Objetivo: instalar y arrancar el servidor desde una extension de Zed.

Criterios:

- manifest `extension.toml`;
- build reproducible;
- logs utiles y redaccionados conforme a la constitucion;
- entorno minimo por allowlist;
- arranque del servidor MCP con comando, argumentos y variables controladas.

## F007: flujo UX dentro de Zed

Objetivo: que el usuario pueda completar la traduccion sin salir del editor.

Criterios:

- flujo documentado;
- friccion baja;
- resultado facil de leer en Agent Panel;
- el buffer no se modifica automaticamente;
- entradas permitidas definidas por el contrato activo;
- validacion manual del flujo real de seleccion antes de ampliar alcance.

## F008: privacidad y configuracion

Objetivo: que el usuario entienda y controle que texto se envia.

Criterios:

- proveedor y modo local/remoto visibles para el usuario;
- secretos fuera del repositorio;
- remoto default deny;
- controles de privacidad antes de cualquier llamada remota;
- entorno heredado minimo entre Zed, MCP y CLI;
- pruebas negativas de privacidad proporcionales al proveedor habilitado.

## F009: empaquetado y publicacion

Objetivo: preparar el proyecto para distribucion.

Criterios:

- licencia compatible con Zed;
- README final;
- checklist de publicacion;
- lockfiles y auditoria de dependencias antes de publicar.
