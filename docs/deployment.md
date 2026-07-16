# Despliegue y publicación

Este proyecto distingue tres operaciones diferentes:

1. **Dev extension local**: sirve para desarrollar la integración con Zed.
2. **Release del proyecto**: publica el archivo nativo exacto en GitHub.
3. **Extension Gallery**: distribuye la extensión desde el registro oficial.

Una prueba local no sustituye la aceptación del paquete público ni una
instalación limpia desde la Gallery.

## Estado actual

La implementación y el candidato están validados. El release público `v0.1.0`
contiene el archivo exacto de 5,548,286 bytes y SHA-256
`9cddf1ede9a19e2e5ad6cdf1c3c775d218cdc455fc27462c8922e6ffd19108d3`;
`make marketplace-release-check` confirma que tag, URL, versión, tamaño y hash
coinciden con el package lock. El PR
[zed-industries/extensions#6843](https://github.com/zed-industries/extensions/pull/6843)
ya pasa `package`, Danger y CLA; siguen pendientes el merge y la aceptación
interactiva 3/3 desde Gallery.

## 1. Probar como dev extension

Primero valida el código de la extensión con el contenedor del proyecto:

```bash
make zed-extension-build
make test-marketplace-contract
make test-marketplace-acquisition
```

Zed permite cargar una extensión sin publicarla:

1. abre la página de extensiones;
2. selecciona **Install Dev Extension** o ejecuta la acción
   `zed: install dev extension`;
3. selecciona el directorio `zed-extension/`, que contiene `extension.toml`;
4. abre `zed: open log` para revisar fallos de compilación o carga.

La documentación oficial de Zed indica que las extensiones Rust se compilan a
WebAssembly y que su flujo de dev extension requiere un Rust instalado mediante
`rustup` en el entorno donde se ejecuta Zed. Este repositorio, en cambio, no
instala Rust globalmente: sus builds soportados usan Docker. Si necesitas la
prueba interactiva, usa un entorno de desarrollo aislado o una excepción de
host revisada; no cambies el Makefile ni el package lock para esquivar esa
frontera.

Referencias oficiales:

- <https://zed.dev/docs/extensions/developing-extensions>
- <https://zed.dev/docs/extensions/installing-extensions>

### Qué puede validar una dev extension

- que Zed reconoce el manifest y el language server;
- que el WASM carga y muestra estados de adquisición;
- que la acción y los logs aparecen como se espera;
- que una release ya publicada se descarga y activa.

No demuestra por sí sola que el registro central, una instalación limpia o la
remoción de la Gallery funcionen.

## 2. Construir el paquete exacto

Ejecuta desde un checkout persistente y revisado:

```bash
make workspace-storage-check
make worktree-audit
make clean-preview
make clean
make test-marketplace-package
make test-marketplace-release-contents
make test-marketplace-offline
```

El archivo producido queda en:

```text
target/marketplace-package/
  en-es-translator-<version>-linux-x86_64.tar.gz
```

Contiene únicamente:

```text
bin/translator-lsp
bin/translator-embedded-runtime
LICENSES/THIRD_PARTY_NOTICES.md
LICENSES/MPL-2.0.txt
LICENSES/SOURCE.md
```

Los modelos no se incluyen en ese archivo. La extensión descarga por separado
los tres recursos públicos descritos en `ops/marketplace/package.lock.json` y
los verifica antes de activar el paquete.

`make test-marketplace-offline` prepara en `target/marketplace-real/` la forma
instalada completa, ejecuta 3 traducciones smoke y un corpus real de 20 casos
con la red deshabilitada durante inferencia.

## 3. Preparar una versión

Antes de etiquetar, sincroniza como mínimo:

- `zed-extension/extension.toml`;
- `zed-extension/Cargo.toml` y su lock;
- manifests de `translator-core` y `translator-lsp`;
- `ops/marketplace/package.lock.json`;
- nombre de paquete en scripts y tests;
- URL `releases/download/v<version>/...`;
- tamaños y SHA-256 exactos de LSP, runtime y archivo;
- licencias, avisos y evidencia de validación.

Busca cualquier identidad antigua:

```bash
rg -n '0\.1\.0|en-es-translator|Theos48' \
  --glob '!.cache/**' --glob '!Cargo.lock' --glob '!zed-extension/Cargo.lock'
```

Después ejecuta el gate completo:

```bash
make test-repository-boundary
make fmt
make clippy
make deny
make test
make test-marketplace-foundation
make test-marketplace-contract
make test-marketplace-acquisition
make test-marketplace-native-supply-chain
make test-marketplace-package
make test-marketplace-release-contents
make test-marketplace-offline
git diff --check
```

Revisa y fusiona el cambio antes de crear el tag. Nunca publiques el asset desde
un commit distinto al que se envía al registro.

## 4. Publicar el release de GitHub

El workflow `.github/workflows/marketplace-package.yml` se activa con tags
`v*.*.*`. Comprueba que:

- el tag sea `v<extension.toml version>`;
- `package.lock.json` use la misma versión;
- el nombre del asset sea
  `en-es-translator-<version>-linux-x86_64.tar.gz`;
- el tag apunte al commit ya revisado y enviado.

Ejemplo, solo después de aprobar el candidato:

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

GitHub Actions reconstruye el runner, verifica supply chain, ensambla dos veces
el archivo determinista, genera su `.sha256` y crea el release sin reemplazar
un asset anterior.

Cuando termine:

```bash
make marketplace-release-check
```

Este target vuelve a descargar el asset público y compara tag, URL, layout,
permisos, tamaños y hashes con el lock local.

Si el candidato publicado está mal, no reemplaces silenciosamente el archivo
de la misma versión. Corrige, incrementa la versión de parche, regenera todas
las identidades y publica un nuevo release.

## 5. Publicar en la Extension Gallery

Zed publica extensiones mediante PRs al repositorio público
`zed-industries/extensions`. Para esta estructura monorepo, el cambio upstream
debe:

1. agregar este repositorio como submódulo HTTPS en
   `extensions/en-es-translator`;
2. añadir a `extensions.toml`:

   ```toml
   [en-es-translator]
   submodule = "extensions/en-es-translator"
   path = "zed-extension"
   version = "0.1.0"
   ```

3. ejecutar `pnpm sort-extensions` en el repositorio upstream;
4. enviar el commit público exacto, que debe pertenecer a una rama;
5. asegurar que `zed-extension/LICENSE` está presente y aceptado;
6. describir en el PR la prueba real sobre Linux `x86_64` y enlazar la evidencia
   del release.

Tras el merge, Zed empaqueta y publica la extensión en su registro. Consulta el
procedimiento oficial vigente antes de abrir el PR:

- <https://zed.dev/docs/extensions/developing-extensions#publishing-your-extension>
- <https://github.com/zed-industries/extensions>

## 6. Aceptación limpia después del merge

En un perfil de Zed sin checkout, binario configurado ni dev extension:

1. abre la Gallery (`Ctrl+Shift+X` en Linux/Windows);
2. instala **English to Spanish Translator**;
3. abre un fixture público Markdown o texto plano;
4. ejecuta **Translate English to Spanish**;
5. observa checking/downloading y después el preview en hover;
6. deshabilita la red, reinicia Zed y repite los casos offline;
7. confirma que los bytes del archivo fuente no cambiaron;
8. deshabilita la extensión y confirma que no inicia otro proceso;
9. desinstala desde Zed y confirma que su work dir desaparece.

Registra solamente versiones, plataforma, IDs de fixtures públicos, resultado y
métricas de recursos. No guardes texto privado ni rutas sensibles.

## Desplegar un fork o producto derivado

Antes de publicar una variante:

1. elige un ID de extensión nuevo y permanente;
2. cambia nombre, autores, repositorio y versión en `extension.toml`;
3. cambia package ID, URL, tests y allowlist de repositorio;
4. publica el release exacto en el repositorio del fork;
5. prueba la dev extension contra esa URL pública;
6. usa el nuevo ID y submódulo en el PR al registro de Zed.

Para otro idioma o plataforma no basta con cambiar una etiqueta. Se requieren
modelos con licencia, URLs/tamaños/hashes exactos, runner compatible, límites,
fixtures reales, package ID nuevo y evidencia completa de adquisición, offline,
recursos y remoción.

## Estado instalado y rollback

Zed es propietario de `installed/` y `work/en-es-translator/`. Dentro del work
dir, la extensión mantiene `install.lock`, `state.json`, `staging/` y como
máximo el paquete activo y el anterior verificado. Un candidato fallido no
reemplaza el activo.

No repares manualmente el work dir como procedimiento de producto. Un usuario
reintenta desde Zed; disable y uninstall usan el lifecycle normal del editor.
Para un release defectuoso, el rollback de publicación es una nueva versión que
apunta a un commit y paquete verificados, no mutar el asset existente.

## Problemas frecuentes

| Síntoma | Causa probable | Acción |
|---|---|---|
| Dev extension carga pero no prepara | El tag/asset del package lock todavía no existe. | Publica primero el release exacto del fork/proyecto. |
| `marketplace-release-check` falla | Tag, URL, versión, asset o hash no coincide. | Compara manifest, lock y release; no edites hashes a ciegas. |
| Plataforma no soportada | La primera release solo acepta Linux `x86_64`. | No fuerces la descarga; implementa un paquete de plataforma nuevo. |
| Zed muestra fallo de descarga | Red, storage o identidad inválida. | Reintenta desde Zed y revisa `zed: open log` sin exponer contenido. |
| Funciona online pero falla offline | El paquete activo no está completo o una ruta intentó adquirir durante inferencia. | Ejecuta `make test-marketplace-offline`. |
| El build consume demasiado disco | Targets Rust/C++ acumulados. | Usa `make clean-preview` y después `make clean`. |
