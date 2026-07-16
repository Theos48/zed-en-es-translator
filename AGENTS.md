<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan
at specs/010-repository-convergence/plan.md
<!-- SPECKIT END -->

## Flujo documental del proyecto

Este proyecto usa `docs/` para dirección estratégica y decisiones estables, y
`specs/<feature>/` como fuente operativa de las features activas.

Jerarquia:

1. `.specify/memory/constitution.md`: principios y reglas no negociables.
2. `specs/010-repository-convergence/`: requisitos, plan, contratos y tareas
   de la limpieza activa.
3. `specs/009-zed-marketplace-install/`: contratos y gates de la release que
   sigue pendiente.
4. `docs/decisions.md` y `docs/adr/`: decisiones aceptadas o supersedidas.
5. `docs/PLAN.md`, `docs/feature-map.md` y `docs/diagrams.md`: estado,
   roadmap futuro y arquitectura vigente.

Al retomar una sesion:

1. Leer la constitucion.
2. Leer `specs/010-repository-convergence/` mientras la limpieza siga activa.
3. Leer `specs/009-zed-marketplace-install/` para trabajo de paquete/release.
4. Leer `docs/PLAN.md` para estado y secuencia actual.
5. Leer `docs/feature-map.md` solo al preparar una nueva feature.

## Regla de gates Spec Kit

No cerrar una fase de Spec Kit sin reportar explicitamente el estado de los
gates relacionados:

- `speckit-specify`: crear/actualizar `spec.md` y checklist de calidad de
  requisitos.
- `speckit-clarify`: ejecutar el prerequisito del comando y resolver o declarar
  que no hay ambiguedades criticas.
- `speckit-checklist`: generar checklist de requisitos cuando el dominio tenga
  riesgos especificos o el usuario lo pida.
- `speckit-plan`: ejecutar `setup-plan`, generar research/data-model/contracts/
  quickstart y actualizar el contexto de agente.
- `speckit-tasks`: no aplicar antes de plan aprobado; cuando aplique, generar
  `tasks.md`.
- `speckit-analyze`: aplicar despues de `tasks.md` y antes de implementar; si
  falta `tasks.md`, ejecutar el prerequisito y reportar el bloqueo.
- `speckit-implement`: aplicar solo despues de tasks/analyze cuando corresponda.
- `speckit-converge`: aplicar despues de una pasada de implementacion; si falta
  `tasks.md` o no hubo implementacion, ejecutar el prerequisito y reportar el
  bloqueo.

Si un gate no aplica por la fase actual, no omitirlo en silencio: indicar
`no aplicable` o `bloqueado`, con el comando/prerrequisito usado para decidirlo.

## Regla para Rust

Cuando se escriba, revise o refactorice codigo Rust, o cuando se ejecute
`/implement` para tareas que toquen Rust, usar tambien la skill
`rust-best-practices`.

Aplicacion:

- Leer `.agents/skills/rust-best-practices/SKILL.md` antes de cambiar codigo
  Rust.
- Leer las referencias relevantes de la skill en el mismo turno cuando la
  decision involucre ownership, errores, performance, traits, tests o
  documentacion publica.
- Mantener la politica de host limpio: Rust se ejecuta mediante el `Makefile`
  y el contenedor Docker del proyecto, no con `cargo`/`rustc` instalados en el
  host para este repo.
- Si la skill no esta disponible en una sesion futura, avisar antes de
  implementar cambios Rust.

Regla de sincronizacion:

- Si solo avanza la implementación esperada, actualizar la feature activa.
- Si cambia alcance, limites, seguridad, tecnologia, arquitectura o proceso,
  actualizar tambien `docs/decisions.md` o un ADR.
- Si esa decision estable tiene un resumen narrativo en `README.md` o
  `docs/PLAN.md` (por ejemplo el estado de una feature o un limite ya
  cerrado), actualizar tambien ese resumen en el mismo cambio. Un ADR o una
  fila en `docs/decisions.md` no basta si README/PLAN siguen describiendo el
  estado anterior.
- No duplicar en `docs/` el detalle operativo que ya vive en las features
  activas.
- Los ciclos 001-007 retirados se consultan en Git; no restaurarlos como
  archivo dentro del working tree. Sus decisiones estables viven en los ADRs.
- No borrar el detalle futuro del mapa de features: sirve como entrada para
  ciclos `specify -> clarify -> plan -> tasks -> analyze -> implement -> converge`.

## Higiene de worktrees y almacenamiento temporal

- No crear worktrees, clones ni salidas de compilacion del proyecto bajo
  `/tmp`, `/dev/shm`, `/run/user/*` u otro filesystem `tmpfs`/`ramfs`.
- Crear worktrees temporales bajo
  `~/dev/.worktrees/zed-en-es-translator/<nombre>` para que sus artefactos
  permanezcan en disco y no ocupen RAM/swap.
- Antes de compilar, ejecutar `make workspace-storage-check`. Los targets Rust
  lo aplican automaticamente mediante `rust-image`.
- Auditar todos los checkouts registrados con `make worktree-audit`.
- Retirar un worktree con `git worktree remove <ruta>` y luego
  `git worktree prune`; no borrar su directorio directamente.
- No forzar la retirada de un worktree sucio: revisar y preservar primero sus
  cambios y comprobar que ningun proceso o montaje lo este usando.
