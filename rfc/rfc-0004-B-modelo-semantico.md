# RFC-0004-B: Modelo Semántico del Lenguaje Cosmos

**Estado:** 📋 BORRADOR — Pendiente auditoría de Hans

**Versión:** 0.1

**Fecha:** 2026-07-12

**Autor:** Solvens (Arquitecto del Estándar)

---

## 1. Prefacio y Contexto

Este documento define el **significado** de cada constructo sintáctico definido en RFC-0004-A. Establece las reglas semánticas que garantizan la consistencia del programa Cosmos.

### 1.1 Relación con Otros RFC

| RFC | Título | Relación |
|-----|--------|----------|
| RFC-0004-A | Gramática Formal | Sintaxis canónica |
| **RFC-0004-B** | **Este documento** | Semántica de cada constructo |
| RFC-0004-C | Reglas de Validación | Errores en tiempo de lectura |

### 1.2 Principios Semánticos

#### Principio de Dualidad Forma-Semántica

Cada elemento visual en un archivo `.cosmos` tiene **exactamente un** significado ejecutable. No existen decoraciones puramente visuales.

#### Principio de Herencia Posicional

La posición de una entidad contiene **toda** la información de alcance y jerarquía. No existe herencia de comportamiento entre entidades.

---

## 2. Significado de Cada Constructo

### 2.1 Universe

```cosmos
universe "Nombre" { ... }
```

**Significado:** Define el dominio de ejecución completo. Es el **contenedor absoluto** de todo lo que existe en el programa Cosmos.

**Propiedades:**
- Nombre: Identificador único del universo (STRING)
- Sun: Origen absoluto de toda ejecución
- Galaxies: Dominios de negocio o librerías principales
- Connections: Vectores de flujo de datos

**Alcance:** Global. Todo lo definido dentro pertenece a este universo.

**Ejemplo:** `universe "SistemaBancario" { ... }` define un universo llamado "SistemaBancario".

---

### 2.2 Sun (Sol)

```cosmos
sun Sol
```

**Significado:** El Sol es el **punto de origen absoluto** del universo. Toda ejecución comienza aquí. Es el *single entry point* determinista.

**Propiedades:**
- Posición: `(radius: 0.0, theta: 0.0)` — **fija, no declarable**
- Masa: 0 (no ejecutable)
- Tipo: `Origin`
- Alcance: Infinito en teoría, pero no emite luz (es el origen)

**Jerarquía:** El Sun **no pertenece** a ninguna Galaxy. Es previo a toda estructura.

**Semántica especial:**
- No puede recibir luz de ninguna entidad (OrbitViolation)
- Solo puede emitir hacia entidades en banda Logic o superior
- Es el único nodo con `radius = 0`

**Analogía física:** Es el centro del sistema de coordenadas polares. Sin él, no existe referencia.

---

### 2.3 Galaxy (Galaxia)

```cosmos
galaxy MiGalaxia {
    nebula N1 { ... }
    planet P1 { ... }
}
```

**Significado:** Una Galaxy representa un **Dominio de Negocio** (*Bounded Context*) o una librería principal independiente. Es la agrupación macroscópica más grande después del propio universo.

**Propiedades:**
- Nombre: Identificador único dentro del universo
- Posición: Define su ubicación en el espacio topológico
- Masa: Suma de las masas de sus componentes internos
- Alcance: Calculado inversamente a su masa

**Alcance semántico:** Todo lo dentro de una Galaxy tiene *acoplamiento débil* con otras Galaxies. Las conexiones entre galaxias deben pasar validaciones estrictas.

**Regla G-002 del RFC-0001:**
> Una Galaxy no puede existir en la banda orbital Core (0.0 - 10.0).

**Validación de posición:**
- `radius` mínimo: `> 10.0` (evita Core)
- `radius` recomendado: `10.1 - 50.0` (banda Logic) o `50.1 - 100.0` (banda Interface)

**Ejemplo:** `galaxy GestionRiesgos { ... }` agrupa toda la funcionalidad de gestión de riesgos.

---

### 2.4 Nebula (Nebulosa)

```cosmos
nebula Contabilidad {
    planet P1 { ... }
    star MAX_VALOR { ... }
}
```

**Significado:** Una Nebula es un **espacio de nombres** (*Namespace*) o sub-módulo organizativo dentro de una Galaxy. Actúa como una nube que agrupa entidades afines para evitar la dispersión caótica.

**Propiedades:**
- Nombre: Identificador único dentro de la Galaxy padre
- **NO tiene posición propia** — hereda contexto visual de la Galaxy
- **NO tiene masa** — es puramente organizativa
- **NO puede tener conexiones propias** — las conexiones son entre entidades, no nebulas

**Alcance semántico:** Limitado a la Galaxy padre. Una Nebula no puede existir fuera de una Galaxy.

**Regla N-001 del RFC-0001:**
> Una Nebula debe pertenecer estrictamente a una sola Galaxy.

**Analogía:** Es como una constelación dibujada en el cielo para ayuda visual. Las estrellas siguen existiendo físicamente aunque no las agrupemos en constelaciones.

**Ejemplo:** `nebula Validadores { planet ValidarEmail, planet ValidarTelefono }`

---

### 2.5 Planet (Planeta)

```cosmos
planet ProcesarOrden {
    position: { radius: 25.0, theta: 1.57 }
    mass: 150
    type: Service
    
    satellite Sat1 { ... }
}
```

**Significado:** El Planet es la **entidad ejecutable principal**. Equivale topológicamente a una Clase pesada, un Servicio o un Módulo Estructural de alta complejidad.

**Propiedades:**

| Propiedad | Tipo | Descripción |
|-----------|------|-------------|
| `position` | PolarPosition | Ubicación en espacio topológico |
| `mass` | u32 | Complejidad y peso ejecutable |
| `type` | IDENTIFIER | Tipo de servicio (Service, Repository, etc.) |
| `satellites` | Vec<Satellite> | Entidades subordinadas |

**Cálculo de órbita (automático, no declarable):**

```rust
fn orbit(&self) -> OrbitBand {
    match self.position.radius {
        r if r <= 10.0 => OrbitBand::Core,
        r if r <= 50.0 => OrbitBand::Logic,
        r if r <= 100.0 => OrbitBand::Interface,
        _ => OrbitBand::DeepSpace,
    }
}
```

**Cálculo de alcance máximo:**

```rust
fn max_reach(&self) -> f64 {
    const BASE_CONSTANT: f64 = 1000.0;
    BASE_CONSTANT / self.mass as f64
}
```

**Semántica de masa:**
- `mass` alto = menor alcance = entidad "pesada" que necesita intermediarios
- `mass` bajo = mayor alcance = entidad "ligera" que puede comunicarse a distancia

**Restricciones topológicas:**
- Dos Planets no pueden tener la misma posición (`MassCollision`)
- Planet en Interface (50.1-100.0) NO puede emitir hacia Core (0-10) (`OrbitViolation`)

---

### 2.6 Satellite (Satélite)

```cosmos
satellite ValidarInput {
    relative_position: { radius: 2.0, theta: 0.5 }
    mass: 10
    type: Validator
}
```

**Significado:** Un Satellite es una **entidad ejecutora subordinada** (función pura auxiliar, transformador de datos, método específico) que existe anclada a la gravedad de un Planet.

**Propiedades:**

| Propiedad | Tipo | Descripción |
|-----------|------|-------------|
| `relative_position` | PolarPosition | Posición relativa al Planet padre |
| `mass` | u32 | Complejidad de la función |
| `type` | IDENTIFIER | Tipo (Validator, Transformer, etc.) |

**Cálculo de posición absoluta:**

```rust
fn absolute_position(&self, planet: &PolarPosition) -> PolarPosition {
    PolarPosition {
        radius: planet.radius + self.relative_position.radius,
        theta: planet.theta + self.relative_position.theta,
    }
}
```

**Reglas de existenciales (SA-001, SA-002):**
- Un Satellite **no puede existir en el vacío** — debe estar dentro de un Planet
- Un Satellite solo puede emitir luz hacia:
  1. Su propio Planet anfitrión
  2. Satellites de la misma órbita (mismo Planet)

**Para comunicarse con otra Galaxy:** La luz debe canalizarse a través del Planet anfitrión.

---

### 2.7 Star (Estrella)

```cosmos
star MAX_RETRY_COUNT {
    position: { radius: 8.0, theta: 0.785 }
    value: 3
    type: Integer
}
```

**Significado:** Una Star representa una **constante global**, variable de entorno inmutable o configuración estática del universo.

**Propiedades:**

| Propiedad | Tipo | Descripción |
|-----------|------|-------------|
| `position` | PolarPosition | Ubicación en espacio topológico |
| `value` | NUMBER | Valor constante |
| `type` | IDENTIFIER | Tipo de dato (Integer, Float, String, Boolean) |

**Semántica especial (ST-001, ST-002):**

1. **Emisión universal:** Cualquier entidad puede **leer** una Star sin importar la distancia
2. **Recepción prohibida:** Una Star **no puede recibir** luz de ninguna otra entidad
3. **Evaluación en compilación:** El valor se resuelve en tiempo de compilación, no ejecución

**Analogía:** Es como el sol del sistema solar — todos los planetas pueden ver su luz sin necesidad de cables de conexión.

**Posición permitida:** Recomendablemente en banda Core (0-10) o Logic (10.1-50) para mantener cercanía con el centro del universo.

---

## 3. Sistema de Luz (Conexiones)

### 3.1 Significado de Light

```cosmos
light {
    from: EntidadOrigen
    to: EntidadDestino
    flow: Primary
}
```

**Significado:** Representa un **vector dirigido de flujo de datos y control** entre dos entidades topológicas.

**Propiedades semánticas:**

| Propiedad | Significado |
|-----------|--------------|
| `from` | Origen del flujo — emisor de luz |
| `to` | Destino del flujo — receptor de luz |
| `flow` | Tipo semántico del flujo |

### 3.2 Tipos de Flujo

| Tipo | Significado | ¿Puede recibir respuesta? |
|------|-------------|---------------------------|
| `Primary` | Flujo principal de ejecución | Sí, con `light` separado tipo `Read` |
| `Asynchronous` | Ejecución paralela, no bloqueante | No (fire-and-forget) |
| `Read` | Solo lectura de datos | N/A (es单向) |

### 3.3 Ley C-003 — Unidireccionalidad

> **La luz es vectorial y unidireccional.**

Una conexión bidireccional `A ⇄ B` **NO existe como entidad**. Debe representarse como:

```cosmos
light { from: A, to: B, flow: Primary }  // A pide datos a B
light { from: B, to: A, flow: Read }       // B envía datos a A
```

**Justificación:**

1. **Asimetría arquitectónica:** La capacidad de emitir luz depende del nodo (`LightReach = f(masa, tipo, estado)`). Por lo tanto `A → B ≠ B → A`.

2. **Transparencia visual:** Una flecha doble oculta dos eventos distintos bajo un solo símbolo, violando Forma ↔ Semántica.

3. **Verificación estática:** Cada flujo puede ser analizado individualmente (permisos, alcance, tipos, ciclos, tensión).

---

## 4. Leyes Derivables

### 4.1 Ley L001 — No Redundancia Semántica

**Derivada de G-003:**

> Toda propiedad derivable de la geometría NO debe almacenarse como dato independiente.

**Aplicación:**

| Declarar | Inferir | ¿Almacenar? |
|----------|---------|-------------|
| `position.radius` | `orbit` | ❌ NO — calcular con función |
| `relative_position` + `planet.position` | `absolute_position` | ❌ NO — calcular |
| `mass` | `max_reach` | ❌ NO — calcular |
| `galaxy` contiene `nebula` | `scope` | ❌ NO — implícito |

### 4.2 Ley L002 — Asimetría de Flujo

**Derivada de C-003:**

> El flujo `A → B` y el flujo `B → A` son eventos distintos con propiedades independientes.

**Aplicación:**
- Pueden tener diferentes tipos de flujo (`Primary` vs `Read`)
- Pueden tener diferentes latencias
- Pueden tener diferentes costos de energía
- Uno puede ser válido y el otro inválido (según orbitas)

### 4.3 Ley L003 — Localidad de Satélites

**Derivada de SA-001, SA-002:**

> Un Satellite existe exclusivamente en el contexto de su Planet padre. No tiene comunicación directa con entidades externas a esa órbita.

**Aplicación:**
- Validar que `from: Satellite` en un `light` tenga como `to` el mismo Planet o un Satellite del mismo Planet
- Para comunicación cross-galaxy, el flujo debe pasar por el Planet

### 4.4 Ley L004 — Inmutabilidad de Estrellas

**Derivada de ST-002:**

> Una Star no puede participar como receptor (`to`) en ninguna conexión.

**Aplicación:**
- El validador debe rechazar `light { to: MAX_VALOR, ... }`
- Stars solo pueden ser emisores implícitos (leídos por otros)

---

## 5. Tabla de Alcances

| Entidad | Puede emitir luz | Puede recibir luz | Tiene alcance finito |
|---------|------------------|-------------------|---------------------|
| `sun` | ✅ (como origen) | ❌ | ✅ (infinito teóricamente) |
| `galaxy` | ✅ | ✅ | ✅ |
| `planet` | ✅ | ✅ | ✅ |
| `satellite` | ✅ (limitado) | ✅ | ✅ (limitado) |
| `star` | ❌ (es leído) | ❌ | Infinito (lectura sin costo) |
| `nebula` | ❌ (no existe físicamente) | ❌ | N/A |

---

## 6. Propiedades Inferidas vs Declaradas

### 6.1 Tabla Completa

| Entidad | Campo | Tipo | Fuente |
|---------|-------|------|--------|
| `sun` | position | PolarPosition | **Implícito** (0,0) |
| `sun` | orbit | OrbitBand | **Inferido** (Core) |
| `galaxy` | position | PolarPosition | **Declarado** |
| `galaxy` | orbit | OrbitBand | **Inferido** |
| `nebula` | position | — | **No existe** |
| `planet` | position | PolarPosition | **Declarado** |
| `planet` | orbit | OrbitBand | **Inferido** |
| `planet` | max_reach | f64 | **Inferido** |
| `satellite` | relative_position | PolarPosition | **Declarado** |
| `satellite` | absolute_position | PolarPosition | **Inferido** |
| `satellite` | orbit | OrbitBand | **Inferido** |
| `star` | position | PolarPosition | **Declarado** |
| `star` | orbit | OrbitBand | **Inferido** |

---

## 7. Historial de Cambios

| Versión | Fecha | Autor | Cambios |
|---------|-------|-------|---------|
| 0.1 | 2026-07-12 | Solvens | Borrador inicial |

---

## 8. Estados de Auditoría

| Observación | Estado | Notas |
|-------------|--------|-------|
| G-001 | ✅ RESUELTA | Sun tiene semántica de origen absoluto |
| G-002 | ✅ RESUELTA | Nombre es label humano, NodeId es interno |
| G-003 | ✅ APLICADA | Semántica derivada de geometría |
| G-004 | ✅ RESUELTA | Tipos son de dato, no de órbita |
| G-005 | ✅ RESUELTA | Light es vectorial y extensible |
| G-006 | ✅ RESUELTA | Nebula es puramente organizativa |
| G-007 | ✅ RESUELTA | Metadatos definen contexto del universo |

---

**Documento preparado para auditoría de Hans.**
