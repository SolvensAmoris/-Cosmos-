# RFC-0004-A: Gramática Formal del Lenguaje Cosmos

**Estado:** 📋 BORRADOR — Pendiente auditoría de Hans

**Versión:** 0.1

**Fecha:** 2026-07-12

**Autor:** Solvens (Arquitecto del Estándar)

---

## 1. Prefacio y Contexto

Este documento define la gramática formal del lenguaje Cosmos (archivos `.cosmos`). Establece la sintaxis canónica que todo parser debe implementar para garantizar interoperabilidad entre herramientas del ecosistema.

### 1.1 Relación con Otros RFC

| RFC | Título | Relación |
|-----|--------|----------|
| RFC-0001 | Fundamentos de Cosmos | Define las entidades topológicas |
| RFC-0002 | Modelo AST | Define la estructura de datos en memoria |
| RFC-0003 | Ley CIR-001 | Principio de Doble Representación |
| RFC-0004-A | **Este documento** | Gramática formal para archivos `.cosmos` |

### 1.2 Principios Fundacionales

#### G-003 — Primacía Geométrica (LEY)

> La geometría ES el programa, no lo representa.
> La órbita se CALCULA de la posición, nunca se declara.
> Mover una entidad CAMBIA el programa.

Esta ley es inmutable y aplica a toda la gramática.

#### Ley de No Redundancia Semántica

> Toda propiedad derivable de la geometría NO debe almacenarse como dato independiente.

**Prohibido:**
```cosmos
planet MiServicio {
    orbit: Core  // ❌ REDUNDANTE
}
```

**Permitido:**
```cosmos
planet MiServicio {
    position: { radius: 5.0, theta: 0.785 }
}
```

El compilador infiere `orbit: Core` de `radius: 5.0`.

---

## 2. Notación EBNF

### 2.1 Convenciones

| Símbolo | Significado |
|---------|-------------|
| `::=` | Definición de producción |
| `|` | Alternativa |
| `[ ]` | Opcional |
| `{ }` | Repetición (cero o más) |
| `" "` | Token literal |
| `*` | Comentario |
| `IDENTIFIER` | Letras, números, guiones bajos |
| `NUMBER` | Número decimal o entero |

### 2.2 Gramática Completa

```ebnf
cosmos_file        ::= universe_declaration
                      EOF

universe_declaration ::= "universe" IDENTIFIER "{" 
                          metadata_section
                          sun_declaration
                          { entity_declaration }
                          [ light_section ]
                          "}"

metadata_section   ::= "metadata" "{"
                        "version" ":" STRING
                        "author" ":" STRING
                        ["created" ":" STRING]
                        "standard" ":" "Cosmos"
                        "}"

sun_declaration    ::= "sun" IDENTIFIER

entity_declaration ::= galaxy_declaration
                     | nebula_declaration
                     | planet_declaration
                     | satellite_declaration
                     | star_declaration

galaxy_declaration ::= "galaxy" IDENTIFIER "{" 
                        { entity_declaration }
                        "}"

nebula_declaration ::= "nebula" IDENTIFIER "{" 
                        { planet_declaration | satellite_declaration | star_declaration }
                        "}"

planet_declaration ::= "planet" IDENTIFIER "{" 
                       position_declaration
                       "mass" ":" NUMBER
                       ["type" ":" IDENTIFIER]
                       { satellite_declaration }
                       "}"

satellite_declaration ::= "satellite" IDENTIFIER "{" 
                           relative_position_declaration
                           "mass" ":" NUMBER
                           ["type" ":" IDENTIFIER]
                           "}"

star_declaration   ::= "star" IDENTIFIER "{" 
                       position_declaration
                       "value" ":" NUMBER
                       "type" ":" IDENTIFIER
                       "}"

position_declaration ::= "position" ":" "{" 
                          "radius" ":" NUMBER
                          "theta" ":" NUMBER
                          "}"

relative_position_declaration ::= "relative_position" ":" "{" 
                                   "radius" ":" NUMBER
                                   "theta" ":" NUMBER
                                   "}"

light_section      ::= "light" "{"
                        { light_declaration }
                        "}"

light_declaration  ::= "from" ":" IDENTIFIER
                       "to" ":" IDENTIFIER
                       "flow" ":" flow_type
                       ["latency" ":" NUMBER]
                       ["security" ":" IDENTIFIER]
                       ["priority" ":" NUMBER]
                       ["energy_cost" ":" NUMBER]
                       "}"

flow_type          ::= "Primary" | "Asynchronous" | "Read"
```

---

## 3. Tokens Léxicos

### 3.1 Palabras Reservadas

```
universe, metadata, version, author, created, standard
sun
galaxy, nebula
planet, satellite, star
position, relative_position, radius, theta
mass, type, value
light, from, to, flow, latency, security, priority, energy_cost
Primary, Asynchronous, Read
Cosmos
```

### 3.2 Definición de Tokens

```ebnf
IDENTIFIER    ::= [a-zA-Z_][a-zA-Z0-9_]*
NUMBER        ::= [0-9]+("."[0-9]+)?
STRING        ::= '"' [^"]* '"'
COMMENT       ::= "//" [^\n]*
WHITESPACE    ::= [ \t\r\n]+
```

### 3.3 Convenciones de Nomenclatura

| Entidad | Convenciones |
|---------|--------------|
| `universe` | Singular, descriptivo (ej: "TradingUniverse", "AnalyticsCore") |
| `galaxy` | PascalCase, sustantivo (ej: "RiskManagement", "UserAuth") |
| `nebula` | PascalCase, sustantivo (ej: "Validators", "Repositories") |
| `planet` | PascalCase, verbo-sustantivo (ej: "ProcessOrder", "AuthenticateUser") |
| `satellite` | camelCase, verbo (ej: "validateInput", "parseToken") |
| `star` | SCREAMING_SNAKE_CASE (ej: "MAX_RETRY_COUNT", "DEFAULT_TIMEOUT") |

---

## 4. Estructura Sintáctica Detallada

### 4.1 Raíz del Archivo

```cosmos
universe "MiAplicacion" {
    // ...
}
```

**Regla G-001 aplicada:** El nombre del universe es un STRING, no tiene posición.

### 4.2 Metadatos Obligatorios

```cosmos
metadata {
    version: "1.0.0"
    author: "EquipoArquitectura"
    created: "2026-01-15"
    standard: "Cosmos"
}
```

| Campo | Obligatorio | Tipo | Validación |
|-------|-------------|------|------------|
| `version` | ✅ | STRING | SemVer recomendado |
| `author` | ✅ | STRING | No vacío |
| `created` | ⚠️ | STRING | Formato ISO 8601 |
| `standard` | ✅ | STRING | Literal "Cosmos" |

### 4.3 Declaración del Sol

```cosmos
sun Sol
```

**Regla G-001 — Sol Automático:**

```
❌ PROHIBIDO: sun Sol { position { radius: 0, theta: 0 } }
✅ REQUERIDO: sun Sol
```

El compilador asigna implícitamente `position = (radius: 0.0, theta: 0.0)`. Declarar la posición del Sol es un **OrbitViolation**.

### 4.4 EntidadesGeométricas

#### 4.4.1 Galaxy

```cosmos
galaxy MiGalaxia {
    nebula Contabilidad {
        planet ProcesarTransaccion
        planet GenerarReporte
    }
    
    nebula Reportes {
        planet ExportarDatos
    }
}
```

- `galaxy` tiene **posición propia**
- `nebula` NO tiene posición (organizador visual)
- Puede contener `nebula`, `planet`, `satellite`, `star`

#### 4.4.2 Planet

```cosmos
planet ProcesarOrden {
    position: { radius: 25.0, theta: 1.57 }
    mass: 150
    type: Service
    
    satellite validarItems {
        relative_position: { radius: 2.0, theta: 0.5 }
        mass: 10
        type: Validator
    }
}
```

| Campo | Obligatorio | Derivado | Notas |
|-------|-------------|----------|-------|
| `position` | ✅ | — | Define órbita automáticamente |
| `mass` | ✅ | — | u32, afecta max_reach |
| `type` | ⚠️ | — | Opcional, para documentación |
| `satellite` | ⚠️ | — | Cero o más |

#### 4.4.3 Satellite

```cosmos
satellite calcularImpuesto {
    relative_position: { radius: 1.5, theta: 0.3 }
    mass: 5
}
```

| Campo | Obligatorio | Derivado | Notas |
|-------|-------------|----------|-------|
| `relative_position` | ✅ | `absolute_position` (hereda del planeta) | Siempre relativo |
| `mass` | ✅ | — | u32 |
| `type` | ⚠️ | — | Opcional |

**Cálculo de posición absoluta:**
```
absolute_position = planet.position + relative_position
```

#### 4.4.4 Star

```cosmos
star MAX_RETRY_COUNT {
    position: { radius: 8.0, theta: 0.0 }
    value: 3
    type: Integer
}
```

| Campo | Obligatorio | Derivado | Notas |
|-------|-------------|----------|-------|
| `position` | ✅ | — | Define ubicación |
| `value` | ✅ | — | Constante literal |
| `type` | ✅ | — | Tipo de dato |

**Regla:** Las Stars son inmutables. No participan en flujo de ejecución (ver RFC-0004-B).

---

## 5. Sistema de Tipos (G-003 Aplicado)

**Principio:** `type` define el TIPO DE DATO, NO la órbita.

| type | Descripción | Valores típicos |
|------|-------------|-----------------|
| `Integer` | Entero con signo | 1, 42, -100 |
| `Float` | Punto flotante | 3.14, -0.001 |
| `String` | Texto | "Hola mundo" |
| `Boolean` | Verdadero/Falso | true, false |
| `Service` | Servicio ejecutable | N/A |
| `Validator` | Validador puro | N/A |
| `Repository` | Acceso a datos | N/A |

### 5.1 Tabla de Campos por Entidad

| Entidad | Campos obligatorios | Campos inferidos |
|---------|---------------------|------------------|
| `sun` | nombre | position=(0,0), tipo=Origin |
| `galaxy` | nombre, position | N/A |
| `nebula` | nombre | N/A (sin posición) |
| `planet` | position, mass | orbit (de radius) |
| `satellite` | relative_position, mass | absolute_position (de planeta) |
| `star` | position, value, type | N/A (constante) |

---

## 6. Sección de Luz (Conexiones)

### 6.1 Sintaxis

```cosmos
light {
    from: EntidadOrigen
    to: EntidadDestino
    flow: Primary
    
    latency: 10
    security: encrypted
    priority: high
    energy_cost: 5
}
```

### 6.2 Propiedades

| Propiedad | Obligatorio | Tipo | Notas |
|-----------|-------------|------|-------|
| `from` | ✅ | IDENTIFIER | Nodo emisor |
| `to` | ✅ | IDENTIFIER | Nodo receptor |
| `flow` | ✅ | Primary\|Asynchronous\|Read | Tipo de flujo |
| `latency` | ⚠️ | NUMBER | Milisegundos (opcional) |
| `security` | ⚠️ | IDENTIFIER | Niveles de seguridad |
| `priority` | ⚠️ | IDENTIFIER | high, medium, low |
| `energy_cost` | ⚠️ | NUMBER | Unidades de ejecución |

### 6.3 Tipos de Flujo

| Tipo | Descripción | ¿Bidireccional? |
|------|-------------|------------------|
| `Primary` | Flujo principal de ejecución | No (ver C-003) |
| `Asynchronous` | Ejecución paralela | No |
| `Read` | Solo lectura de datos | No |

**Ley C-003:** No existen conexiones bidireccionales. `A ⇄ B` se representa como:
```cosmos
light { from: A, to: B, flow: Primary }
light { from: B, to: A, flow: Read }
```

---

## 7. Ejemplo Completo de Referencia

```cosmos
universe "SistemaPedidos" {
    metadata {
        version: "1.0.0"
        author: "ArquitecturaCorp"
        created: "2026-07-12"
        standard: "Cosmos"
    }
    
    sun Sol
    
    galaxy GestionPedidos {
        nebula Core {
            planet ProcesarOrden {
                position: { radius: 25.0, theta: 0.0 }
                mass: 200
                type: Service
                
                satellite validarStock {
                    relative_position: { radius: 2.0, theta: 0.5 }
                    mass: 15
                    type: Validator
                }
            }
            
            planet GestionarInventario {
                position: { radius: 28.0, theta: 1.2 }
                mass: 180
                type: Service
            }
        }
        
        star MAX_RETRY_ATTEMPTS {
            position: { radius: 5.0, theta: 0.785 }
            value: 3
            type: Integer
        }
    }
    
    galaxy Notificaciones {
        nebula Alertas {
            planet EnviarEmail {
                position: { radius: 55.0, theta: 2.5 }
                mass: 80
                type: Service
            }
        }
    }
    
    light {
        from: ProcesarOrden
        to: GestionarInventario
        flow: Primary
        priority: high
    }
    
    light {
        from: GestionarInventario
        to: ProcesarOrden
        flow: Read
    }
    
    light {
        from: ProcesarOrden
        to: EnviarEmail
        flow: Asynchronous
        latency: 100
    }
}
```

---

## 8. Tokens Reservados y Conflictos

### 8.1 Lista Completa

```
universe, metadata, version, author, created, standard,
sun, galaxy, nebula, planet, satellite, star,
position, relative_position, radius, theta, mass, type, value,
light, from, to, flow, latency, security, priority, energy_cost,
Primary, Asynchronous, Read,
Cosmos,
true, false,
Infinity, NaN
```

### 8.2 Palabras Reservadas No Utilizables como Identificadores

- Todos los tokens listados arriba
- Identificadores que empiecen con `_cosmos_` (reservados para uso interno)

---

## 9. Errores Léxicos

| Código | Descripción | Ejemplo |
|--------|-------------|---------|
| L001 | Carácter inválido | `@`, `#`, `$` en posición válida |
| L002 | Número mal formado | `3.14.15`, `1e10.5` |
| L003 | Cadena no terminada | `"Hola mundo` (sin cierre) |
| L004 | Identificador con caracteres inválidos | `mi-variable` (guión no permitido) |

---

## 10. Historial de Cambios

| Versión | Fecha | Autor | Cambios |
|---------|-------|-------|---------|
| 0.1 | 2026-07-12 | Solvens | Borrador inicial |

---

## 11. Estados de Auditoría

| Observación | Estado | Notas |
|-------------|--------|-------|
| G-001 | ✅ RESUELTA | Sol automático, sin posición manual |
| G-002 | ✅ RESUELTA | Nombre vs NodeId separados |
| G-003 | ✅ APLICADA | Primacía geométrica en toda la gramática |
| G-004 | ✅ RESUELTA | Sistema de tipos sin redundancia |
| G-005 | ✅ RESUELTA | Conexiones extensibles en bloque `light` |
| G-006 | ✅ RESUELTA | Nebulas como organizadores sin posición |
| G-007 | ✅ RESUELTA | Metadatos obligatorios definidos |

---

**Documento preparado para auditoría de Hans.**
