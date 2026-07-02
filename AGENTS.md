<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan
at specs/001-translation-core-contract/plan.md
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

Regla de sincronizacion:

- Si solo avanza la implementacion esperada, actualizar `specs/<feature>/`.
- Si cambia alcance, limites, seguridad, tecnologia, arquitectura o proceso,
  actualizar tambien `docs/decisions.md` o un ADR.
- No duplicar en `docs/` el detalle operativo que ya vive en `specs/<feature>/`.
- No borrar detalle del mapa de features: sirve como entrada para futuros
  ciclos `specify -> clarify -> plan -> tasks -> implement`.
