# Investigacion: archivos iniciales y reglas de comentarios/docstrings

## Objetivo

Definir:

1. Que extensiones acepta `translate_file` en el MVP.
2. Que reglas usa el core para detectar texto traducible en codigo.
3. Que formatos quedan fuera hasta tener preservacion especifica.

## Fuentes oficiales

### Zed

Zed documenta soporte para Markdown de forma nativa y lista lenguajes relevantes para este proyecto, incluyendo AsciiDoc, Bash, JavaScript, Markdown, Python, ReStructuredText, Rust, Shell Script y TypeScript.

Fuentes:

- <https://zed.dev/docs/languages/markdown>
- <https://zed.dev/docs/languages/rust>
- <https://zed.dev/docs/languages/typescript>
- <https://zed.dev/docs/languages/python>

### Markdown

CommonMark define bloques de codigo fenced, bloques de codigo indentados, headings, listas, links y code spans. Para el MVP usaremos esta base para preservar codigo y estructura en Markdown.

Fuente: <https://spec.commonmark.org/0.31.2/>

### Rust

La referencia de Rust define comentarios de linea, bloque, doc comments externos e internos:

- `//`
- `/* ... */`
- `///`
- `//!`
- `/** ... */`
- `/*! ... */`

Fuente: <https://doc.rust-lang.org/reference/comments.html>

### JavaScript / TypeScript

ECMAScript define comentarios de una linea y multi-linea:

- `//`
- `/* ... */`

Tambien define hashbang comments. TypeScript y TSX se trataran con las mismas reglas base para comentarios, sin traducir JSX/TSX visible en el MVP.

Fuente: <https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-comments>

### Python

Python define comentarios con `#` hasta el final de la linea. PEP 257 define docstrings como literales string que aparecen como primera sentencia de modulo, funcion, clase o metodo.

Fuentes:

- <https://docs.python.org/3/reference/lexical_analysis.html>
- <https://peps.python.org/pep-0257/>

### Shell / Bash

Bash define comentarios con una palabra que empieza con `#`, al inicio de linea, despues de espacios no citados o despues de un operador. El comentario y el resto de la linea se ignoran. Para scripts, el shebang de primera linea `#!` debe preservarse.

Fuente: <https://www.gnu.org/software/bash/manual/bash.html#Comments>

## Contexto propio

Decisiones relevantes:

- MVP solo lectura.
- No modificar buffer.
- `translate_file` tiene limite de 20 KiB.
- Docs primero, codigo solo en modo comentarios/docstrings.
- Salida limpia en exito y errores claros cuando algo impida traducir.
- Proteger codigo siempre que sea posible.

## Decision A: extensiones iniciales de `translate_file`

El objetivo del MVP usable es que `translate_file` acepte:

```text
.md
.markdown
.txt
.rs
.ts
.tsx
.js
.jsx
.py
.sh
.bash
.zsh
```

### Clasificacion

Documentacion/texto completo:

```text
.md
.markdown
.txt
```

Codigo con modo comentarios/docstrings:

```text
.rs
.ts
.tsx
.js
.jsx
.py
.sh
.bash
.zsh
```

## Decision A1: recorte para primer ciclo Spec Kit

El primer ciclo formal de Spec Kit aceptara solo:

```text
.md
.markdown
.txt
```

Razon:

- permite cerrar core, contrato CLI, limites y privacidad sin depender todavia de parsing de codigo;
- evita traducir codigo por accidente antes de tener segmentador/parser robusto;
- deja las reglas de comentarios/docstrings como objetivo del MVP usable posterior, no como requisito del primer ciclo.

El soporte de `.rs`, `.ts`, `.tsx`, `.js`, `.jsx`, `.py`, `.sh`, `.bash` y `.zsh` se habilitara cuando existan fixtures y pruebas negativas que demuestren preservacion segura.

### Fuera del MVP de archivo completo

```text
.mdx
.rst
.adoc
.html
.xml
.json
.yaml
.yml
.toml
```

Razon:

- `.mdx` mezcla Markdown con JSX y requiere reglas especificas para no traducir codigo o expresiones.
- `.rst` y `.adoc` son formatos de documentacion relevantes, pero requieren preservacion propia antes de habilitar archivo completo.
- JSON/YAML/TOML/XML/HTML suelen contener configuracion, datos o markup donde traducir valores automaticamente puede ser destructivo.

`translate_text` seguira aceptando texto seleccionado o pegado sin validar extension, porque el usuario ya acota el fragmento.

## Decision B: reglas para comentarios/docstrings

### Rust

Traducir:

- contenido de `//`;
- contenido de `/* ... */`;
- contenido de `///`;
- contenido de `//!`;
- contenido de `/** ... */`;
- contenido de `/*! ... */`.

Preservar:

- delimitadores;
- indentacion;
- codigo alrededor.

### JavaScript / TypeScript / JSX / TSX

Traducir:

- contenido de `//`;
- contenido de `/* ... */`;
- contenido de docblocks `/** ... */`.

Preservar:

- hashbang `#!` si aparece al inicio;
- codigo;
- strings;
- template literals;
- JSX/TSX visible.

No traducir JSX/TSX visible en MVP, aunque sea texto humano, porque vive dentro de codigo y requiere reglas adicionales.

### Python

Traducir:

- contenido de comentarios `#`;
- docstrings de modulo, funcion, clase o metodo cuando sean primera sentencia segun PEP 257.

Preservar:

- encoding declarations y shebangs;
- strings que no sean docstrings;
- codigo.

### Shell

Traducir:

- comentarios `#` reconocidos por reglas shell.

Preservar:

- shebang inicial `#!`;
- comandos;
- strings quoted;
- variables;
- here-docs.

## Decision C: comportamiento en ambiguedad

Si el core no puede distinguir con confianza comentario/docstring de codigo, debe preservar el fragmento sin traducirlo.

Si un archivo de codigo no contiene segmentos traducibles, la herramienta debe responder con un mensaje claro en vez de inventar traduccion.

Codigo de error recomendado: `NO_TRANSLATABLE_SEGMENTS`.

## Decision D: seguridad de lectura

`translate_file` no debe tratar la extension como unica proteccion.

Reglas minimas:

- leer solo dentro del workspace autorizado;
- canonicalizar antes de abrir;
- rechazar escapes por `..` o symlink;
- rechazar contenido no UTF-8;
- rechazar contenido binario;
- rechazar archivos ocultos sensibles por defecto;
- aplicar limite de 20 KiB por bytes.

## Revision futura

Revisar estas decisiones cuando:

- se agregue soporte completo para `.rst`, `.adoc` o `.mdx`;
- se use tree-sitter u otro parser estructural;
- se permita traducir texto visible en JSX/TSX;
- se agreguen mas lenguajes de codigo.
