# Mapa de features

Este mapa conserva suficiente contexto para iniciar futuros ciclos de Spec Kit
sin duplicar la feature activa. Los identificadores funcionales `F###` son
históricos y no tienen que coincidir con el número del directorio `specs/`.

## Dirección de producto

La única arquitectura soportada es:

```text
Zed Gallery -> paquete local verificado -> LSP -> core -> runtime embebido
```

La extensión traduce inglés → español de forma local después de una adquisición
pública inicial, muestra un preview de solo lectura y conserva el contenido
original. No se agrega otra frontera ejecutable sin una feature, consumidor
demostrado y revisión constitucional.

## Estado de la release inicial

### F009 + F012: paquete automático y publicación

**Estado**: implementado localmente en
`specs/009-zed-marketplace-install/`; publicación pendiente.

Entrega:

- instalación desde la Gallery sin preparación externa;
- paquete Linux `x86_64` de identidad fija bajo propiedad de Zed;
- runtime Bergamot y tres recursos Mozilla `en → es` verificados;
- traducción offline después de preparar el paquete;
- estado de preparación, retry, concurrencia y último paquete válido;
- no-mutación, límites, redacción, licencias y remoción por Zed.

Pendiente:

1. publicar tag/asset y validar el paquete interactivo;
2. enviar al registro oficial;
3. completar 3/3 instalaciones limpias después del merge upstream.

### Ciclo 010: convergencia del repositorio

**Estado**: completado y validado en
`specs/010-repository-convergence/`.

Resultado: quedaron únicamente superficies con consumidor en el producto
Gallery, su supply chain, validación, gobierno o roadmap. Git conserva la
historia completa; ADR 0007 registra la retirada de las arquitecturas
experimentales.

## Fundamentos retenidos

| ID | Capacidad | Estado vigente |
|---|---|---|
| F001 | Contratos de traducción | Consolidado en core, constitución y features 009/010. |
| F002 | Preservación de Markdown y código protegido | Obligatoria y cubierta por gates retenidos. |
| F003 | Doble determinista de traducción | Retenido solo para pruebas inyectadas. |
| F008 | Privacidad, límites y redacción | Consolidado en constitución 2.0.0 y 009. |
| F010 | Acción directa y preview en Zed | Retenida mediante LSP; siempre de solo lectura. |
| F012 | Runtime local embebido | Integrado en el paquete automático de F009. |

Las iteraciones F004-F007 y F011 validaron alternativas que ya no tienen
consumidor en la release. ADR 0001-0005 conservan su razón histórica y ADR 0007
las supersede operacionalmente. Sus artefactos completos permanecen en Git, no
como instrucciones o specs activas.

## Backlog posterior a `v0.1.0`

### F013: paridad con las plataformas oficiales de Zed

**Prioridad**: primera feature de producto posterior a `v0.1.0`.

**Objetivo**: llevar el mismo producto local y de solo lectura a todas las
combinaciones de 64 bits soportadas oficialmente por Zed, sin debilitar el
paquete exacto ni generalizar una identidad no probada.

Matriz objetivo:

| Sistema | `x86_64` | `aarch64` |
|---|---|---|
| Linux | base de `v0.1.0` | pendiente |
| macOS | pendiente | pendiente |
| Windows | pendiente | pendiente |

Cada ciclo debe:

- seleccionar una sola combinación de sistema operativo y arquitectura;
- construir LSP y runtime de forma reproducible para esa combinación;
- registrar fuentes, tamaños, hashes, licencias y compatibilidad exactos;
- medir adquisición, tamaño instalado, RAM, hilos y latencia;
- probar preparación, retry, offline, disable y uninstall en Zed real;
- conservar el fallo antes de red/estado para plataformas aún no soportadas.

La implementación avanza una combinación a la vez y puede ordenar entregas
según la capacidad real de construcción y prueba. F013 solo se cierra cuando
todas las celdas pendientes de la matriz tengan paquete publicado y evidencia
real. Si Zed cambia su soporte oficial, la matriz se revisa sin prometer
plataformas de 32 bits ni sistemas que Zed no soporte.

Fuera de alcance: cambiar la experiencia, agregar proveedores, permitir
configuración de ejecutables o crear otro ciclo de instalación.

### F014: mejoras nativas de preview y estado

**Objetivo**: aprovechar nuevas APIs estables de Zed para mejorar lectura y
progreso sin crear una segunda aplicación ni degradar seguridad.

Criterios iniciales:

- conservar localidad de selección, versión del documento y cancelación;
- mejorar estados de checking, download, ready y retry dentro de Zed;
- mantener el preview legible y de solo lectura;
- no crear almacenamiento fuera del directorio propiedad de la extensión;
- documentar la versión de API y evidencia real antes de cambiar UX;
- excluir insert, replace o apply hasta que una enmienda constitucional lo
  permita expresamente.

### F015: pares de idiomas adicionales

**Objetivo**: agregar un par de idioma como paquete exacto independiente sin
convertir la primera release en un selector arbitrario de modelos.

Cada par debe:

- tener demanda, dirección y alcance explícitos;
- usar recursos con procedencia, licencia, tamaños y hashes revisados;
- conservar segmentación, límites y estructuras protegidas;
- demostrar calidad mínima con fixtures públicos y métricas reproducibles;
- definir cómo convive con el presupuesto de descarga/almacenamiento;
- mantener adquisición separada del contenido traducido y uso posterior
  offline.

## Regla de promoción

Al promover una entrada futura:

1. usar sus criterios como entrada de `speckit-specify`;
2. separar con claridad lo incluido y lo diferido;
3. mantener aquí solo estado y detalle útil para ciclos posteriores;
4. registrar cambios estables en `docs/decisions.md` o un ADR;
5. dejar contratos, tareas y evidencia operativa en `specs/<feature>/`.
