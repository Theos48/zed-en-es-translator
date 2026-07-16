# Diagramas

Diagramas Mermaid de la única arquitectura soportada y sus fronteras estables.

## Arquitectura del producto

```mermaid
flowchart LR
    user[Usuario en Zed]
    gallery[Extension Gallery]
    extension[Extensión Rust WASM]
    work[Work dir propiedad de Zed]
    package[Paquete local verificado]
    lsp[translator-lsp]
    core[translator-core]
    runner[Runtime Bergamot privado]
    models[Recursos Mozilla en-es]
    preview[Hover de solo lectura]

    user --> gallery
    gallery --> extension
    extension --> work
    work --> package
    package --> lsp
    lsp --> core
    core --> runner
    runner --> models
    lsp --> preview
    preview --> user
```

Solo la extensión adquiere entradas públicas. LSP, core y runtime no descargan
ni aceptan rutas o motores elegidos por el usuario.

## Preparación del paquete

```mermaid
flowchart TD
    activate[Zed solicita el language server]
    platform{Linux x86_64?}
    current{Paquete activo válido?}
    lock[Adquirir lock exclusivo]
    download[Descargar fuentes fijas a staging]
    verify[Validar tamaño hash layout y licencias]
    promote[Promoción atómica]
    ready[Lanzar LSP verificado]
    previous[Conservar último paquete válido]
    retry[Error redaccionado y retry normal]
    unsupported[Mensaje de plataforma no soportada]

    activate --> platform
    platform -- no --> unsupported
    platform -- sí --> current
    current -- sí --> ready
    current -- no --> lock
    lock --> download
    download --> verify
    verify -- válido --> promote
    promote --> previous
    previous --> ready
    download -. fallo .-> retry
    verify -. fallo .-> retry
```

Staging nunca es ejecutable. Un fallo no desplaza un paquete activo verificado.

## Traducción local

```mermaid
sequenceDiagram
    actor User as Usuario
    participant Zed
    participant LSP as translator-lsp
    participant Core as translator-core
    participant Runtime as runtime embebido

    User->>Zed: Ejecutar acción sobre selección o documento permitido
    Zed->>LSP: Rango, URI y versión
    LSP->>Core: Snapshot validado
    Core->>Core: Límites, path, UTF-8 y segmentación segura
    Core->>Runtime: Solo segmentos permitidos
    Runtime-->>Core: Segmentos traducidos
    Core-->>LSP: Resultado reconstruido y acotado
    LSP-->>Zed: Preview vigente sin WorkspaceEdit
    Zed-->>User: Hover de solo lectura
```

Un cambio o cierre del documento invalida el preview. Ningún paso escribe el
buffer o el archivo fuente.

## Estado local de adquisición

```mermaid
stateDiagram-v2
    [*] --> Ausente
    Ausente --> Comprobando: activación soportada
    Comprobando --> Descargando: lock adquirido
    Comprobando --> Listo: activo válido
    Descargando --> Listo: verificación y promoción
    Descargando --> Fallido: red almacenamiento o integridad
    Fallido --> Comprobando: retry desde Zed
    Listo --> Comprobando: actualización
    Comprobando --> Listo: candidato falla y activo sigue válido
    Listo --> [*]: uninstall de Zed
```

## Autoridad documental

```mermaid
flowchart TD
    constitution[Constitución 2.0.0]
    cleanup[Feature 010 convergencia]
    release[Feature 009 release]
    decisions[Decisiones y ADRs]
    roadmap[PLAN y feature map]
    code[Código y gates retenidos]

    constitution --> cleanup
    constitution --> release
    decisions --> cleanup
    decisions --> release
    cleanup --> code
    release --> code
    roadmap --> cleanup
    roadmap --> release
```
