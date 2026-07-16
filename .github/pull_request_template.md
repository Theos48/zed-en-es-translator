## Resumen

<!-- Resume el cambio y el resultado esperado. -->

## Alcance

- [ ] Mantiene el scope de la feature activa o actualiza `specs/<feature>/`.
- [ ] Conserva el unico flujo de producto: Zed Gallery -> extension -> LSP -> core -> runtime embebido.
- [ ] No agrega entradas ejecutables alternativas, proveedores configurables/remotos, wrappers manuales ni binarios arbitrarios sin una spec nueva.
- [ ] No expone secretos, texto fuente, traducciones, headers, tokens ni paths sensibles.

## Validacion

- [ ] `make test-repository-boundary`
- [ ] `make fmt`
- [ ] `make clippy`
- [ ] `make test`
- [ ] `make deny`
- [ ] Se ejecutaron los gates `test-marketplace-*` afectados por el cambio.

## Notas para revision

Revisar primero seguridad, privacidad, contratos LSP/Core/runtime, empaquetado reproducible, limites y trazabilidad Spec Kit.
