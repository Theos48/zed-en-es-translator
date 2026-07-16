# Plan de desarrollo

## Fuentes de verdad

1. `.specify/memory/constitution.md` gobierna los principios no negociables.
2. `specs/010-repository-convergence/` registra la limpieza completada y sus
   gates de convergencia.
3. `specs/009-zed-marketplace-install/` conserva los requisitos, contratos y
   gates de la publicación en Gallery pendiente.
4. `docs/decisions.md` y `docs/adr/` registran decisiones estables e historia
   explícitamente aceptada o supersedida.
5. `docs/feature-map.md` mantiene el backlog futuro; `docs/diagrams.md` muestra
   la arquitectura vigente.
6. `CONTRIBUTING.md` y `docs/deployment.md` son las guías operativas para
   desarrollar, modificar, probar y publicar sin duplicar los contratos.

El detalle operativo no se duplica aquí. Si cambia alcance, seguridad,
arquitectura, tecnología o proceso, se actualizan la feature afectada y una
decisión estable. Si solo avanza la implementación prevista, se actualiza la
feature activa.

## Objetivo vigente

Publicar una extensión ordinaria de Zed que traduzca inglés → español con este
único recorrido:

```text
Gallery -> extensión -> paquete verificado -> LSP -> core -> runtime embebido
```

La preparación inicial descarga entradas públicas de identidad fija dentro del
directorio de trabajo de la extensión. La traducción posterior es local y
offline. El resultado es un preview de solo lectura; el buffer y el archivo
fuente permanecen sin cambios.

La primera release está limitada a Linux `x86_64`, Markdown y texto plano. No
hay selección de motor, endpoint, credencial, ejecutable o segundo ciclo de
vida fuera de Zed.

La cobertura final del producto incluye todas las combinaciones de sistema
operativo y arquitectura de 64 bits soportadas oficialmente por Zed:

| Sistema | Arquitecturas objetivo |
|---|---|
| Linux | `x86_64`, `aarch64` |
| macOS | `x86_64`, `aarch64` |
| Windows | `x86_64`, `aarch64` |

La release inicial es la base validada, no el límite final del producto. La
expansión conserva el mismo flujo, UX, motor local y garantías; solo agrega el
paquete nativo verificado correspondiente a cada plataforma.

## Estado actual

La feature 009 implementó y validó localmente:

- adquisición con fuentes, tamaños y SHA-256 fijos;
- staging no ejecutable, promoción atómica y paquete anterior verificado;
- traducción real offline mediante el runtime nativo acotado;
- preview directo y no-mutación;
- fallos recuperables, concurrencia y plataformas no soportadas;
- límites de paquete, memoria, hilos, tiempo, licencias y redacción.

La feature 010 eliminó superficies sin consumidor, migró sus invariantes vivos
a los gates retenidos y redujo la documentación al producto publicado. Las
identidades de binario y archivo registradas por 009 ya fueron regeneradas y
validadas antes del tag.

El release público `v0.1.0` ya fue generado por el workflow reproducible desde
`fb2d76c`, y su archivo de 5,548,286 bytes coincide en tamaño y SHA-256 con el
package lock. El check contra el tag, la URL y el asset públicos también pasa.

## Secuencia hasta `v0.1.0`

### 1. Convergencia del repositorio — completada

- conservar solo extensión, `translator-lsp`, `translator-core`, runtime
  embebido, supply chain y validación marketplace;
- retirar entradas, dependencias, automatización, pruebas e instrucciones sin
  consumidor en la Gallery;
- mantener Mock únicamente como doble inyectado de pruebas;
- conservar Git como archivo completo y los ADRs como resumen de decisiones;
- hacer pasar la frontera negativa del repositorio y todos los links locales.

### 2. Regeneración del candidato exacto — completada

- limpiar salidas generadas con la allowlist normal;
- reconstruir LSP, runtime y archivo reproducible;
- actualizar tamaños, SHA-256 y evidencia de 009;
- repetir tres traducciones reales con el paquete de forma marketplace;
- ejecutar formato, Clippy, dependencias, adquisición, package, offline,
  privacidad, licencias, recursos y remoción.

### 3. Release del proyecto — publicada; aceptación interactiva pendiente

- convergencia revisada y fusionada;
- tag y asset exactos publicados como `v0.1.0`;
- check público de release completado;
- completar la aceptación interactiva en Zed con ese asset exacto.

### 4. Publicación en Gallery

- enviar el cambio al registro oficial de extensiones de Zed;
- esperar sus checks y merge;
- realizar 3/3 instalaciones limpias desde la Gallery;
- verificar preparación visible, traducción offline, no-mutación, disable y
  uninstall sin pasos externos.

La feature 009 permanece abierta hasta que estos gates externos pasen.

## Roadmap posterior

El trabajo posterior a `v0.1.0` se abre como features independientes:

1. completar F013 hasta alcanzar paridad con todas las plataformas oficiales
   de Zed de 64 bits, cada una con paquete y evidencia reproducible propios;
2. mejoras de preview y estado usando únicamente APIs estables de Zed;
3. nuevos pares de idiomas solo con recursos exactos, presupuesto y revisión
   de distribución propios.

F013 se implementa una combinación a la vez para mantener verificable cada
entrega, pero no se considera terminada hasta completar la matriz de Linux,
macOS y Windows indicada en el objetivo vigente. No requiere proveedores,
interfaces ni ciclos de vida nuevos.

Cualquier escritura, inserción o reemplazo de contenido requiere primero una
enmienda constitucional. No es una extensión implícita del roadmap.

## Flujo Spec Kit

Para cada nueva iteración:

1. seleccionar una entrada de `docs/feature-map.md`;
2. ejecutar `specify` y el checklist de calidad;
3. ejecutar `clarify` y registrar si no hay ambigüedades críticas;
4. ejecutar `plan`, incluidos research, contratos, quickstart y Constitution
   Check;
5. generar `tasks` solo después del plan aprobado;
6. ejecutar `analyze` antes de implementar;
7. implementar con pruebas o contratos negativos primero;
8. ejecutar `converge` y reportar todos los gates.

Los worktrees temporales viven bajo
`~/dev/.worktrees/zed-en-es-translator/`. Antes de compilar se usa la guarda de
almacenamiento del proyecto y la retirada se realiza con `git worktree remove`
y `git worktree prune`, después de preservar cualquier cambio.
