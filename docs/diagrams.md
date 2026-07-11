# Diagramas

Diagramas Mermaid fuente para arquitectura y flujos estables. No hay feature
formal activa; la siguiente candidata es F010, flujo directo de extension Zed
sin Agent. Los detalles operativos de una feature activa viven en
`specs/<feature>/`.

## Arquitectura objetivo

```mermaid
flowchart LR
    user[Usuario en Zed]
    action[Accion propia de extension]
    preview[Preview o resultado en Zed]
    extension[zed-extension]
    boundary[Frontera de traduccion]
    mcp[Servidor MCP]
    cli[CLI Rust]
    core[Core Rust]
    provider[Provider]
    mock[MockProvider]
    local[Proveedor local LibreTranslate compatible]
    remote[Proveedor remoto confirmado]
    agent[Agent Panel puente F007]

    user --> action
    action --> preview
    extension --> action
    action --> boundary
    boundary --> core
    boundary -. compatibilidad .-> mcp
    cli -. frontera publica .-> core
    mcp --> core
    core --> provider
    provider --> mock
    provider --> local
    provider -. confirmacion por request .-> remote
    agent -. validacion historica .-> mcp
```

## Flujo de producto objetivo

```mermaid
flowchart TD
    input[Seleccion o documento permitido]
    command[Accion de la extension]
    validate[Validar limites, workspace, UTF-8 y secretos]
    provider{Proveedor requiere salir del equipo?}
    confirm[Confirmacion explicita por solicitud]
    translate[Traducir segmentos permitidos]
    preview[Mostrar preview en Zed]
    apply{Usuario decide salida}
    copy[Copiar traduccion]
    insert[Insertar o aplicar si Zed lo permite]
    keep[No mutar buffer]
    reject[Error normalizado redaccionado]

    input --> command
    command --> validate
    validate --> provider
    validate -. fallo .-> reject
    provider -- no --> translate
    provider -- si --> confirm
    confirm -- aceptado --> translate
    confirm -- omitido --> reject
    translate --> preview
    preview --> apply
    apply -- copiar --> copy
    apply -- accion explicita --> insert
    apply -- cerrar --> keep
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
