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

- En progreso formal: F001, F002 parcial, F003 y F008 parcial mediante
  `specs/001-translation-core-contract/`.
- Futuro: F004-F007 y F009.

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

- no versionar secretos;
- configuracion clara;
- errores normalizados;
- proveedor remoto deshabilitado por defecto;
- confirmacion por cada traduccion remota;
- evaluacion de privacidad antes de habilitar proveedores gratuitos/remotos;
- no enviar `file_path` a proveedores remotos.

## F005: servidor MCP

Objetivo: exponer la traduccion como herramienta invocable desde Zed.

Criterios:

- tools `translate_text` y `translate_file`;
- validacion de parametros;
- salida limpia en caso exitoso;
- errores claros cuando algo impida traducir;
- `translate_file` workspace-only con canonicalizacion;
- rechazo de traversal, symlinks fuera del workspace, no UTF-8 y contenido
  binario;
- mapear errores del CLI Rust a resultados MCP con `isError: true`.

## F006: wrapper Zed

Objetivo: instalar y arrancar el servidor desde una extension de Zed.

Criterios:

- manifest `extension.toml`;
- build reproducible;
- logs utiles sin contenido fuente, traduccion, secretos, headers, tokens ni
  rutas sensibles;
- entorno minimo por allowlist.

## F007: flujo UX dentro de Zed

Objetivo: que el usuario pueda completar la traduccion sin salir del editor.

Criterios:

- flujo documentado;
- friccion baja;
- resultado facil de leer en Agent Panel;
- el buffer no se modifica automaticamente;
- la seleccion directa desde Zed se validara manualmente;
- el flujo base acepta texto pegado o path permitido.

## F008: privacidad y configuracion

Objetivo: que el usuario entienda y controle que texto se envia.

Criterios:

- proveedor explicito;
- secretos fuera del repositorio;
- mensajes claros sobre red/local;
- remoto default deny;
- deteccion basica de secretos antes de llamadas remotas;
- no heredar todo el entorno de Zed hacia MCP/CLI;
- pruebas negativas de privacidad.

## F009: empaquetado y publicacion

Objetivo: preparar el proyecto para distribucion.

Criterios:

- licencia compatible con Zed;
- README final;
- checklist de publicacion;
- lockfiles y auditoria de dependencias antes de publicar.
