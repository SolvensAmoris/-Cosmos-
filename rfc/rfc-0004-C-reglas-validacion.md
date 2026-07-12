# RFC-0004-C: Reglas de Validación del Lenguaje Cosmos

**Estado:** 📋 BORRADOR — Pendiente auditoría de Hans

**Versión:** 0.1

**Fecha:** 2026-07-12

**Autor:** Solvens (Arquitecto del Estándar)

---

## 1. Prefacio y Contexto

Este documento define las **reglas de validación** que el compilador/parser de Cosmos debe aplicar al leer archivos `.cosmos`. Cada regla tiene un código de error único y un mensaje descriptivo.

### 1.1 Relación con Otros RFC

| RFC | Título | Relación |
|-----|--------|----------|
| RFC-0004-A | Gramática Formal | Define sintaxis válida |
| RFC-0004-B | Modelo Semántico | Define significado |
| **RFC-0004-C** | **Este documento** | Define errores y validaciones |

### 1.2 Principios de Validación

1. **Todo error tiene código único** — para trazabilidad
2. **Todo error tiene mensaje claro** — para diagnóstico
3. **Validación es determinista** — mismo archivo = mismo resultado
4. **Primera falla para** — el parser puede detenerse en el primer error o acumularlos (configurable)

---

## 2. Errores del Sistema Topológico

Estos errores provienen del sistema `topology.rs` y son verificados en tiempo de lectura.

### 2.1 E001 — OrbitViolation

**Categoría:** Violación Orbital

**Definición:** Se intentó acceder directamente a un nivel interno saltando capas de abstracción.

**Regla violada:** Ningún nodo de Interface (50.1-100.0) puede conectarse directamente a Core (0-10).

**Condiciones que lo detonan:**
1. `light { from: EntidadEnInterface, to: EntidadEnCore }`
2. `light { to: Sun }` — El Sun no puede recibir luz
3. `light { from: EntidadEnDeepSpace }` — DeepSpace no puede emitir

**Ejemplo de código inválido:**
```cosmos
galaxy InterfazUsuario {
    planet RenderizarUI {
        position: { radius: 75.0, theta: 0.5 }  // Interface
    }
}

light {
    from: RenderizarUI      // Interface
    to: Sol                // Core (0,0)
    flow: Primary
}
```

**Mensaje de error:**
```
E001: OrbitViolation
Mensaje: "Violación orbital: no se puede acceder directamente a niveles internos saltando capas"
Detalle: "Entidad 'RenderizarUI' (banda Interface) intentó conectar con 'Sol' (banda Core)"
```

**Corrección sugerida:**
```cosmos
// La luz debe pasar por Logic obligatoriamente:
light { from: RenderizarUI, to: ControladorCentral, flow: Primary }
light { from: ControladorCentral, to: Sol, flow: Primary }
```

---

### 2.2 E002 — MassCollision

**Categoría:** Colisión de Masa

**Definición:** Dos entidades ocupan posición idéntica en el espacio topológico.

**Regla violada:** G-001 del RFC-0001 — Dos módulos no pueden compartir la misma posición semántica.

**Condiciones que lo detonan:**
```
distance(entidad_a.position, entidad_b.position) < EPSILON
```
Donde `EPSILON = 0.5`

**Ejemplo de código inválido:**
```cosmos
planet ServicioA {
    position: { radius: 25.0, theta: 1.57 }
    mass: 100
}

planet ServicioB {
    position: { radius: 25.0, theta: 1.57 }  // ¡Misma posición!
    mass: 80
}
```

**Mensaje de error:**
```
E002: MassCollision
Mensaje: "Colisión: dos módulos comparten la misma posición semántica"
Detalle: "Entidades 'ServicioA' y 'ServicioB' en posición (25.0, 1.57) — distancia 0.0 < 0.5"
```

**Corrección sugerida:**
```cosmos
planet ServicioA {
    position: { radius: 25.0, theta: 1.57 }
    mass: 100
}

planet ServicioB {
    position: { radius: 26.0, theta: 1.57 }  // Posición diferenciada
    mass: 80
}
```

---

### 2.3 E003 — LightPathTooLong

**Categoría:** Ruta de Luz Inválida

**Definición:** La distancia entre emisor y receptor supera el alcance máximo del emisor.

**Regla violada:** La luz solo puede viajar hasta `max_reach` del emisor.

**Cálculo:**
```
distancia = emitter.position.distance(receiver.position)
alcance = emitter.max_reach() = BASE_CONSTANT / emitter.mass

Si distancia > alcance → Error
```

**Condiciones que lo detonan:**
```
distance(emitter, receiver) > (1000.0 / emitter.mass)
```

**Ejemplo de código inválido:**
```cosmos
planet EmisorLigero {
    position: { radius: 5.0, theta: 0.0 }
    mass: 500  // Alto peso → bajo alcance
}

planet ReceptorLejano {
    position: { radius: 80.0, theta: 3.14 }
    mass: 10
}

light {
    from: EmisorLigero      // alcance = 1000/500 = 2.0
    to: ReceptorLejano      // distancia ≈ 75.0
    flow: Primary
}
```

**Mensaje de error:**
```
E003: LightPathTooLong
Mensaje: "Ruta demasiado larga: {distancia} > alcance {alcance}"
Detalle: "Emisor 'EmisorLigero' (alcance: 2.0) no puede alcanzar 'ReceptorLejano' (distancia: 75.0)"
```

**Corrección sugerida:**
- Reducir la masa del emisor (aumenta alcance)
- Usar un intermediario entre ambos
- Mover el receptor más cerca

---

### 2.4 E004 — SpaghettiTensionExceeded

**Categoría:** Tensión Excesiva

**Definición:** La red tiene demasiadas conexiones cruzadas, indicando acoplamiento excesivo.

**Regla violada:** C-002 del RFC-0001 — La cantidad de cruces espaciales entre conexiones determina la Tensión del Universo.

**Cálculo:**
```
cruces_totales = count_crossings(all_connections)

Si cruces_totales > umbral → Error
```

**Condiciones que lo detonan:**
- Dos segmentos de conexión se cruzan en el espacio cartesiano
- El número de cruces supera el umbral configurado (default: 10)

**Ejemplo de código inválido:**
```cosmos
// Muchas conexiones cruzadas generan tensión
light { from: A, to: D }
light { from: B, to: E }
light { from: C, to: F }
// Los segmentos A-D, B-E, C-F probablemente se cruzan
```

**Mensaje de error:**
```
E004: SpaghettiTensionExceeded
Mensaje: "Sobrecarga de conexiones: demasiadas dependencias cruzadas"
Detalle: "Se detectaron 15 cruces entre conexiones — umbral: 10"
```

**Corrección sugerida:**
- Reorganizar posiciones para minimizar cruces
- Crear galaxias separadas para agrupar conexiones locales
- Eliminar conexiones redundantes

---

### 2.5 E005 — InvalidCoordinates

**Categoría:** Coordenadas Inválidas

**Definición:** Las coordenadas polares no son válidas para el espacio topológico.

**Regla violada:** Toda posición debe tener radio no-negativo y ángulo finito.

**Condiciones que lo detonan:**
1. `radius < 0.0`
2. `theta` es `NaN` o `Infinity`

**Ejemplo de código inválido:**
```cosmos
planet Invalido1 {
    position: { radius: -5.0, theta: 0.0 }  // Radio negativo
}

planet Invalido2 {
    position: { radius: 25.0, theta: inf }  // Theta infinito
}
```

**Mensaje de error:**
```
E005: InvalidCoordinates
Mensaje: "Coordenadas no válidas: radio negativo o ángulo inválido"
Detalle: "Entidad 'Invalido1' tiene radius=-5.0"
```

**Corrección sugerida:**
- Usar radio >= 0.0
- Usar theta en rango válido (normalmente 0 a 2π)

---

### 2.6 E006 — BidirectionalLightForbidden

**Categoría:** Luz Bidireccional Prohibida

**Definición:** Se intentó declarar una conexión bidireccional como entidad única.

**Regla violada:** C-003 del RFC-0001 — La luz es estrictamente unidireccional.

**Condiciones que lo detonan:**
- El parser detecta sintaxis bidireccional (si se implementara)
- Se intenta usar `direction: both` o similar

**Ejemplo de código inválido (si existiera la sintaxis):**
```cosmos
// ❌ NO VÁLIDO — La gramática no lo permite
connection A <-> B {
    direction: both
}
```

**Mensaje de error:**
```
E006: BidirectionalLightForbidden
Mensaje: "Conexión bidireccional declarada como entidad única"
Detalle: "Las conexiones bidireccionales deben representarse como dos flujos separados"
```

**Corrección sugerida:**
```cosmos
// ✅ VÁLIDO
light { from: A, to: B, flow: Primary }
light { from: B, to: A, flow: Read }
```

---

### 2.7 E007 — DeepSpaceViolation

**Categoría:** Violación de Espacio Profundo

**Definición:** Una entidad existe en la banda orbital DeepSpace, fuera de los límites operativos del universo.

**Regla violada:** La banda DeepSpace (>100.0) está reservada y prohibida para operación.

**Condiciones que lo detonan:**
```
position.radius > 100.0
```

**Ejemplo de código inválido:**
```cosmos
planet PerdidoEnElEspacio {
    position: { radius: 150.0, theta: 0.0 }  // ¡DeepSpace!
    mass: 50
}
```

**Mensaje de error:**
```
E007: DeepSpaceViolation
Mensaje: "Entidad en banda prohibida: DeepSpace (>100.0)"
Detalle: "Entidad 'PerdidoEnElEspacio' tiene radius=150.0"
```

**Corrección sugerida:**
- Mover la entidad a una banda válida (0-100)
- Si es intencional (para archival), documentar como inactiva

---

## 3. Errores Estructurales

Estos errores verifican la estructura sintáctica y semántica del documento.

### 3.1 E101 — DuplicateIdentifier

**Categoría:** Identificador Duplicado

**Definición:** Dos entidades dentro del mismo alcance tienen el mismo nombre.

**Condiciones que lo detonan:**
- Dos `planet`, `galaxy`, `nebula`, `satellite`, o `star` con el mismo nombre
- El nombre es único dentro del universo

**Ejemplo de código inválido:**
```cosmos
galaxy Gestion {
    planet Procesar { ... }
    planet Procesar { ... }  // ¡Duplicado!
}
```

**Mensaje de error:**
```
E101: DuplicateIdentifier
Mensaje: "Identificador duplicado: 'Procesar' ya existe en este alcance"
Detalle: "Primera declaración en línea 5, segunda en línea 8"
```

---

### 3.2 E102 — MissingRequiredField

**Categoría:** Campo Obligatorio Ausente

**Definición:** Una entidad no declara un campo que es obligatorio según la gramática.

**Condiciones que lo detonan:**
- `planet` sin `position` o `mass`
- `star` sin `position`, `value`, o `type`
- `universe` sin `metadata`

**Ejemplo de código inválido:**
```cosmos
planet Incompleto {
    // Falta: position, mass
}
```

**Mensaje de error:**
```
E102: MissingRequiredField
Mensaje: "Campo obligatorio ausente: 'position'"
Detalle: "Entidad 'Incompleto' requiere: position, mass"
```

---

### 3.3 E103 — InvalidEntityReference

**Categoría:** Referencia Inválida

**Definición:** Un `light` referencia una entidad que no existe.

**Condiciones que lo detonan:**
- `from: EntidadInexistente`
- `to: EntidadInexistente`

**Ejemplo de código inválido:**
```cosmos
light {
    from: ServicioReal
    to: ServicioInexistente  // ¡No existe!
    flow: Primary
}
```

**Mensaje de error:**
```
E103: InvalidEntityReference
Mensaje: "Referencia inválida: 'ServicioInexistente' no existe"
Detalle: "Declarado en light { from: ServicioReal, to: ??? }"
```

---

### 3.4 E104 — InvalidMetadata

**Categoría:** Metadatos Inválidos

**Definición:** La sección de metadatos no cumple con los requisitos.

**Condiciones que lo detonan:**
- `version` ausente o vacía
- `author` ausente o vacía
- `standard` no es "Cosmos"

**Ejemplo de código inválido:**
```cosmos
metadata {
    version: ""           // ¡Vacío!
    author: "Juan"
    standard: "Cosmos"
}
```

**Mensaje de error:**
```
E104: InvalidMetadata
Mensaje: "Metadatos inválidos: 'version' no puede estar vacío"
Detalle: "Campo 'version' en universe metadata"
```

---

### 3.5 E105 — StarAsReceiver

**Categoría:** Estrella como Receptor

**Definición:** Una Star fue utilizada como receptor (`to`) en una conexión.

**Regla violada:** ST-002 del RFC-0001 — Las Stars no pueden recibir luz.

**Condiciones que lo detonan:**
```cosmos
light {
    from: Entidad
    to: UNA_STAR  // ❌ Inválido
    flow: Primary
}
```

**Ejemplo de código inválido:**
```cosmos
star MAX_RETRY {
    position: { radius: 5.0, theta: 0.0 }
    value: 3
    type: Integer
}

light {
    from: Procesar
    to: MAX_RETRY  // ❌ Las estrellas no reciben
    flow: Primary
}
```

**Mensaje de error:**
```
E105: StarAsReceiver
Mensaje: "Stars no pueden recibir luz de otras entidades"
Detalle: "Entidad 'MAX_RETRY' es una Star y no puede ser receptor"
```

---

### 3.6 E106 — SatelliteScopeViolation

**Categoría:** Violación de Alcance de Satélite

**Definición:** Un Satellite intentó comunicarse con una entidad fuera de su órbita.

**Regla violada:** SA-002 del RFC-0001 — Satellites solo pueden comunicarse con su Planet o Satellites del mismo Planet.

**Condiciones que lo detonan:**
```cosmos
// Satellite → EntidadEnOtraGalaxy
light {
    from: UnSatellite
    to: EntidadEnGalaxyDiferente
    flow: Primary
}
```

**Mensaje de error:**
```
E106: SatelliteScopeViolation
Mensaje: "Satellite 'ValidarInput' intentó conectar con entidad fuera de su órbita"
Detalle: "Para comunicación cross-galaxy, use el Planet anfitrión como intermediario"
```

---

### 3.7 E107 — GalaxyInCore

**Categoría:** Galaxia en Core

**Definición:** Una Galaxy fue posicionada en la banda orbital Core.

**Regla violada:** G-002 del RFC-0001 — Las Galaxies no pueden habitar Core.

**Condiciones que lo detonan:**
```cosmos
galaxy EnElNucleo {
    position: { radius: 5.0, theta: 0.0 }  // ❌ Core
}
```

**Mensaje de error:**
```
E107: GalaxyInCore
Mensaje: "Galaxy no puede existir en banda Core (0.0-10.0)"
Detalle: "Galaxy 'EnElNucleo' tiene radius=5.0"
```

---

### 3.8 E108 — SunWithPosition

**Categoría:** Sol con Posición Declarada

**Definición:** Se intentó declarar explícitamente la posición del Sun.

**Regla violada:** G-001 de RFC-0004-A — El Sun tiene posición fija (0,0) implícita.

**Condiciones que lo detonan:**
```cosmos
// ❌ NO VÁLIDO
sun Sol {
    position: { radius: 0.0, theta: 0.0 }  // ¡Prohibido!
}
```

**Mensaje de error:**
```
E108: SunWithPosition
Mensaje: "Sun no puede tener posición declarada"
Detalle: "La posición del Sol es (0,0) implícita — declararla es OrbitViolation"
```

---

## 4. Tabla Resumen de Errores

| Código | Nombre | Categoría | Severidad |
|--------|--------|-----------|-----------|
| E001 | OrbitViolation | Topológica | Error |
| E002 | MassCollision | Topológica | Error |
| E003 | LightPathTooLong | Topológica | Warning |
| E004 | SpaghettiTensionExceeded | Topológica | Warning |
| E005 | InvalidCoordinates | Topológica | Error |
| E006 | BidirectionalLightForbidden | Topológica | Error |
| E007 | DeepSpaceViolation | Topológica | Error |
| E101 | DuplicateIdentifier | Estructural | Error |
| E102 | MissingRequiredField | Estructural | Error |
| E103 | InvalidEntityReference | Estructural | Error |
| E104 | InvalidMetadata | Estructural | Error |
| E105 | StarAsReceiver | Semántica | Error |
| E106 | SatelliteScopeViolation | Semántica | Error |
| E107 | GalaxyInCore | Semántica | Error |
| E108 | SunWithPosition | Semántica | Error |

---

## 5. Comportamiento del Compilador

### 5.1 Modo Strict (Default)

El compilador se detiene y reporta todos los errores encontrados:
- Errores topológicos y estructurales: **Bloquean compilación**
- Warnings (E003, E004): **Advertir pero permitir**

### 5.2 Modo Lax

El compilador continúa y genera advertencias:
- Todos los errores se reportan como warnings
- Útil para prototipado rápido

### 5.3 Salida de Error

Formato estándar:
```
[archivo.cosmos:Línea:Columna] CÓDIGO: Mensaje
  → Detalle adicional
  → Sugerencia de corrección (si disponible)
```

Ejemplo:
```
[miapp.cosmos:45:8] E001: OrbitViolation
  → Violación orbital: no se puede acceder directamente a niveles internos saltando capas
  → Entidad 'RenderizarUI' (banda Interface) intentó conectar con 'Sol' (banda Core)
  → Considere usar un intermediario en banda Logic
```

---

## 6. Historial de Cambios

| Versión | Fecha | Autor | Cambios |
|---------|-------|-------|---------|
| 0.1 | 2026-07-12 | Solvens | Borrador inicial |

---

## 7. Estados de Auditoría

| Observación | Estado | Notas |
|-------------|--------|-------|
| G-001 | ✅ RESUELTA | E108: SunWithPosition |
| G-002 | ✅ RESUELTA | E107: GalaxyInCore, E101: DuplicateIdentifier |
| G-003 | ✅ APLICADA | E001-E007: Errores topológicos |
| G-004 | ✅ RESUELTA | E102: MissingRequiredField |
| G-005 | ✅ RESUELTA | E006: BidirectionalLightForbidden |
| G-006 | ✅ RESUELTA | E103: InvalidEntityReference |
| G-007 | ✅ RESUELTA | E104: InvalidMetadata |

---

**Documento preparado para auditoría de Hans.**
