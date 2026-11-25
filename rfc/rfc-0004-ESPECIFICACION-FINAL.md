# RFC-0004-F: Especificación Completa del Lenguaje Cosmos

> **Estado:** FINAL | **Versión:** 1.0 (reescritura 2026-11-25) | **Fecha:** 2026-11-25
> © 2026 Luis Alberto Burciaga Miker. Todos los derechos reservados.
> CC BY-NC-ND 4.0
>
> **Nota de reescritura:** este texto reemplaza en su lugar al del 2026-07-12. No es
> una versión nueva: es la misma v1.0, corregida. El texto anterior se contradecía
> a sí mismo (la regla de acceso estaba escrita de dos formas incompatibles, las
> bandas orbitales no coincidían con la implementación en Rust, y el ejemplo
> completo violaba sus propias reglas). Esta reescritura lo deja en un estado
> en el que un compilador puede implementarlo literalmente y rechazar lo que
> el documento dice que debe rechazar. No existe solicitud provisional de
> patente en trámite; el enunciado que afirmaba lo contrario se elimina.

---

## TABLA DE CONTENIDOS

0. [Decisiones de la reescritura](#0-decisiones-de-la-reescritura)
1. [Principios Fundacionales](#1-principios-fundacionales)
2. [Estructura del Universo](#2-estructura-del-universo)
3. [Gramática Formal BNF](#3-gramática-formal-bnf)
4. [Sistema de Órbitas](#4-sistema-de-órbitas)
5. [Reglas de Acceso](#5-reglas-de-acceso)
6. [Sistema de Restricción Temporal en Flujos](#6-sistema-de-restricción-temporal-en-flujos)
7. [Sistema de Tipos de Flujo](#7-sistema-de-tipos-de-flujo)
8. [Catálogo de Errores](#8-catálogo-de-errores)
9. [Reglas de Validación](#9-reglas-de-validación)
10. [Comparativa Cosmos vs Gestores Tradicionales](#10-comparativa-cosmos-vs-gestores-tradicionales)
11. [Ejemplo Completo](#11-ejemplo-completo)
12. [Historial de Cambios](#12-historial-de-cambios)

---

## 0. Decisiones de la reescritura

Cada punto resuelve una contradicción del texto anterior. Ninguna decisión
descarta un ejemplo del documento: todas se tomaron para que los ejemplos
que el texto mostraba como válidos lo sean de verdad, y para que la
especificación coincida con la única implementación que valida topología
(`cosmos-topology`, Rust, `src/lib.rs`, 26 tests).

| # | Decisión | Texto anterior | Razón |
|---|----------|----------------|-------|
| D1 | **Bandas: 4, no 5.** Core 0–10, Logic 10.1–50, Interface 50.1–100, DeepSpace >100 prohibido. | §4 tenía 5 bandas (Service 50.1–100, Data 100.1–200, Interface >200). | El Rust implementa 4 bandas y el propio catálogo de errores del texto anterior (E007, banda prohibida >100) ya decía 4. Un radio de 150 era válido en el §4 e inválido en E007. |
| D2 | **La regla de acceso se parte en dos.** `usa` (dependencia): hacia adentro, el objetivo debe estar en una órbita de número mayor o igual. `light` (flujo): lo gobiernan la matriz de tipos (§7) y el alcance (§5 de acceso), no hay regla de dirección por órbita. | La "REGLA ORO" (el menor accede al mayor) y la tabla §4.4 decían lo contrario, y el ejemplo permitía un flujo 5 → 4. | Con dos reglas, todos los ejemplos del documento son válidos a la vez: `usa` 4 → 5, 3 → 4, 3 → 5; `light` 5 → 4. Una sola regla no podía cubrirlos. |
| D3 | **No existe el tipo de planeta `Core`.** `PlanetType` = Storage, Operation, Compute, Interface. La banda Core es solo del Sol. | §3.4 listaba `Core` como tipo de planeta y el ejemplo tenía planetas de tipo `Core`. | El Rust no tiene variante `Core` en `PlanetType`, y todos los tipos exigen Logic o Interface. Un planeta de tipo `Core` no es representable en la implementación. |
| D4 | **`mass` y `type` son obligatorios en cada planeta.** No hay masa por defecto. | La BNF los marcaba opcionales y §2 decía "default: 100". | P007/P008 (campo `mass`/`type` faltante) ya existían en el código, y sin `type` la matriz de flujos no se puede evaluar. |
| D5 | **`restringido_hasta` solo va en flujos `light`.** Nunca en planetas. | §5.1 lo decía, pero el ejemplo lo ponía en un planeta. | Se mueve la caducidad del ejemplo a su flujo. |
| D6 | **Comentarios `//` y `/* */` son sintaxis válida. La `@` va pegada al identificador.** | La BNF no incluía comentarios y el ejemplo usaba `@ galaxy_legacy` con espacio, que la gramática rechaza. | El ejemplo debe compilar. Se fija el ejemplo a `@id` pegado, que es la forma canónica. |
| D7 | **El Sol tiene masa 10 y alcance 100.** Posición implícita (0,0). | El Sol no tenía masa y el alcance del emisor no estaba definido para él. | Con masa 10, `max_reach = 1000 / 10 = 100`, y el Sol alcanza cualquier nodo del universo (radio máximo 100). |
| D8 | **E401 es solo caducidad de flujo.** La referencia sin `@` es error de parser P009, no E401. | E401 era a la vez "AmbiguousReference" (§7.3) y "FlowAuthorizationExpired" (§5.8). | La gramática exige `@` en las referencias, así que el caso no llega a validación semántica. |
| D9 | **La fecha de caducidad se compara con la fecha real.** | El validador temporal comparaba contra 2026-07-13 fija. | Una caducidad comparada contra una fecha congelada no caduca nunca. |
| D10 | **Las constantes numéricas quedan fijadas en el documento.** `EPSILON = 0.5`, `GALACTIC_EPSILON = 1.0`, `SATELLITE_MASS_RATIO = 0.5`, `BASE_CONSTANT = 1000.0`, `TENSION_THRESHOLD = 10`. | E003 (alcance) y E004 (tensión) no tenían valores definidos en el texto. | Sin constantes, esos errores no eran implementables. Los valores son los del Rust. |

---

## 1. Principios Fundacionales

### G-001: Unicidad del Origen

> Solo puede existir UN `Sun` por Universe. El Sun es la fuente de verdad absoluta.

```cosmos
// ✅ VÁLIDO
universe "MiProyecto" {
    sun NucleoGlobal
    // ...
}

// ❌ INVÁLIDO - Error E108
universe "MiProyecto" {
    sun NucleoUno
    sun NucleoDos  // Error: Solo un Sun permitido
}
```

### G-002: Separación Nombre / Identificador

> Cada entidad tiene un nombre visible Y un `@identificador_único`. Las referencias NUNCA usan nombres.
>
> **Única excepción:** el Sol. El Sol se declara sin `@id` y se referencia por
> su nombre, porque G-001 garantiza que hay exactamente uno y la referencia
> no puede ser ambigua.

| Entidad | Nombre | Identificador |
|---------|--------|---------------|
| Planet | `React` | `@react_17_0_2` |
| Galaxy | `LegacyApp` | `@galaxy_legacy_app` |
| Universe | `MiProyecto` | `@universe_mi_proyecto` |
| Sun | `NucleoSistema` | (sin `@id`; referencia por nombre) |

```cosmos
// ✅ VÁLIDO
planet "React" @react_17_0_2 {
    usa @react_17_0_2  // Usa identificador
}

// ❌ INVÁLIDO - Error P009
planet React {
    usa React  // Error: Referencia sin @id
}
```

### G-003: Primacía Geométrica

> La órbita se CALCULA de la posición, nunca se declara. La geometría ES el programa.

```cosmos
// ✅ La órbita se calcula de la posición automáticamente
planet LogicaCore {
    position: (5.0, 0.0)  // → Core (radio 5.0) — ver §4
}

planet Servicio {
    position: (25.0, 0.0)  // → Logic (radio 25.0)
}
```

### G-004: Independencia de tipos externos

> Ningún tipo de Cosmos depende de un tipo de otro lenguaje o plataforma.
> `PlanetType` y `FlowType` son cerrados y están definidos en §7.

### G-005: Flujos vectoriales definidos

> Todo flujo tiene origen, destino y tipo. No existen flujos implícitos.
> Un flujo que no está declarado no existe.

### G-006: Nebulous solo agrupadoras

> Una `nebula` es un espacio de nombres organizativo. No tiene posición
> propia, no tiene masa, no emite ni recibe flujos. Los planetas dentro
> de una nebulosa pertenecen a la galaxia que la contiene.

### G-007: Metadatos obligatorios

> Todo `universe` declara `metadata` con `version`, `author` y
> `standard: "Cosmos"`. Sin metadatos, el archivo no es un programa Cosmos.

---

## 2. Estructura del Universo

### Jerarquía Completa

```
Universe
├── metadata (obligatorio)
├── Sun (exactamente uno; posición implícita (0, 0); masa 10)
├── Star (cero o más; constante global inmutable; sin posición)
├── Galaxy (una o más; E112 si no hay ninguna)
│   ├── position (obligatorio)
│   ├── Planet (múltiples)
│   │   ├── position (obligatorio)
│   │   ├── mass (obligatorio)
│   │   ├── type (obligatorio)
│   │   ├── usa (dependencias, cero o más)
│   │   └── Satellite (múltiples; declarados DENTRO del planeta padre)
│   │       ├── position (relativa al padre: desplazamiento polar)
│   │       ├── mass (obligatorio)
│   │       └── usa (cero o más)
│   └── Nebula (opcional; sin posición)
│       └── Planet (agrupados; pertenecen a la galaxia)
└── LightFlow (conexiones)
    ├── from (identificador, o nombre del Sol)
    ├── to (identificador)
    ├── flow (tipo)
    └── restringido_hasta (opcional, solo en flujos)
```

### Reglas de posición

- **Posición polar.** Toda posición es un par `(radio, theta)`: radio en
  unidades de distancia al Sol, `theta` en radianes. La conversión a
  cartesianas es `x = radio · cos(theta)`, `y = radio · sin(theta)`.
  Todas las distancias se miden en cartesianas.
- **Radio.** `0.0 <= radio <= 100.0`. Radio negativo → E005. Radio mayor
  que 100.0 → E007 (DeepSpace). `theta` debe ser finito → E005 si no lo es.
- **Sol.** Posición implícita `(0, 0)`. Declarar una posición para el Sol
  es error P012.
- **Satélite.** Su posición declarada es un **desplazamiento polar**
  respecto a su planeta padre: `radio_absoluto = radio_padre + radio_declarado`,
  `theta_absoluto = theta_padre + theta_declarado`. La posición absoluta
  resultante se valida como cualquier otra (E005, E007) y debe quedar en
  la misma banda orbital que el padre (E106 si no).
- **Galaxia.** Su posición es el ancla del dominio. No puede estar en Core
  (radio ≤ 10.0) → E107, ni en DeepSpace (radio > 100.0) → E007.

### Reglas de masa

- **Planeta:** `mass` obligatoria, entero ≥ 1.
- **Sol:** masa 10, fija. Alcance `max_reach = 1000 / 10 = 100`.
- **Satélite:** `mass` obligatoria. La suma de las masas de los satélites
  de un planeta no puede superar el 50% de la masa del planeta
  (`SATELLITE_MASS_RATIO = 0.5`) → E111.
- **Galaxia y nebulosa:** no tienen masa.
- **Alcance.** Todo nodo con masa `m` tiene alcance máximo
  `max_reach = 1000.0 / m`. Un flujo cuya distancia supera el alcance
  del emisor es error E003.

---

## 3. Gramática Formal BNF

### 3.1 Notación BNF

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

### 3.2 Comentarios

```cosmos
// Comentario de línea: todo hasta el fin de la línea se ignora.

/*
   Comentario de bloque: todo entre /* y */ se ignora.
*/
```

### 3.3 Tokens Reservados

```
universe, metadata, sun, star, galaxy, nebula
planet, satellite
position, mass, type, usa, value
light, from, to, flow
restringido_hasta
Primary, Asynchronous, Read
Storage, Operation, Compute, Interface
Cosmos
```

### 3.4 Tipos de Entidades

| Tipo | Descripción | Requiere Position |
|------|-------------|-------------------|
| `Sun` | Origen único de verdad | ❌ Prohibido (P012 si se declara) |
| `Star` | Constante global inmutable | ❌ No tiene |
| `Galaxy` | Dominio aislado | ✅ Obligatorio |
| `Nebula` | Espacio de nombres | ❌ No tiene |
| `Planet` | Módulo funcional | ✅ Obligatorio |
| `Satellite` | Submódulo relativo | ✅ Relativo al padre |

### 3.5 Tipos de PlanetType y órbitas válidas

| Tipo | Órbitas Válidas | Descripción |
|------|-----------------|-------------|
| `Storage` | Logic (4) | Persistencia de datos |
| `Operation` | Logic (4) | Lógica de negocio |
| `Compute` | Logic (4) | Procesamiento de datos |
| `Interface` | Interface (3) | Capa de entrada/salida |

No existe el tipo `Core`. La banda Core es solo del Sol (§4). Un planeta
cuyo tipo no admite su banda es error E110.

---

## 4. Sistema de Órbitas

### 4.1 Definición Formal

Las **órbitas** son niveles jerárquicos de privilegio y cercanía al centro.
Un número mayor de órbita = mayor privilegio = mayor cercanía al centro.
Hay tres bandas válidas; más allá de 100.0 no hay órbita, hay violación.

| Órbita | Banda | Radio | Privilegio | Descripción |
|--------|-------|-------|------------|-------------|
| **5** | Core | 0.0 – 10.0 | Máximo | Solo el Sol. Estado inmutable, reglas fundamentales. |
| **4** | Logic | 10.1 – 50.0 | Alto | Lógica de negocio, decisiones. |
| **3** | Interface | 50.1 – 100.0 | Medio | Capa de entrada/salida, adaptadores externos. |
| — | DeepSpace | > 100.0 | — | Prohibido. Error E007. |

**Límites inclusivos.** La banda se deriva del radio así: radio ≤ 10.0 →
Core; 10.0 < radio ≤ 50.0 → Logic; 50.0 < radio ≤ 100.0 → Interface;
radio > 100.0 → DeepSpace. Un radio exactamente en el límite pertenece
a la banda interior (10.0 es Core, 50.0 es Logic, 100.0 es Interface).

### 4.2 Cálculo de Órbita

```cosmos
función calcular_orbita(radio):
    si radio <= 10.0:          return 5  // Core
    sino si radio <= 50.0:     return 4  // Logic
    sino si radio <= 100.0:    return 3  // Interface
    sino:                      return DeepSpace  // Error E007
```

### 4.3 Constantes

| Constante | Valor | Uso |
|-----------|-------|-----|
| `EPSILON` | 0.5 | Distancia mínima entre dos nodos de la misma galaxia (E002). |
| `GALACTIC_EPSILON` | 1.0 | Distancia mínima entre dos galaxias (E002). |
| `SATELLITE_MASS_RATIO` | 0.5 | Masa satelital máxima: 50% de la masa del planeta (E111). |
| `BASE_CONSTANT` | 1000.0 | Alcance: `max_reach = 1000.0 / masa` (E003). |
| `TENSION_THRESHOLD` | 10 | Cruces máximos entre flujos (E004). |

### 4.4 Notas de Implementación

1. **El valor de órbita es INMUTABLE** una vez asignado desde la posición.
2. **La verificación ocurre en tiempo de compilación** para todos los
   `light` y todos los `usa`.
3. **La banda Core es solo del Sol.** Ningún planeta puede estar en Core:
   ningún `PlanetType` admite la banda Core (E110).
4. **No hay excepciones** a las reglas de este capítulo salvo
   `restringido_hasta` (§6), que suspende temporalmente la regla del
   flujo que lo lleva, no la regla de órbitas en general.

---

## 5. Reglas de Acceso

Hay **dos** reglas, una para dependencias y una para flujos. Son distintas
porque describen movimientos distintos: una dependencia apunta hacia el
núcleo (lo que un módulo necesita), un flujo sale del núcleo (lo que un
módulo entrega). El texto anterior las fundía en una sola regla y se
contradecía; aquí quedan separadas.

### 5.1 Dependencias (`usa`): hacia adentro

> Un nodo puede depender de nodos en su misma órbita o en órbitas de
> número mayor (más cercanas al núcleo). Nunca de órbitas de número menor.

```
usa PERMITIDO:  3 → 3, 3 → 4, 3 → 5, 4 → 4, 4 → 5, 5 → 5
usa DENEGADO:   4 → 3, 5 → 4, 5 → 3, 4 → 2, ...
```

- El objetivo debe existir (E103) y estar en la misma galaxia (E403).
- Un `usa` que apunta a una órbita menor es error E001.
- El grafo de dependencias no puede tener ciclos (E404).
- Un satélite solo puede usar `usa` hacia nodos de su misma galaxia
  (E106 si apunta fuera).

```cosmos
// ✅ 4 → 5: Logic depende de Core
planet "Servicio" @servicio_logico {
    position: (25.0, 0.0)
    usa @reglas_core
}

// ❌ E001: 3 → 4 no es el problema; 4 → 3 sí lo es
planet "Nucleo" @nucleo {
    position: (5.0, 0.0)
    usa @util_servicio  // Error E001: Core no depende de Interface
}
```

### 5.2 Flujos (`light`): matriz de tipos y alcance

> Un flujo no tiene regla de dirección por órbita. Su dirección la
> gobiernan dos cosas: la **matriz de tipos** (§7.3) y el **alcance**
> del emisor (`max_reach = 1000.0 / masa`). Además debe cumplir:
> mismo dominio (E403), no bidireccional (E006), sin ciclos (E404),
> receptor existente (E103), receptor que no sea el Sol ni una Star (E105),
> distancia dentro del alcance (E003), y emisor y receptor fuera de
> DeepSpace (E007).

- **El Sol** puede emitir hacia cualquier planeta (es el origen; su
  alcance es 100 y cubre todo el universo). El Sol no recibe flujos (E105).
- **Un satélite** está exento de la matriz de tipos: es parte de su
  planeta padre, no un planeta independiente. Un flujo hacia un satélite
  solo debe cumplir alcance, dominio, no bidireccionalidad y aciclicidad.
- **Interface → Core directo** está prohibido (E001). En la práctica
  solo puede darse hacia el Sol, donde rige E105, que se comprueba antes.

### 5.3 Resumen

| Construcción | Regla de dirección | Otras reglas |
|--------------|--------------------|--------------|
| `usa` | Hacia adentro: órbita destino ≥ órbita origen | E103, E403, E404, E001 |
| `light` | Matriz de tipos (§7.3) + alcance (E003) | E103, E403, E404, E105, E006, E007 |

---

## 6. Sistema de Restricción Temporal en Flujos

### 6.1 Definición Formal

`restringido_hasta` es un modificador de **flujo** (`light`) que indica
autorización temporal. Es una excepción controlada que permite un flujo
que de otra forma estaría prohibido por la matriz de tipos o por el
alcance. **No cambia la regla de órbitas; solo la suspende temporalmente
para ese flujo.**

**Restricciones:**
- ✅ Solo se define en `light` (flujos), **nunca en planetas ni en ningún
  otro elemento**. Ponerlo en un planeta es error de parser (P014).
- ✅ Requiere formato AAAA-MM-DD.
- ✅ Al vencer la fecha, el flujo se bloquea AUTOMÁTICAMENTE.
- ✅ La fecha se compara con la **fecha real actual**, no con una fecha fija.

### 6.2 Sintaxis BNF

```
<expiry> ::= "restringido_hasta:" <date>
<date>   ::= <year> "-" <month> "-" <day>
<year>   ::= <digit>{4}    // 2026, 2027, etc.
<month>  ::= <digit>{2}    // 01-12
<day>    ::= <digit>{2}    // 01-31
```

### 6.3 Ubicación en la Gramática

```cosmos
light {
    from: @identificador_origen
    to: @identificador_destino
    flow: Primary
    restringido_hasta: 2027-12-31  // ← SOLO AQUÍ, no en elementos
}
```

### 6.4 Comportamiento

| Fase | Estado del Flujo |
|------|-----------------|
| **Antes de fecha límite** | Flujo PERMITIDO, funciona normalmente |
| **En fecha límite** | Último día de funcionamiento |
| **Después de fecha límite** | Flujo BLOQUEADO automáticamente (Error E401) |

### 6.5 Casos de Uso

```cosmos
// Caso 1: Transición gradual
// Permite usar API Legacy mientras se migra a nueva API
light {
    from: @cliente_legacy
    to: @api_moderna
    flow: Primary
    restringido_hasta: 2027-06-30  // Hasta que migración complete
}

// Caso 2: Licencia temporal
light {
    from: @sistema_principal
    to: @modulo_premium
    flow: Primary
    restringido_hasta: 2026-12-31
}

// Caso 3: Deprecación programada
light {
    from: @nuevo_servicio
    to: @old_adapter
    flow: Asynchronous
    restringido_hasta: 2026-03-01
}
```

### 6.6 Verificación de Caducidad

| Momento | Verificación |
|---------|-------------|
| **Compilación, modo strict** | Error E401 si `restringido_hasta < fecha_actual` |
| **Compilación, modo lax** | Warning W001 si `restringido_hasta < fecha_actual` |
| **Ejecución** | Bloqueo automático si `fecha_actual > restringido_hasta` |

```bash
# Compilación
$ cosmos compile proyecto.cosmos
Warning W001: Flow "@legacy_flow" expires on 2025-01-01 (already expired)

# Ejecución (si se intenta usar flujo expirado)
Error E401: Flow "@legacy_flow" blocked - authorization expired on 2025-01-01
```

### 6.7 Registro de Autorización Temporal

```
╔═══════════════════════════════════════════════════════════════════════╗
║              REGISTRO DE AUTORIZACIONES TEMPORALES              ║
╠═══════════════════════════════════════════════════════════════════════╣
║  Universe: MiSistema                                             ║
╠═══════════════════════════════════════════════════════════════════════╣
║  FLUJO                    │ DESDE    │ HASTA      │ FECHA_LÍMITE  ║
║  ─────────────────────────────────────────────────────────────────── ║
║  @ flujo_legacy          │ Orbit 4  │ Orbit 3    │ 2027-06-30   ║
║  @ licencia_modulo       │ Orbit 5  │ Orbit 4    │ 2026-12-31   ║
║  @ adapter_deprecated    │ Orbit 3  │ Orbit 2    │ ⚠️ 2025-01-01 ║
╠═══════════════════════════════════════════════════════════════════════╣
║  TOTAL FLUJOS RESTRINGIDOS: 3                                      ║
║  ACTIVOS: 2 | EXPIRADOS: 1                                        ║
╚═══════════════════════════════════════════════════════════════════════╝
```

### 6.8 Errores Relacionados

| Código | Nombre | Descripción |
|--------|--------|-------------|
| W001 | TemporalAccessExpired | Flujo con `restringido_hasta` ya vencido (warning en lax) |
| E401 | FlowAuthorizationExpired | Acceso denegado: fecha límite superada |
| E405 | MalformedDate | Fecha de `restringido_hasta` mal formada |

### 6.9 Notas de Implementación

1. **`restringido_hasta` NO anula la regla de órbitas**, solo la suspende temporalmente para ese flujo.
2. **La fecha se verifica en cada ejecución**, no solo en compilación, y contra la fecha real del sistema.
3. **Renovación**: para renovar, actualizar la fecha; para hacer permanente, remover el modificador.
4. **Múltiples flujos**: cada flujo tiene su propia fecha límite independiente.
5. Un flujo con `restringido_hasta` vencido sigue contando para el resto de las validaciones (existencia, dominio, ciclos); la caducidad es una comprobación adicional, no un sustituto.

---

## 7. Sistema de Tipos de Flujo

### 7.1 Tipos Disponibles

| Tipo | Símbolo | Descripción |
|------|---------|-------------|
| `Primary` | ➡️ | Flujo principal de ejecución |
| `Asynchronous` | ⏩ | Ejecución paralela, no bloqueante |
| `Read` | 📖 | Solo lectura de datos |

### 7.2 Comportamiento

```cosmos
light {
    from: @origen
    to: @destino
    flow: Primary  // Bloqueante, secuencial
}

light {
    from: @origen
    to: @destino
    flow: Asynchronous  // No bloqueante, paralelo
}

light {
    from: @origen
    to: @destino
    flow: Read  // Solo consulta, sin modificación
}
```

### 7.3 Matriz de interacción de flujo

> **Ningún permiso implícito. Lo que no está escrito está prohibido.**
> La matriz se aplica entre planetas. El Sol y los satélites están
> exentos (§5.2). Violarla es error E109.

| Emisor \ Receptor | Storage | Operation | Compute | Interface |
|-------------------|:-------:|:---------:|:-------:|:---------:|
| **Storage** | ❌ | ✅ | ✅ | ❌ |
| **Operation** | ❌ | ❌ | ✅ | ✅ |
| **Compute** | ❌ | ❌ | ❌ | ✅ |
| **Interface** | ❌ | ❌ | ❌ | ❌ |

Lectura: los datos salen del almacenamiento hacia operación o cálculo;
la operación alimenta cálculo o salida; el cálculo alimenta salida.
Nada fluye hacia atrás, nada sale de Interface, y Storage nunca va
directo a Interface: debe pasar por Operation o Compute.

---

## 8. Catálogo de Errores

Un mismo código, un solo significado, en todos lados. Los alias indican
el nombre que la misma condición tiene en la implementación Rust
(`cosmos-topology`, `src/lib.rs`): el compilador reporta el código
canónico de la primera columna.

### 8.1 Errores de Parser (P001–P015)

| Código | Nombre | Descripción |
|--------|--------|-------------|
| P001 | UnexpectedToken | Token inesperado en la posición actual |
| P002 | UnexpectedEOF | Fin de archivo inesperado |
| P003 | MissingUniverse | Falta declaración `universe` |
| P004 | MissingMetadata | Falta bloque `metadata` |
| P005 | MissingSun | Falta declaración `sun` |
| P006 | MissingPosition | Campo `position` obligatorio ausente |
| P007 | MissingMass | Campo `mass` obligatorio ausente |
| P008 | MissingType | Campo `type` obligatorio ausente |
| P009 | InvalidIdentifier | Identificador mal formado o referencia sin `@` |
| P010 | InvalidNumber | Número mal formado |
| P011 | InvalidString | Cadena mal formada |
| P012 | SunPositionOverride | Intento de declarar posición del Sol (G-001) |
| P013 | DuplicateEntity | Identificador duplicado (alias de E101) |
| P014 | UnknownBlock | Bloque desconocido (p. ej. `restringido_hasta` en un planeta) |
| P015 | InvalidFlowType | Tipo de flujo inválido (usar Primary, Asynchronous o Read) |

### 8.2 Errores Topológicos (E001–E007)

| Código | Nombre | Descripción | Gravedad |
|--------|--------|-------------|----------|
| E001 | OrbitViolation | Salto ilegal entre bandas: `usa` hacia órbita menor, o flujo Interface → Core | ❌ Bloqueante |
| E002 | MassCollision | Dos nodos a distancia menor que `EPSILON` (0.5) en la misma galaxia, o dos galaxias a distancia menor que `GALACTIC_EPSILON` (1.0) | ❌ Bloqueante |
| E003 | LightPathTooLong | Distancia del flujo supera `max_reach = 1000.0 / masa` del emisor | ❌ Bloqueante |
| E004 | SpaghettiTensionExceeded | Cruces entre flujos superan `TENSION_THRESHOLD` (10) | ⚠️ Warning |
| E005 | InvalidCoordinates | Radio negativo o ángulo no finito | ❌ Bloqueante |
| E006 | BidirectionalLightForbidden | Flujos A → B y B → A declarados | ❌ Bloqueante |
| E007 | DeepSpaceViolation | Entidad con radio > 100.0 | ❌ Bloqueante |

### 8.3 Errores Estructurales (E101–E112)

| Código | Nombre | Descripción | Gravedad |
|--------|--------|-------------|----------|
| E101 | DuplicateIdentifier | Identificador duplicado (alias S001, P013) | ❌ Bloqueante |
| E102 | MissingRequiredField | Campo obligatorio ausente | ❌ Bloqueante |
| E103 | InvalidEntityReference | Referencia a entidad inexistente (alias S002, S004, S005) | ❌ Bloqueante |
| E104 | InvalidMetadata | Metadatos inválidos o `standard` distinto de `"Cosmos"` | ❌ Bloqueante |
| E105 | SunOrStarAsReceiver | El Sol o una Star como receptor de flujo | ❌ Bloqueante |
| E106 | SatelliteScopeViolation | Satélite fuera de su galaxia, fuera de la banda de su padre, o con padre incorrecto (alias S003) | ❌ Bloqueante |
| E107 | GalaxyInCore | Galaxia con radio ≤ 10.0 | ❌ Bloqueante |
| E108 | MultipleSun | Múltiples Sun declarados | ❌ Bloqueante |
| E109 | FlowMatrixViolation | Flujo prohibido por la matriz de tipos (alias S006) | ❌ Bloqueante |
| E110 | OrbitTypeMismatch | `PlanetType` incompatible con su banda orbital (alias S008) | ❌ Bloqueante |
| E111 | SatelliteMassRatioExceeded | Masa satelital total > 50% de la masa del planeta (alias S007) | ❌ Bloqueante |
| E112 | EmptyUniverse | Universo sin galaxias (alias S009) | ❌ Bloqueante |

### 8.4 Errores Temporales y de Ciclo

| Código | Nombre | Descripción | Gravedad |
|--------|--------|-------------|----------|
| E401 | FlowAuthorizationExpired | Acceso denegado: fecha límite superada | ❌ Bloqueante |
| E403 | CrossGalaxyAccess | Acceso entre galaxias prohibido (alias S011) | ❌ Bloqueante |
| E404 | FlowCycle | Ciclo detectado en el grafo de flujo o de dependencias (alias S012) | ❌ Bloqueante |
| E405 | MalformedDate | Fecha de `restringido_hasta` mal formada | ❌ Bloqueante |
| W001 | TemporalAccessExpired | Flujo con `restringido_hasta` ya vencido (warning en modo lax) | ⚠️ Warning |

### 8.5 Errores de Contrato (RFC-0008)

| Código | Nombre | Descripción |
|--------|--------|-------------|
| CT001 | ContractNotFound | Contrato no existe |
| CT002 | ContractMismatch | Librería no cumple contrato |
| CT003 | PurityViolation | Modificación de estado puro |
| CT004 | SignatureMismatch | Firma exportada cambió |
| CT005 | ImportWithoutContract | Import sin contrato |

### 8.6 Errores CIR (representación binaria, RFC-0007 pendiente)

Reservados para la representación binaria canónica (CIR). El documento
RFC-0007 no existe todavía; los códigos están definidos en la
implementación Rust y se listan aquí para que no se reutilicen.

| Código | Nombre | Descripción |
|--------|--------|-------------|
| C001 | InvalidAst | AST inválido recibido |
| C002 | LightPathTooLong | Ruta de luz muy larga (distancia, alcance) |
| C003 | DeepSpaceViolation | Nodo en DeepSpace |
| C004 | MassCollision | Colisión de masa (nodos, distancia) |
| C005 | SpaghettiTension | Tensión excesiva (cruces, umbral) |
| C006 | InfiniteOrbitLoop | Bucle infinito sin salida (ciclo) |
| C007 | SerializationError | Error de serialización |
| C008 | HashMismatch | Hash no coincide (corrupción) |
| C009 | VersionMismatch | Versión incompatible |
| C010 | EntityNotFound | Entidad no encontrada |

### 8.7 Errores Semánticos del Rust (S001–S013)

Son los nombres que las mismas condiciones tienen en `SemanticError`
(Rust). No son códigos aparte: cada S tiene su código canónico en
las tablas anteriores.

| Código Rust | Condición | Código canónico |
|-------------|-----------|-----------------|
| S001 DuplicateNodeId | Identificador duplicado | E101 |
| S002 MissingReference | Referencia a nodo inexistente | E103 |
| S003 InvalidSatelliteParent | Satélite con padre incorrecto | E106 |
| S004 InvalidFlowSource | Flujo con origen inválido | E103 |
| S005 InvalidFlowTarget | Flujo con destino inválido | E103 |
| S006 FlowViolation | Matriz de interacción violada | E109 |
| S007 MassMismatch | Masa satelital excede 50% | E111 |
| S008 OrbitTypeMismatch | Tipo incompatible con banda | E110 |
| S009 EmptyUniverse | Universo sin galaxias | E112 |
| S010 InvalidPosition | Posición inválida | E005 |
| S011 GalacticIsolationViolation | Violación de aislamiento galáctico | E403 |
| S012 InfiniteOrbitLoop | Bucle infinito | E404 |
| S013 Topology | Error topológico genérico | E001–E007 |

---

## 9. Reglas de Validación

### 9.1 Reglas de Compilación

| Regla | Descripción | Errores |
|-------|-------------|---------|
| RV-001 | Todo Universe debe tener exactamente un Sun | E108, P005 |
| RV-002 | Todo Planet debe tener position, mass, type y @id | P006, P007, P008, E102 |
| RV-003 | Toda referencia debe usar `@identificador` | P009 |
| RV-004 | Las posiciones deben ser únicas dentro de una galaxia (distancia ≥ 0.5) | E002 |
| RV-005 | Los flujos y las dependencias no pueden crear ciclos | E404 |
| RV-006 | Los flujos deben respetar alcance, matriz de tipos y dominio | E003, E109, E403 |
| RV-007 | Las dependencias deben respetar la regla hacia adentro (§5.1) | E001 |
| RV-008 | El tipo de cada planeta debe admitir su banda | E110 |
| RV-009 | La masa satelital total no supera el 50% de la masa del planeta | E111 |
| RV-010 | Ningún planeta en banda Core; ninguna galaxia en Core ni en DeepSpace | E110, E107, E007 |
| RV-011 | El universo tiene al menos una galaxia | E112 |
| RV-012 | Cruces entre flujos: más de 10 es warning | E004 |

### 9.2 Reglas de Ejecución

| Regla | Descripción | Errores |
|-------|-------------|---------|
| RE-001 | Verificar `restringido_hasta` antes de cada flujo, contra la fecha real | E401 |
| RE-002 | Verificar alcance (`max_reach`) del emisor | E003 |
| RE-003 | Verificar que el receptor existe | E103 |

### 9.3 Modos de Validación

```bash
# Modo Strict (default)
cosmos compile --mode strict proyecto.cosmos
# ❌ Todos los errores bloquean la compilación

# Modo Lax
cosmos compile --mode lax proyecto.cosmos
# ⚠️ Todos los errores son warnings (salvo errores de parser)
```

Un error de parser (P001–P015) bloquea siempre, en cualquier modo:
sin parseo no hay nada que validar. En modo lax, los errores E y S se
reportan como warnings y la compilación continúa con exit 0.

---

## 10. Comparativa Cosmos vs Gestores Tradicionales

### 10.1 Tabla de Conflictos Resueltos

| Problema | npm/yarn/pnpm | pip/poetry/conda | Cosmos |
|----------|---------------|------------------|--------|
| Misma librería, 2 versiones | ❌ Override | ❌ Unsatisfiable | ✅ Por diseño |
| Accesos ambiguos | ⚠️ Warning | ⚠️ Warning | ❌ Error P009 |
| Múltiples Sun | N/A | N/A | ❌ Error E108 |
| Flujos cruzados | N/A | N/A | ❌ Error E403 |
| Ciclos en dependencias | ❌ Rompe | ❌ Rompe | ❌ Error E404 |
| Caducidad de licencias | ❌ Manual | ❌ Manual | ✅ Automático |

### 10.2 Principios de Diseño

| Principio | Implementación |
|-----------|----------------|
| **Unicidad** | Cada entidad tiene `@id` único (E101) |
| **Aislamiento** | Galaxies son dominios separados (E403) |
| **Determinismo** | Geometría = Programa (G-003) |
| **Temporalidad** | `restringido_hasta` con caducidad automática (E401) |

---

## 11. Ejemplo Completo

```cosmos
/*
 * Sistema de Demo Completo - Cosmos v1.0 (reescritura 2026-11-25)
 * Demuestra: Múltiples versiones, Órbitas, Restricción temporal
 * Este ejemplo compila VALID con el compilador de referencia.
 * Cada número está elegido para satisfacer las reglas; ver §11.1.
 */

universe "SistemaHibrido" {
    metadata {
        version: "1.0"
        author: "Luis Alberto Burciaga Miker"
        standard: "Cosmos"
    }

    // ============================================
    // ORIGEN ÚNICO DE VERDAD
    // ============================================
    sun NucleoSistema

    // ============================================
    // GALAXIA: Versión Legacy (ej: React 17)
    // ============================================
    galaxy "LegacyCore" @galaxy_legacy {
        position: (50.0, 0.0)

        // Logic - posición 25.0, masa 20 → alcance 50.0
        planet "ReactLegacy" @react_17_0_2 {
            position: (25.0, 0.0)
            mass: 20
            type: Operation
        }

        // Interface - posición 55.0, masa 100 → alcance 10.0
        planet "ComponenteLegacy" @comp_btn_legacy {
            position: (55.0, 0.0)
            mass: 100
            type: Interface
            usa @react_17_0_2  // ✅ Dependencia: Interface(3) → Logic(4)
        }
    }

    // ============================================
    // GALAXIA: Versión Moderna (ej: React 18)
    // ============================================
    galaxy "ModernCore" @galaxy_modern {
        position: (75.0, 1.57)  // Separación angular de Legacy

        // Logic - posición 25.0, masa 20 → alcance 50.0
        planet "ReactModerno" @react_18_3_1 {
            position: (25.0, 0.5)
            mass: 20
            type: Operation

            // Satellite: posición relativa (1.0, 0.0) → absoluta (26.0, 0.5)
            // Masa 5 ≤ 50% de 20. Hereda el contexto del padre.
            satellite "Hook" @custom_hook {
                position: (1.0, 0.0)
                mass: 5
                usa @react_18_3_1  // ✅ Usa a su propio padre
            }
        }

        // Interface - posición 55.0, masa 100 → alcance 10.0
        planet "ComponenteModerno" @comp_btn_modern {
            position: (55.0, 0.5)
            mass: 100
            type: Interface
            usa @react_18_3_1  // ✅ Dependencia: Interface(3) → Logic(4)
        }
    }

    // ============================================
    // FLUJOS: Aislamiento por dominio
    // ============================================

    // Flujo dentro de Legacy (licencia hasta 2027-12-31)
    light {
        from: NucleoSistema
        to: @react_17_0_2
        flow: Primary  // ✅ Sol → Logic, distancia 25.0 ≤ alcance 100
        restringido_hasta: 2027-12-31
    }

    light {
        from: @react_17_0_2
        to: @comp_btn_legacy
        flow: Primary  // ✅ Operation → Interface, distancia 30.0 ≤ alcance 50.0
    }

    // Flujo dentro de Modern
    light {
        from: NucleoSistema
        to: @react_18_3_1
        flow: Primary  // ✅ Sol → Logic, distancia 25.0 ≤ alcance 100
    }

    light {
        from: @react_18_3_1
        to: @comp_btn_modern
        flow: Asynchronous  // ✅ Operation → Interface, distancia 30.0 ≤ alcance 50.0
    }

    light {
        from: @react_18_3_1
        to: @custom_hook
        flow: Read  // ✅ Hacia satélite (exento de matriz), distancia 1.0 ≤ alcance 50.0
    }
}
```

### 11.1 Verificación del ejemplo (por qué compila)

| Comprobación | Cálculo | Resultado |
|--------------|---------|-----------|
| Un Sol | Solo `NucleoSistema` | ✅ E108 no aplica |
| Metadatos | version, author, standard `"Cosmos"` | ✅ E104 no aplica |
| Galaxias | 2 galaxias | ✅ E112 no aplica |
| Galaxia Legacy en Core | radio 50.0 > 10.0 | ✅ E107 no aplica |
| Galaxia Modern en DeepSpace | radio 75.0 ≤ 100.0 | ✅ E007 no aplica |
| Separación entre galaxias | distancia ≈ 90.0 ≥ 1.0 | ✅ E002 no aplica |
| Tipos y bandas | Operation en Logic, Interface en Interface | ✅ E110 no aplica |
| Masa satelital | 5 ≤ 50% de 20 = 10 | ✅ E111 no aplica |
| Satélite en banda del padre | absoluta (26.0, 0.5) → Logic, como el padre | ✅ E106 no aplica |
| Colisiones | distancia mínima entre nodos: 1.0 (react–hook) ≥ 0.5 | ✅ E002 no aplica |
| Alcance Sol → react | 25.0 ≤ 100 | ✅ E003 no aplica |
| Alcance react → comp | 30.0 ≤ 50.0 | ✅ E003 no aplica |
| Alcance react → hook | 1.0 ≤ 50.0 | ✅ E003 no aplica |
| Matriz de tipos | Operation → Interface ✅; Sol y satélite exentos | ✅ E109 no aplica |
| Dependencias | Interface(3) → Logic(4), hacia adentro | ✅ E001 no aplica |
| Dominio | todos los flujos dentro de su galaxia | ✅ E403 no aplica |
| Bidireccionalidad | ningún par A → B y B → A | ✅ E006 no aplica |
| Ciclos | grafos de flujo y de dependencia acíclicos | ✅ E404 no aplica |
| Cruces | 0 cruces ≤ 10 | ✅ E004 no aplica |
| Caducidad | 2027-12-31 > fecha actual | ✅ E401 no aplica |
| Identificadores | todos únicos | ✅ E101 no aplica |

```
RESULTADO: VALID ✅
```

---

## 12. Historial de Cambios

| Versión | Fecha | Cambio |
|---------|-------|--------|
| 1.0 | 2026-07-12 | Versión inicial congelada. |
| 1.0 (reescritura) | 2026-11-25 | Reescritura completa en su lugar, sin cambio de versión. Correcciones: D1 bandas 5 → 4 (coinciden con el Rust y con E007); D2 regla de acceso partida en `usa` (hacia adentro) y `light` (matriz + alcance); D3 eliminado el tipo de planeta `Core` (la banda Core es solo del Sol); D4 `mass` y `type` obligatorios; D5 `restringido_hasta` solo en flujos; D6 comentarios admitidos y `@` pegada al identificador; D7 masa del Sol fijada en 10 (alcance 100); D8 E401 queda solo como caducidad (la referencia sin `@` es P009); D9 fecha de caducidad contra fecha real; D10 constantes numéricas fijadas. Se elimina el enunciado de solicitud provisional de patente (no existe). Se agregan los códigos E109 (matriz), E110 (tipo–órbita), E111 (masa satelital), E112 (universo vacío) y el catálogo unificado P/E/S/C/W/CT. Ejemplo completo reescrito con números que satisfacen todas las reglas y verificación paso a paso (§11.1). |

---

## ESTADOS DE AUDITORÍA

| Observación | Estado |
|-------------|--------|
| G-001: Unicidad del Sun | ✅ RESUELTA |
| G-002: Separación Nombre/Id | ✅ RESUELTA (con excepción documentada del Sol) |
| G-003: Primacía Geométrica | ✅ APLICADA |
| G-004: Independencia de tipos externos | ✅ RESUELTA |
| G-005: Flujos vectoriales definidos | ✅ RESUELTA |
| G-006: Nebulous solo agrupadoras | ✅ RESUELTA |
| G-007: Metadatos obligatorios | ✅ RESUELTA |
| Orbit Access Rules | ✅ DEFINIDA (dos reglas: §5.1 y §5.2) |
| Restringido_hasta System | ✅ DEFINIDO |
| Constantes numéricas | ✅ DEFINIDAS (§4.3) |
| Catálogo de errores unificado | ✅ DEFINIDO (§8) |

---

> © 2026 Luis Alberto Burciaga Miker. Todos los derechos reservados.
> Este sistema y su lógica fundamental están protegidos por secreto comercial.
> Licencia: AGPL-3.0 + Excepción Comercial
