# Diagramas

Diagramas Mermaid fuente para arquitectura y flujos estables. La feature activa
se detalla en `specs/001-translation-core-contract/`.

## Arquitectura objetivo

```mermaid
flowchart LR
    user[Usuario en Zed]
    agent[Agent Panel]
    wrapper[Wrapper Zed]
    mcp[Servidor MCP]
    cli[CLI Rust]
    core[Core Rust]
    provider[Provider]
    mock[MockProvider]
    local[Proveedor local futuro]
    remote[Proveedor remoto futuro]

    user --> agent
    agent --> mcp
    wrapper -. arranca .-> mcp
    mcp --> cli
    cli --> core
    core --> provider
    provider --> mock
    provider -. futuro .-> local
    provider -. confirmacion por request .-> remote
```

## Frontera de documentacion

```mermaid
flowchart TD
    constitution[Constitucion]
    feature[specs/<feature>]
    decisions[docs/decisions y ADRs]
    context[docs/research product diagrams PLAN]
    code[Codigo y pruebas]

    constitution --> feature
    feature --> code
    decisions --> feature
    context --> decisions
```

## Primer ciclo formal

```mermaid
flowchart LR
    setup[Setup Rust/CLI]
    contracts[Contratos y limites]
    tests[Pruebas fallidas]
    impl[Implementacion minima]
    validate[make test]

    setup --> contracts
    contracts --> tests
    tests --> impl
    impl --> validate
```

## Lectura segura de archivo

```mermaid
flowchart TD
    path[Path solicitado]
    canonical[Canonicalizar workspace y path]
    inside{Dentro del workspace?}
    sensitive{Oculto o credencial?}
    allowed{Extension permitida?}
    size{Hasta 20 KiB?}
    utf8{UTF-8 texto?}
    read[Leer contenido]
    reject[Error normalizado]

    path --> canonical
    canonical --> inside
    inside -- no --> reject
    inside -- si --> sensitive
    sensitive -- si --> reject
    sensitive -- no --> allowed
    allowed -- no --> reject
    allowed -- si --> size
    size -- no --> reject
    size -- si --> utf8
    utf8 -- no --> reject
    utf8 -- si --> read
```

## Provider por segmentos

```mermaid
flowchart LR
    request[TranslateRequest]
    protect[Proteger formato y contenido]
    segments[Segmentos traducibles]
    provider[Provider]
    rebuild[Reconstruccion]
    success[translated_text]
    failure[ErrorCode + message]

    request --> protect
    protect --> segments
    segments --> provider
    provider --> rebuild
    rebuild --> success
    request -. validacion .-> failure
    provider -. fallo .-> failure
```
