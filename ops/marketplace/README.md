# Marketplace package maintenance

Este directorio contiene las identidades y obligaciones exactas del paquete
automatico para Linux `x86_64`. No es una guia de instalacion para usuarios:
la ruta publica es instalar desde la Gallery de Zed y usar la accion de
traduccion.

## Contenido y adquisicion

- `package.lock.json` es el contrato compilado dentro de la extension.
- El release del proyecto aporta `translator-lsp`, el runner nativo y los
  avisos/licencias.
- Los tres recursos `en -> es` se descargan desde URLs publicas fijas de
  Mozilla y se verifican comprimidos y ya decodificados.
- Un paquete solo se vuelve activo despues de validar allowlist, tamanos,
  SHA-256, modos y `installed.json` completo.
- Readiness y traduccion reutilizan exclusivamente el paquete activo; no
  contienen una ruta de descarga o reparacion.

La extension conserva como maximo el paquete activo, el anterior verificado y
un staging no ejecutable. Un fallo o cierre deja el paquete previo disponible y
el siguiente uso reintenta sin limpieza manual.

## Propiedad y eliminacion

Todo el estado de producto se crea relativo al directorio de trabajo que Zed
asigna a `en-es-translator`: `packages/`, `staging/`, `state.json` e
`install.lock`. No se escriben rutas XDG independientes, `/usr`, `/opt`,
servicios, contenedores ni configuracion del host.

El contrato esta anclado al commit de Zed registrado en
`zed-source.lock.json`: al desinstalar, Zed descarga la extension, elimina su
directorio instalado y elimina `work/<extension-id>` con reintentos para cubrir
la carrera de apagado del proceso. Deshabilitar y desinstalar se validan
interactivamente antes de publicar; el usuario nunca ejecuta un comando de
limpieza.

## Gates de mantenimiento

Los mantenedores usan los targets del `Makefile` desde este checkout. El gate
completo construye el runner y paquete fijados, valida supply chain, descarga
los recursos publicos, ejecuta el corpus con red deshabilitada y comprueba
formato, lint, dependencias y pruebas. Esos comandos no son prerrequisitos del
usuario de la Gallery.

Las instrucciones ejecutables viven en:

- [`CONTRIBUTING.md`](../../CONTRIBUTING.md), para preparar el entorno,
  modificar componentes y elegir gates;
- [`docs/deployment.md`](../../docs/deployment.md), para construir el candidato,
  publicar el release, probar una dev extension o desplegar un fork;
- [`specs/009-zed-marketplace-install/quickstart.md`](../../specs/009-zed-marketplace-install/quickstart.md),
  para la evidencia de aceptación exacta de la feature.
