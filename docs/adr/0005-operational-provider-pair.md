# ADR 0005: proveedor local operativo y remoto opcional

## Estado

Superseded by [ADR 0007](./0007-repository-convergence.md). El experimento
operativo permanece como historia en Git; ninguna de sus superficies forma
parte de la arquitectura o de las instrucciones vigentes.

## Contexto

F004/feature 005 implemento un adaptador compatible con LibreTranslate y los
controles genericos de provider, pero su evidencia automatizada usa stubs. F010
ya expone la traduccion mediante CLI y un flujo directo LSP de Zed. Antes de
publicar, el proyecto necesita demostrar un camino local que funcione sin red
despues de prepararse. El remoto conserva consentimiento por solicitud,
secretos fuera del repositorio y destino exacto, pero no debe obligar a crear
una cuenta o gestionar una API key para usar o cerrar la feature.

La seleccion tambien debe resolver operacion cotidiana: versionado, integridad,
persistencia, readiness real, actualizacion, rollback, recursos, privacidad y
evidencia sin contenido.

## Decision

### Proveedor local

Usar LibreTranslate 1.9.6 mediante la imagen CPU oficial fijada al digest
multi-arquitectura
`sha256:1de2d7056bb8ad607a412f4563d9abe324ff632b43b5be9428bcc8e213aebb32`.

El proyecto adaptara Compose con:

- LibreTranslate conectado solo a una red interna, sin puerto publicado;
- relay HTTP minimo del mismo proyecto, publicado exclusivamente en
  `127.0.0.1:5000`, con destino interno fijo, limite de 128 KiB para la
  envolvente HTTP (el core conserva 20 KiB semanticos), respuesta de 40 KiB,
  sin logs, filesystem de solo lectura, capacidades eliminadas y sin proxy
  configurable;
- idiomas `en,es`, UI web y traduccion de archivos deshabilitadas;
- red interna para operacion normal y `pull_policy: never`;
- almacenamiento Docker nombrado y namespaced por proyecto;
- slots candidate/current/previous;
- readiness por `/health` mas traduccion sintetica publica;
- 4 CPU, 4 GiB RAM, 4 GiB libre y readiness preparada <=120 segundos como
  presupuestos verificables;
- preparacion/update online separados de start/verify/rollback offline;
- limpieza destructiva explicita, nunca como parte de `make clean`.

Los modelos Argos se adquieren localmente para el usuario y se verifican contra
hashes observados por el proyecto. No se redistribuyen ni se hornean en una
imagen mientras la licencia del paquete `en-es` siga sin declarar en upstream.

### Adaptador remoto opcional

Conservar Azure AI Translator Text v3 como integracion avanzada opt-in con un
recurso global single-service en F0:

```text
https://api.cognitive.microsofttranslator.com/translate
```

El adaptador construye internamente host, ruta, `api-version=3.0`, `from=en` y
`to=es`. `TRANSLATOR_PROVIDER=azure_translator` requiere
`TRANSLATOR_PROVIDER_API_KEY_ENV` y
`TRANSLATOR_ALLOW_REMOTE_PROVIDER=true`; `TRANSLATOR_PROVIDER_URL` debe estar
ausente. El valor real de la key vive solo en la variable heredada nombrada.

Se deshabilitan redirects, proxy heredado y retries. Cada solicitud requiere
confirmacion nueva, seguida del gate de secretos, antes de contacto. Se
conservan los limites y `ErrorCode` existentes.

Si alguien decide usar la integracion opcional, la documentacion debe explicar
que una cuenta Azure puede pedir telefono y
tarjeta, y puede requerir conversion de la cuenta a pay-as-you-go tras el
periodo introductorio, aunque el recurso Translator permanezca explicitamente
en F0. El endpoint global no garantiza residencia geografica y la pagina
vigente de privacidad documenta no persistencia, pero no promete explicitamente
exclusion de entrenamiento; el proyecto no afirmara esas garantias. Si cambian
F0, endpoint, privacidad, retencion o terminos, el remoto se bloquea hasta
nueva revision; mock/local siguen disponibles. F011 no exige cuenta, key ni
contacto real con Azure.

## Razon

- LibreTranslate encaja con el adaptador existente, es self-hosted y permite
  validar traduccion offline sin instalar Python o un servicio global.
- Fijar imagen y separar preparacion/runtime permite probar supply chain y
  ausencia de egress.
- Separar el relay loopback de LibreTranslate conserva acceso desde el host sin
  abrir una ruta de salida en el contenedor que procesa la traduccion.
- El adaptador Azure ya implementado conserva un endpoint global exacto y
  controles utiles, pero su cuenta/key lo hacen inadecuado como experiencia
  base obligatoria.
- Reutilizar las cuatro variables D075 evita otra superficie de secretos en
  Zed.
- Mantener el mock como default y exigir confirmacion fresca conserva la
  constitucion.

## Consecuencias

- `translator-core` agregara un adaptador Azure, sin crate nuevo.
- El repositorio agrega assets Compose/lock, un relay Python ejecutado dentro
  de la misma imagen fijada y comandos Make para el ciclo de vida local, pero
  no un runtime instalado en Fedora.
- Los tests automaticos seguiran usando dobles controlados; dos pruebas locales
  reales, ciclo de vida/recuperacion/limpieza y revision redactada son
  obligatorios para cerrar F011. Azure real no lo es.
- La operacion local consume descargas y almacenamiento apreciables y conserva
  un slot previo para rollback.
- El usuario que opte por remoto debe gestionar su cuenta/key fuera del
  repositorio y aceptar el limite de privacidad de enviar contenido confirmado
  a Microsoft; el usuario normal no necesita hacerlo.
- F009 no puede redistribuir el modelo actual hasta resolver su licencia o
  seleccionar otra estrategia revisada.
- Un proveedor local embebido/no-Docker queda diferido a F012 porque requiere
  evaluar licencia/procedencia, runtime, tamano, recursos, actualizacion y
  empaquetado.

## Criterio de revision

Revisar esta decision si cambia cualquiera de estos elementos:

- release/digest o mantenimiento de LibreTranslate;
- licencia, origen o integridad de modelos Argos;
- endpoint/API version/F0/privacidad/retencion/terminos de Azure Translator;
- API de Zed para secretos o configuracion segura;
- evidencia real muestra que los presupuestos o la calidad ingles-espanol no
  son aceptables;
- Docker ofrece una forma equivalente de publicar loopback desde una red
  interna sin el relay acotado;
- F009 decide empaquetar o redistribuir modelos.
