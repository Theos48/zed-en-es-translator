# Diagramas

Diagramas Mermaid fuente para arquitectura y flujos estables. F010 esta
completada en `specs/006-direct-zed-translation/`, incluida su validacion manual
interactiva.

## Arquitectura directa actual

```mermaid
flowchart LR
    user[Usuario en Zed]
    action[Code action LSP]
    preview[Hover Markdown en Zed]
    extension[zed-extension]
    lsp[translator-lsp]
    mcp[Servidor MCP]
    cli[CLI Rust]
    core[Core Rust]
    provider[Provider]
    mock[MockProvider]
    local[Proveedor local LibreTranslate compatible]
    remote[Proveedor remoto confirmado]
    agent[Agent Panel puente F007]

    user --> action
    extension --> action
    action --> lsp
    lsp --> preview
    lsp --> core
    extension --> lsp
    extension -. compatibilidad .-> mcp
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
    preview[Mostrar hover versionado en Zed]
    keep[Conservar buffer y archivo sin cambios]
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
    preview --> keep
```

## Secuencia del flujo directo

```mermaid
sequenceDiagram
    actor User as Usuario
    participant Zed
    participant LSP as translator-lsp
    participant Core as translator-core
    participant Provider

    User->>Zed: Abrir code action
    Zed->>LSP: codeAction con URI rango y snapshot actual
    LSP-->>Zed: Accion con localidad sin texto ni edit
    User->>Zed: Ejecutar traduccion
    Zed->>LSP: executeCommand con URI version rango y tipo
    opt Provider remoto allowlisted
        LSP->>Zed: showMessageRequest por esta solicitud
        Zed-->>LSP: Confirmar o cancelar
    end
    LSP->>Core: Snapshot o seleccion permitida
    Core->>Provider: Solo segmentos idioma y tono
    Provider-->>Core: Segmentos traducidos
    Core-->>LSP: Resultado validado
    LSP-->>Zed: Preview listo sin contenido en notificacion
    User->>Zed: Hover sobre rango vigente
    Zed->>LSP: hover
    LSP-->>Zed: Preview Markdown de solo lectura
```

## Estado del preview directo

```mermaid
stateDiagram-v2
    [*] --> SinPreview
    SinPreview --> ObjetivoValidado: executeCommand vigente
    ObjetivoValidado --> ConfirmacionRemota: provider remoto
    ObjetivoValidado --> Traduciendo: offline o local
    ConfirmacionRemota --> Traduciendo: confirmacion positiva
    ConfirmacionRemota --> Rechazado: cancelar timeout o cambio
    Traduciendo --> PreviewVigente: resultado validado
    Traduciendo --> Rechazado: fallo redaccionado
    PreviewVigente --> SinPreview: didChange o didClose
    PreviewVigente --> PreviewVigente: nueva traduccion reemplaza anterior
    Rechazado --> SinPreview
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
