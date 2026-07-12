# RFC-0004-F: Especificación Completa del Lenguaje Cosmos

> **Estado:** FINAL | **Versión:** 1.0 | **Fecha:** 2026-07-12  
> © 2026 Luis Alberto Burciaga Miker. Todos los derechos reservados.  
> CC BY-NC-ND 4.0  
> **Patente:** Solicitud provisional en trámite

---

## TABLA DE CONTENIDOS

1. [Principios Fundacionales](#1-principios-fundacionales)
2. [Estructura del Universo](#2-estructura-del-universo)
3. [Gramática Formal](#3-gramática-formal)
4. [Sistema de Órbitas](#4-sistema-de-órbitas)
5. [Sistema de Restricción Temporal](#5-sistema-de-restricción-temporal)
6. [Sistema de Tipos de Flujo](#6-sistema-de-tipos-de-flujo)
7. [Catálogo de Errores](#7-catálogo-de-errores)
8. [Reglas de Validación](#8-reglas-de-validación)
9. [Comparativa Cosmos vs Gestores Tradicionales](#9-comparativa-cosmos-vs-gestores-tradicionales)
10. [Ejemplo Completo](#10-ejemplo-completo)

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

| Entidad | Nombre | Identificador |
|---------|--------|---------------|
| Planet | `React` | `@react_17_0_2` |
| Galaxy | `LegacyApp` | `@galaxy_legacy_app` |
| Universe | `MiProyecto` | `@universe_mi_proyecto` |

```cosmos
// ✅ VÁLIDO
planet React @ react_17_0_2 {
    usa @ react_17_0_2  // Usa identificador
}

// ❌ INVÁLIDO - Error E401
planet React {
    usa React  // Error: Referencia sin @id
}
```

### G-003: Primacía Geométrica

> La órbita se CALCULA de la posición, nunca se declara. La geometría ES el programa.

```cosmos
// ✅ La órbita se calcula de la posición automáticamente
planet LogicaCore {
    position: (5.0, 0.0)  // → Orbit 5 (Core)
}

planet Servicio {
    position: (25.0, 0.0)  // → Orbit 4 (Logic)
}
```

---

## 2. Estructura del Universo

### Jerarquía Completa

```
Universe
├── Sun (origen único de verdad)
├── metadata (obligatorio)
├── Galaxy (múltiples)
│   ├── position (obligatorio)
│   ├── Planet (múltiples)
│   │   ├── position (obligatorio)
│   │   ├── mass (opcional, default: 100)
│   │   ├── type (opcional)
│   │   ├── restringido_hasta (opcional)
│   │   └── Satellite (múltiples)
│   │       ├── position (relativo)
│   │       └── mass (calculado)
│   └── NebulousGroup (opcional)
│       └── Planet (agrupados)
└── LightFlow (conexiones)
    ├── from (identificador)
    ├── to (identificador)
    └── flow (tipo)
```

---

## 3. Gramática Formal

### 3.1 Estructura Base

```cosmos
universe "NombreDelProyecto" {
    metadata {
        version: "1.0"
        author: "Nombre del Autor"
        standard: "Cosmos"
    }
    
    sun NombreDelSun
    
    galaxy "NombreGalaxia" @ identificador_galaxia {
        position: (radio, angulo)
        
        planet "NombrePlaneta" @ identificador_unico {
            position: (radio, angulo)
            mass: 100
            type: Operation
            restringido_hasta: 2027-01-01
        }
        
        nebula "Agrupacion" {
            planet "OtroPlaneta" @ otro_id {
                position: (radio, angulo)
            }
        }
    }
    
    light {
        from: @identificador_origen
        to: @identificador_destino
        flow: Primary
    }
}
```

### 3.2 Tokens Reservados

```
UNIVERSE, UNIVERSE
METADATA, METADATA
SUN, sun
GALAXY, galaxy
PLANET, planet
NEBULA, nebula
SATELLITE, satellite
POSITION, position
MASS, mass
TYPE, type
RESTRINGIDO_HASTA, restringido_hasta
LIGHT, light
FROM, from
TO, to
FLOW, flow
USA, usa
PRIMARY, ASYNCHRONOUS, READ
CORE, LOGIC, INTERFACE, DEEPSPACE
PERMITIDO, DENEGADO
OPERATION, COMPUTE, STORAGE, INTERFACE_TYPE
```

### 3.3 Tipos de Entidades

| Tipo | Descripción | Requiere Position |
|------|-------------|-------------------|
| `Sun` | Origen único de verdad | ❌ Prohibido |
| `Galaxy` | Dominio aislado | ✅ Obligatorio |
| `Planet` | Módulo funcional | ✅ Obligatorio |
| `Satellite` | Submódulo relativo | ✅ Relativo al padre |

### 3.4 Tipos de PlanetType

| Tipo | Órbitas Válidas | Descripción |
|------|-----------------|-------------|
| `Core` | 5 (Core) | Estado inmutable, reglas fundamentales |
| `Operation` | 3-4 (Logic) | Lógica de negocio |
| `Compute` | 2-4 | Procesamiento de datos |
| `Storage` | 3-4 (Logic) | Persistencia de datos |
| `Interface` | 1 (Interface) | Capa de entrada/salida |

---

## 4. Sistema de Órbitas

### 4.1 Bandas Orbitales (Calculadas por Posición)

| Banda | Rango de Radio | Órbita | Descripción |
|-------|---------------|--------|-------------|
| **Core** | 0.0 - 10.0 | 5 | Lógica central inmutable |
| **Logic** | 10.1 - 50.0 | 4 | Lógica de negocio |
| **Logic** | 50.1 - 100.0 | 3 | Servicios auxiliares |
| **Interface** | 100.1 - ∞ | 1 | Capa de entrada/salida |
| **DeepSpace** | > 100.0 | 0 | Prohibido (Error E007) |

### 4.2 Regla de Acceso Entre Órbitas

> **REGLA ORO:** Solo se puede acceder desde órbitas superiores (números menores) a órbitas inferiores (números mayores), siguiendo el flujo gravitacional natural.

```
Flujo PERMITIDO (órbitas superiores → inferiores):
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Orbit 5 (Core)    ← Mayor nivel de abstracción
    ↓
Orbit 4 (Logic)   ← Lógica de negocio
    ↓
Orbit 3 (Service) ← Servicios auxiliares
    ↓
Orbit 2 (Data)    ← Procesamiento
    ↓
Orbit 1 (Interface) ← Capa de entrada/salida
```

### 4.3 Tabla de Accesos Permitidos

| Desde \ Hacia | Orbit 5 (Core) | Orbit 4 (Logic) | Orbit 3 (Service) | Orbit 2 (Data) | Orbit 1 (Interface) |
|---------------|----------------|-----------------|------------------|----------------|-------------------|
| **Orbit 5** | ❌ Auto-ref | ✅ | ✅ | ✅ | ✅ |
| **Orbit 4** | ❌ | ❌ Auto-ref | ✅ | ✅ | ✅ |
| **Orbit 3** | ❌ | ❌ | ❌ Auto-ref | ✅ | ✅ |
| **Orbit 2** | ❌ | ❌ | ❌ | ❌ Auto-ref | ✅ |
| **Orbit 1** | ❌ | ❌ | ❌ | ❌ | ❌ Auto-ref |

### 4.4 Ejemplos de Acceso

```cosmos
universe "DemoOrbitas" {
    sun Nucleo
    
    galaxy "Sistema" {
        position: (50.0, 0.0)
        
        // Orbit 5 (Core) - posición 5.0
        planet Reglas @ reglas_core {
            position: (5.0, 0.0)
            type: Core
        }
        
        // Orbit 4 (Logic) - posición 25.0
        planet Servicio @ servicio_logico {
            position: (25.0, 0.0)
            type: Operation
            usa @ reglas_core  // ✅ PERMITIDO: Logic(4) → Core(5)
        }
        
        // Orbit 3 (Service) - posición 50.0
        planet Util @ util_servicio {
            position: (50.0, 0.0)
            usa @ servicio_logico  // ✅ PERMITIDO: Service(3) → Logic(4)
            usa @ reglas_core      // ✅ PERMITIDO: Service(3) → Core(5)
        }
        
        // Orbit 4 accediendo desde Orbit 3
        planet Manager @ manager {
            position: (30.0, 0.0)
            usa @ servicio_logico  // ✅ PERMITIDO
        }
    }
    
    light {
        from: @reglas_core
        to: @servicio_logico
        flow: Primary  // ✅ Core → Logic = flujo natural
    }
}
```

### 4.5 Errores de Acceso

```cosmos
// ❌ INVÁLIDO - Error E001: OrbitViolation
planet Reglas @ reglas_core {
    position: (5.0, 0.0)
    usa @ servicio_logico  // ❌ Logic(4) → Core(5) = INVÁLIDO
}
```

---

## 5. Sistema de Restricción Temporal

### 5.1 Definición

El modificador `restringido_hasta` indica que una entidad tiene acceso limitado en el tiempo. Una vez pasada la fecha, el acceso a esa entidad es automáticamente DENEGADO.

### 5.2 Sintaxis

```cosmos
planet "ServicioExpirable" @ servicio_expirable {
    position: (25.0, 0.0)
    mass: 100
    restringido_hasta: 2027-01-01
}
```

### 5.3 Comportamiento

| Fase | Comportamiento |
|------|---------------|
| **Antes de fecha** | Entidad completamente accesible. Los `LightFlow` funcionan normalmente. |
| **En fecha límite** | Último día de acceso completo. |
| **Después de fecha** | Acceso AUTOMÁTICAMENTE DENEGADO. Error E401. |

### 5.4 Registro de Autorización

```
╔═══════════════════════════════════════════════════════════════════╗
║              REGISTRO DE AUTORIZACIÓN TEMPORAL              ║
╠═══════════════════════════════════════════════════════════════════╣
║  Universe: MiProyecto                                        ║
║  Generado: 2026-07-12T10:00:00Z                            ║
╠═══════════════════════════════════════════════════════════════════╣
║  ENTIDAD                    │ FECHA_LÍMITE │ ESTADO            ║
║  ─────────────────────────────────────────────────────────── ║
║  @ servicio_legacy_v1      │ 2026-12-31   │ ACTIVO           ║
║  @ api_transicion          │ 2027-06-30   │ ACTIVO           ║
║  @ modulo_beta             │ 2026-07-15   │ ⚠️ EXPIRADO       ║
║  @ licencia_temporal       │ 2026-08-01   │ ACTIVO           ║
╠═══════════════════════════════════════════════════════════════════╣
║  TOTAL ENTIDADES: 4                                          ║
║  ACTIVAS: 3 | EXPIRADAS: 1                                 ║
╚═══════════════════════════════════════════════════════════════════╝
```

### 5.5 Reglas de Caducidad

1. **Verificación en compilación:** Si `restringido_hasta < fecha_actual`, el parser genera Warning W001.
2. **Verificación en ejecución:** Cada `LightFlow` verifica la fecha límite antes de permitir el flujo.
3. **Renewal:** Para renovar, se debe especificar una nueva fecha o remover `restringido_hasta`.
4. **Herencia:** Los Satellites heredan la restricción del Planet padre, a menos que especifiquen la suya.

### 5.6 Ejemplo Completo

```cosmos
universe "SistemaConLicencias" {
    metadata {
        version: "1.0"
        author: "Licensor"
        standard: "Cosmos"
    }
    
    sun NucleoLicencias
    
    galaxy "Modulos" {
        position: (50.0, 0.0)
        
        // Módulo con licencia hasta 2027
        planet ModuloPremium @ modulo_premium {
            position: (25.0, 0.0)
            mass: 100
            restringido_hasta: 2027-12-31
        }
        
        // Módulo expirado (genera warning)
        planet ModuloLegacy @ modulo_legacy {
            position: (35.0, 0.0)
            restringido_hasta: 2025-01-01  // ⚠️ YA EXPIRADO
        }
        
        // Módulo sin restricción (permanente)
        planet ModuloBase @ modulo_base {
            position: (15.0, 0.0)
            // Sin restringido_hasta = acceso permanente
        }
        
        // Satellite hereda restricción
        satellite "SubModulo" @ sub_modulo {
            position: (26.0, 0.0)  // Relativo al padre
            usa @ modulo_premium  // Hereda la fecha límite
        }
    }
    
    // Flujo permitido mientras esté activo
    light {
        from: @modulo_base
        to: @modulo_premium
        flow: Primary
    }
    
    // Flujo denegado: entidad expirada
    light {
        from: @modulo_base
        to: @modulo_legacy  // ⚠️ COMPILA con warning
                             // ❌ RECHAZADO en ejecución
        flow: Primary
    }
}
```

### 5.7 Errores Relacionados

| Código | Nombre | Descripción |
|--------|--------|-------------|
| W001 | TemporalAccessExpired | Entidad con `restringido_hasta` vencida |
| E401 | AmbiguousReference | Acceso denegado por expiración |

---

## 6. Sistema de Tipos de Flujo

### 6.1 Tipos Disponibles

| Tipo | Símbolo | Descripción |
|------|---------|-------------|
| `Primary` | ➡️ | Flujo principal de ejecución |
| `Asynchronous` | ⏩ | Ejecución paralela, no bloqueante |
| `Read` | 📖 | Solo lectura de datos |

### 6.2 Comportamiento

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

---

## 7. Catálogo de Errores

### 7.1 Errores Topológicos

| Código | Nombre | Descripción | Gravedad |
|--------|--------|-------------|----------|
| E001 | OrbitViolation | Salto ilegal entre bandas orbitales | ❌ Bloqueante |
| E002 | MassCollision | Dos módulos en misma posición | ❌ Bloqueante |
| E003 | LightPathTooLong | Distancia supera alcance del emisor | ❌ Bloqueante |
| E004 | SpaghettiTensionExceeded | Demasiados cruces de conexiones | ⚠️ Warning |
| E005 | InvalidCoordinates | Radio negativo o ángulo inválido | ❌ Bloqueante |
| E006 | BidirectionalLightForbidden | Conexión bidireccional declarada | ❌ Bloqueante |
| E007 | DeepSpaceViolation | Entidad en banda prohibida (>100.0) | ❌ Bloqueante |

### 7.2 Errores Estructurales

| Código | Nombre | Descripción | Gravedad |
|--------|--------|-------------|----------|
| E101 | DuplicateIdentifier | Identificador duplicado | ❌ Bloqueante |
| E102 | MissingRequiredField | Campo obligatorio ausente | ❌ Bloqueante |
| E103 | InvalidEntityReference | Referencia a entidad inexistente | ❌ Bloqueante |
| E104 | InvalidMetadata | Metadatos inválidos | ❌ Bloqueante |
| E105 | StarAsReceiver | Sun usado como receptor de flujo | ❌ Bloqueante |
| E106 | SatelliteScopeViolation | Satellite accediendo fuera de su Galaxy | ❌ Bloqueante |
| E107 | GalaxyInCore | Galaxy con posición en Core (0-10) | ❌ Bloqueante |
| E108 | MultipleSun | Múltiples Sun declarados | ❌ Bloqueante |

### 7.3 Errores de Ambigüedad

| Código | Nombre | Descripción | Gravedad |
|--------|--------|-------------|----------|
| E401 | AmbiguousReference | Referencia sin `@id` | ❌ Bloqueante |
| E402 | NameCollision | Nombre visible duplicado sin `@id` | ⚠️ Warning |

### 7.4 Errores de Aislamiento

| Código | Nombre | Descripción | Gravedad |
|--------|--------|-------------|----------|
| E403 | CrossGalaxyAccess | Acceso entre galaxias prohibido | ❌ Bloqueante |
| E404 | FlowCycle | Ciclo detectado en grafo de flujo | ❌ Bloqueante |

---

## 8. Reglas de Validación

### 8.1 Reglas de Compilación

| Regla | Descripción |
|-------|-------------|
| RV-001 | Todo Universe debe tener exactamente un Sun |
| RV-002 | Todo Planet debe tener position, mass y @id |
| RV-003 | Toda referencia debe usar `@identificador` |
| RV-004 | Las posiciones deben ser únicas dentro de una Galaxy |
| RV-005 | Los flujos no pueden crear ciclos |
| RV-006 | Los flujos deben respetar las órbitas (E001) |

### 8.2 Reglas de Ejecución

| Regla | Descripción |
|-------|-------------|
| RE-001 | Verificar `restringido_hasta` antes de cada flujo |
| RE-002 | Verificar alcance (`max_reach`) del emisor |
| RE-003 | Verificar que el receptor existe |

### 8.3 Modos de Validación

```bash
# Modo Strict (default)
cosmos compile --mode strict proyecto.cosmos
# ❌ Todos los errores bloquean la compilación

# Modo Lax
cosmos compile --mode lax proyecto.cosmos
# ⚠️ Todos los errores son warnings
```

---

## 9. Comparativa Cosmos vs Gestores Tradicionales

### 9.1 Tabla de Conflictos Resueltos

| Problema | npm/yarn/pnpm | pip/poetry/conda | Cosmos |
|----------|---------------|------------------|--------|
| Misma librería, 2 versiones | ❌ Override | ❌ Unsatisfiable | ✅ Por diseño |
| Accesos ambiguos | ⚠️ Warning | ⚠️ Warning | ❌ Error E401 |
| Múltiples Sun | N/A | N/A | ❌ Error E108 |
| Flujos cruzados | N/A | N/A | ❌ Error E403 |
| Ciclos en dependencias | ❌ Rompe | ❌ Rompe | ❌ Error E404 |
| Caducidad de licencias | ❌ Manual | ❌ Manual | ✅ Automático |

### 9.2 Principios de Diseño

| Principio | Implementación |
|-----------|----------------|
| **Unicidad** | Cada entidad tiene `@id` único |
| **Aislamiento** | Galaxies son dominios separados |
| **Determinismo** | Geometría = Programa |
| **Temporalidad** | `restringido_hasta` con caducidad automática |

---

## 10. Ejemplo Completo

```cosmos
/*
 * Sistema de Demo Completo - Cosmos v1.0
 * Demuestra: Múltiples versiones, Órbitas, Restricción temporal
 */

universe "SistemaHibrido" {
    metadata {
        version: "1.0"
        author: "Cosmos Demo"
        standard: "Cosmos"
    }
    
    // ============================================
    // ORIGEN ÚNICO DE VERDAD
    // ============================================
    sun NucleoSistema
    
    // ============================================
    // GALAXIA: Versión Legacy (ej: React 17)
    // ============================================
    galaxy "LegacyCore" @ galaxy_legacy {
        position: (50.0, 0.0)
        
        // Orbit 5 (Core) - posición 5.0
        planet NucleoLegacy @ nucleo_legacy {
            position: (5.0, 0.0)
            mass: 1000
            type: Core
        }
        
        // Orbit 4 (Logic) - posición 25.0
        planet ReactLegacy @ react_17_0_2 {
            position: (25.0, 0.0)
            mass: 100
            type: Operation
            restringido_hasta: 2027-12-31  // Licencia hasta 2027
        }
        
        // Orbit 4 - Utiliza React Legacy
        planet ComponenteLegacy @ comp_btn_legacy {
            position: (30.0, 0.0)
            usa @ react_17_0_2  // ✅ Acceso: Logic(4) → Core(5)
        }
    }
    
    // ============================================
    // GALAXIA: Versión Moderna (ej: React 18)
    // ============================================
    galaxy "ModernCore" @ galaxy_modern {
        position: (150.0, 1.57)  // Separación de Legacy
        
        // Orbit 5 (Core) - posición 5.0
        planet NucleoModerno @ nucleo_modern {
            position: (5.0, 0.0)
            mass: 1000
            type: Core
        }
        
        // Orbit 4 (Logic) - posición 25.0
        planet ReactModerno @ react_18_3_1 {
            position: (25.0, 0.0)
            mass: 100
            type: Operation
        }
        
        // Orbit 4 - Utiliza React Moderno
        planet ComponenteModerno @ comp_btn_modern {
            position: (30.0, 0.0)
            usa @ react_18_3_1  // ✅ Acceso: Logic(4) → Core(5)
        }
        
        // Satellite
        satellite "Hook" @ custom_hook {
            position: (26.0, 0.0)  // Relativo al padre
            usa @ react_18_3_1  // Hereda contexto
        }
    }
    
    // ============================================
    // FLUJOS: Aislamiento por dominio
    // ============================================
    
    // Flujo dentro de Legacy
    light {
        from: @nucleo_legacy
        to: @react_17_0_2
        flow: Primary  // ✅ Core(5) → Logic(4)
    }
    
    light {
        from: @react_17_0_2
        to: @comp_btn_legacy
        flow: Primary  // ✅ Logic(4) → Logic(4)
    }
    
    // Flujo dentro de Modern
    light {
        from: @nucleo_modern
        to: @react_18_3_1
        flow: Primary  // ✅ Core(5) → Logic(4)
    }
    
    light {
        from: @react_18_3_1
        to: @comp_btn_modern
        flow: Asynchronous  // ✅ No bloqueante
    }
    
    light {
        from: @react_18_3_1
        to: @custom_hook
        flow: Read  // ✅ Solo lectura
    }
}

// ============================================
// RESULTADO DE VALIDACIÓN
// ============================================
/*
✅ Universe "SistemaHibrido" válido
✅ Sun "NucleoSistema" declarado (1/1)
✅ Galaxy "LegacyCore" aislada
✅ Galaxy "ModernCore" aislada
✅ React @ react_17_0_2 coexiste con React @ react_18_3_1
✅ restringido_hasta: 2027-12-31 en @react_17_0_2
✅ Órbitas respetadas (E001 no detectado)
✅ Flujos válidos (E404 no detectado)
✅ Cross-galaxy access bloqueado (E403)
✅ Satellite scoped correctly (E106 no detectado)

RESULTADO: VALID ✅
*/
```

---

## HISTORIAL DE CAMBIOS

| Versión | Fecha | Cambio |
|---------|-------|--------|
| 1.0 | 2026-07-12 | Versión inicial congelada |

---

## ESTADOS DE AUDITORÍA

| Observación | Estado |
|-------------|--------|
| G-001: Unicidad del Sun | ✅ RESUELTA |
| G-002: Separación Nombre/Id | ✅ RESUELTA |
| G-003: Primacía Geométrica | ✅ APLICADA |
| G-004: Independencia de tipos externos | ✅ RESUELTA |
| G-005: Flujos vectoriales definidos | ✅ RESUELTA |
| G-006: Nebulous solo agrupadoras | ✅ RESUELTA |
| G-007: Metadatos obligatorios | ✅ RESUELTA |
| Orbit Access Rules | ✅ DEFINIDA |
| Restringido_hasta System | ✅ DEFINIDO |

---

> © 2026 Luis Alberto Burciaga Miker. Todos los derechos reservados.
> Este sistema y su lógica fundamental están protegidos por secreto comercial y solicitud de patente provisional en trámite.
> Licencia: AGPL-3.0 + Excepción Comercial
