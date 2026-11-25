# RFC-0004-B: Modelo Semántico del Lenguaje Cosmos

> © 2026 Luis Alberto Burciaga Miker. Todos los derechos reservados.
> CC BY-NC-ND 4.0

**Estado:** CONGELADO | **Versión:** 1.0 (reescritura 2026-11-25) | **Fecha:** 2026-11-25

> Reescritura: la banda Core es solo del Sol (no existe el tipo de
> planeta `Core`), la posición del satélite es un desplazamiento polar
> respecto a su padre, y se agrega la ley L004 (dependencia y flujo son
> movimientos distintos). El resto del modelo no cambia.

---

## 1. Significado de Cada Constructo

### Universe
Define el dominio de ejecución completo. Es el contenedor absoluto de todo lo que existe.

### Sun
El Sol es el punto de origen absoluto. Toda ejecución comienza aquí.
Posición implícita (0, 0). Masa fija 10. Alcance 100: puede emitir hacia
cualquier nodo del universo. No recibe flujos.

### Star
Constante global inmutable. Se declara a nivel de universo, no tiene
posición ni masa, y no recibe flujos. Existe para nombrar valores
fijos del sistema.

### Galaxy
Representa un Dominio de Negocio o librería principal independiente.
Su posición es el ancla del dominio. No puede estar en Core ni en
DeepSpace.

### Nebula
Espacio de nombres organizativo sin posición propia. Agrupa planetas
dentro de una galaxia; no emite ni recibe flujos.

### Planet
Entidad ejecutable principal con estado dinámico. Tiene posición
absoluta, masa y tipo obligatorios. Su banda orbital se deriva de su
radio; su tipo debe admitir esa banda (E110).

### Satellite
Entidad subordinada anclada a un Planet. Se declara dentro de su
planeta padre. Su posición es un desplazamiento polar respecto al
padre. Su masa cuenta para el límite del 50% (E111). Hereda el
contexto del padre: mismo dominio, misma banda.

---

## 2. Propiedades Inferidas vs Declaradas

| Entidad | Campo | Fuente |
|---------|-------|--------|
| Sun | position (0,0) | Implícito |
| Sun | mass 10, alcance 100 | Fijo por especificación |
| Planet | orbit | Inferido de radius |
| Planet | max_reach | Calculado: 1000.0 / mass |
| Satellite | absolute_position | Padre + desplazamiento declarado |
| Satellite | orbit | Inferido de su posición absoluta |
| Galaxy | orbit | Inferido de radius |
| Star | (ninguna) | Sin posición ni masa |

## 3. Leyes Derivables

### L001 — No Redundancia Semántica
Toda propiedad derivable de la geometría NO debe almacenarse.

### L002 — Asimetría de Flujo
El flujo A → B y B → A son eventos distintos. Declarar ambos es
error E006, no un flujo bidireccional.

### L003 — Localidad de Satélites
Satellite existe exclusivamente en contexto de su Planet padre.

### L004 — Dependencia y flujo son movimientos distintos
Una dependencia (`usa`) apunta hacia el núcleo: lo que un módulo
necesita. Un flujo (`light`) sale del núcleo: lo que un módulo
entrega. Se rigen por reglas distintas (§5 de RFC-0004-F) y no se
pueden evaluar con una sola regla de dirección.

---

## 4. Estados de Auditoría

| Observación | Estado |
|-------------|--------|
| G-001 a G-007 | ✅ RESUELTAS |

---

> © 2026 Luis Alberto Burciaga Miker. Todos los derechos reservados.
> Este sistema y su lógica fundamental están protegidos por secreto comercial.
