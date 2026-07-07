<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan
at specs/005-real-provider-config/plan.md
<!-- SPECKIT END -->

## Flujo documental del proyecto

Este proyecto usa `docs/` como planeacion estrategica para alimentar ciclos de
Spec Kit, y `specs/<feature>/` como fuente operativa de la feature activa.

Jerarquia:

1. `.specify/memory/constitution.md`: principios y reglas no negociables.
2. `specs/<feature>/`: requisitos, plan, contratos, tareas y quickstart de la
   feature activa.
3. `docs/feature-map.md`: backlog detallado de features futuras.
4. `docs/decisions.md` y `docs/adr/`: decisiones estables y cambios de
   arquitectura/proceso.
5. `docs/research/`, `docs/product.md`, `docs/diagrams.md`, `docs/PLAN.md`:
   contexto estrategico e investigacion.

Al retomar una sesion:

1. Leer la constitucion.
2. Leer `docs/PLAN.md` para ubicar la jerarquia y roadmap.
3. Leer `docs/feature-map.md` si se va a preparar una nueva feature.
4. Leer `specs/<feature>/` si se va a implementar o ajustar la feature activa.

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

- Si solo avanza la implementacion esperada, actualizar `specs/<feature>/`.
- Si cambia alcance, limites, seguridad, tecnologia, arquitectura o proceso,
  actualizar tambien `docs/decisions.md` o un ADR.
- Si esa decision estable tiene un resumen narrativo en `README.md` o
  `docs/PLAN.md` (por ejemplo el estado de una feature o un limite ya
  cerrado), actualizar tambien ese resumen en el mismo cambio. Un ADR o una
  fila en `docs/decisions.md` no basta si README/PLAN siguen describiendo el
  estado anterior.
- No duplicar en `docs/` el detalle operativo que ya vive en `specs/<feature>/`.
- No borrar detalle del mapa de features: sirve como entrada para futuros
  ciclos `specify -> clarify -> plan -> tasks -> implement`.
