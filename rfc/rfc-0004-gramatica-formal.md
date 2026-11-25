# RFC-0004-A: Gramática Formal del Lenguaje Cosmos

> © 2026 Luis Alberto Burciaga Miker. Todos los derechos reservados.
> CC BY-NC-ND 4.0

**Estado:** CONGELADO | **Versión:** 1.0 (reescritura 2026-11-25) | **Fecha:** 2026-11-25

> Esta es la gramática canónica que todo parser debe implementar. Es la
> misma BNF del §3 de RFC-0004-F. La reescritura agrega comentarios,
> `usa`, `satellite` dentro del planeta padre, `star`, `type` obligatorio
> y la `@` pegada al identificador. El texto anterior no incluía
> comentarios y su ejemplo usaba `@ id` con espacio, que ningún parser
> de esta gramática acepta.

---

## 1. Prefacio y Contexto

Este documento define la gramática formal del lenguaje Cosmos (archivos `.cosmos`). Establece la sintaxis canónica que todo parser debe implementar.

### Principios Fundacionales

#### G-003 — Primacía Geométrica (LEY)

> La geometría ES el programa, no lo representa.
> La órbita se CALCULA de la posición, nunca se declara.

---

## 2. Gramática Formal BNF

```bnf
<cosmos-file>       ::= "universe" <string> "{" <universe-body> "}"

<universe-body>     ::= <metadata> <sun-declaration>+ <star-declaration>* <galaxy-declaration>+ <light-declaration>*

<metadata>          ::= "metadata" "{" 
                          "version:" <string> 
                          "author:" <string> 
                          "standard:" "Cosmos" 
                          "}"

<sun-declaration>   ::= "sun" <identifier>
                      | "sun" <identifier> "{" <position> "}"   // → P012 si se usa

<star-declaration>  ::= "star" <string> <orbit-identifier> "{" "value:" <value> "}"
<value>             ::= <string> | <number>

<galaxy-declaration>::= "galaxy" <string> <orbit-identifier> "{" <position> <galaxy-body> "}"
<galaxy-body>       ::= <planet-declaration>* <nebula-declaration>*

<nebula-declaration>::= "nebula" <string> "{" <planet-declaration>* "}"

<planet-declaration>::= "planet" <string> <orbit-identifier> "{" <planet-body> "}"
<planet-body>       ::= <position> <mass> <type> <usa-declaration>* <satellite-declaration>*

<satellite-declaration>::= "satellite" <string> <orbit-identifier> "{" 
                             <relative-position> <mass> <usa-declaration>* 
                             "}"
<relative-position> ::= "position:" "(" <number> "," <number> ")"

<position>          ::= "position:" "(" <number> "," <number> ")"
<mass>              ::= "mass:" <integer>
<type>              ::= "type:" <planet-type>
<planet-type>       ::= "Storage" | "Operation" | "Compute" | "Interface"
<usa-declaration>   ::= "usa" <orbit-identifier>

<light-declaration> ::= "light" "{" <light-from> <light-to> <light-flow> <expiry>? "}"
<light-from>        ::= "from:" (<orbit-identifier> | <identifier>)
<light-to>          ::= "to:" <orbit-identifier>
<light-flow>        ::= "flow:" <flow-type>
<expiry>            ::= "restringido_hasta:" <date>
<flow-type>         ::= "Primary" | "Asynchronous" | "Read"

<orbit-identifier>  ::= "@" <identifier>
<date>              ::= <year> "-" <month> "-" <day>
<year>              ::= <digit>{4}
<month>             ::= <digit>{2}
<day>               ::= <digit>{2}
<identifier>        ::= <letter> (<letter> | <digit> | "_")*
<string>            ::= "\"" <char>* "\""
<number>            ::= <integer> | <float>
<integer>           ::= <digit>+
<float>             ::= <integer> "." <digit>+
<letter>            ::= "a".."z" | "A".."Z"
<digit>             ::= "0".."9"
```

## 3. Comentarios

```
// Comentario de línea.

/*
   Comentario de bloque.
*/
```

Los comentarios se ignoran antes del análisis léxico. No tienen
significado semántico y no aparecen en el AST.

## 4. Sistema de Tipos de Flujo

| Tipo | Descripción |
|------|-------------|
| `Primary` | Flujo principal de ejecución |
| `Asynchronous` | Ejecución paralela, no bloqueante |
| `Read` | Solo lectura de datos |

## 5. Sistema de Tipos de Planeta

| Tipo | Órbitas válidas |
|------|-----------------|
| `Storage` | Logic (4) |
| `Operation` | Logic (4) |
| `Compute` | Logic (4) |
| `Interface` | Interface (3) |

No existe el tipo `Core`. La banda Core es solo del Sol.

## 6. Estructura de Archivo .cosmos

```cosmos
universe "Nombre" {
    metadata {
        version: "1.0"
        author: "Autor"
        standard: "Cosmos"
    }

    sun Sol

    galaxy "Galaxia" @galaxia {
        position: (20.0, 0.0)

        nebula "Nebulosa" {
            planet "Planeta" @planeta {
                position: (25.0, 0.5)
                mass: 100
                type: Operation
            }
        }
    }

    light {
        from: Sol
        to: @planeta
        flow: Primary
    }
}
```

## 7. Tokens Reservados

```
universe, metadata, sun, star, galaxy, nebula, planet, satellite
position, mass, type, usa, value
light, from, to, flow
Primary, Asynchronous, Read
Storage, Operation, Compute, Interface
restringido_hasta
Cosmos
```

## 8. Notas para implementadores

1. **El `@` va pegado al identificador.** `@id` es un solo token;
   `@ id` con espacio es error de parser (P001/P009).
2. **Las referencias usan `@id`, salvo el Sol**, que se referencia por
   su nombre (es único por G-001). `from:` acepta `@id` o el nombre del
   Sol; `to:` y `usa` solo aceptan `@id`.
3. **`sun` admite más de una declaración en la gramática** para que el
   segundo Sol sea error semántico E108, no error de parser. La
   gramática no decide la validez; la validación sí.
4. **La posición del Sol en la gramática existe solo para emitir P012.**
   Un Sol con posición es error, no sintaxis válida.
5. **Los satélites solo se declaran dentro de su planeta padre.**
   Un satélite fuera de un planeta es error de parser (P014).
6. **`mass` y `type` son obligatorios** en cada planeta y `mass` en cada
   satélite. Su ausencia es P007/P008.

## 9. Estados de Auditoría

| Observación | Estado |
|-------------|--------|
| G-001 | ✅ RESUELTA |
| G-002 | ✅ RESUELTA (con excepción documentada del Sol) |
| G-003 | ✅ APLICADA |
| G-004 | ✅ RESUELTA |
| G-005 | ✅ RESUELTA |
| G-006 | ✅ RESUELTA |
| G-007 | ✅ RESUELTA |

---

> © 2026 Luis Alberto Burciaga Miker. Todos los derechos reservados.
> Este sistema y su lógica fundamental están protegidos por secreto comercial.
