## Resumen

@coderabbitai summary

## Alcance

- [ ] Mantiene el scope de la feature activa o actualiza `specs/<feature>/`.
- [ ] No agrega proveedor real, red, MCP/Zed wrapper, edicion de buffers ni soporte completo de archivos de codigo sin una spec nueva.
- [ ] No expone secretos, texto fuente, traducciones, headers, tokens ni paths sensibles.

## Validacion

- [ ] `make fmt`
- [ ] `make clippy`
- [ ] `make test`

## Notas para revision

Revisar primero seguridad, privacidad, contrato CLI/Core, limites y trazabilidad Spec Kit.
