# RFC-0004-C: Reglas de Validación del Lenguaje Cosmos

> © 2026 Luis Alberto Burciaga Miker. Todos los derechos reservados.
> CC BY-NC-ND 4.0

**Estado:** CONGELADO | **Versión:** 1.0 (reescritura 2026-11-25) | **Fecha:** 2026-11-25

> Reescritura: el catálogo es ahora el catálogo unificado de RFC-0004-F
> §8 (P/E/S/C/W/CT con un solo significado por código). El texto
> anterior tenía E001/E002 reutilizados con otros significados en el
> compilador, E401 con dos significados, y E108 con dos significados
> (MultipleSun en F, SunWithPosition en C). Todo eso queda resuelto
> aquí: un código, un significado.

---

## 1. Catálogo de Errores

### Errores de Parser (P001–P015)

| Código | Nombre | Descripción |
|--------|--------|-------------|
| P001 | UnexpectedToken | Token inesperado |
| P002 | UnexpectedEOF | Fin de archivo inesperado |
| P003 | MissingUniverse | Falta `universe` |
| P004 | MissingMetadata | Falta `metadata` |
| P005 | MissingSun | Falta `sun` |
| P006 | MissingPosition | Falta `position` |
| P007 | MissingMass | Falta `mass` |
| P008 | MissingType | Falta `type` |
| P009 | InvalidIdentifier | Identificador mal formado o referencia sin `@` |
| P010 | InvalidNumber | Número mal formado |
| P011 | InvalidString | Cadena mal formada |
| P012 | SunPositionOverride | Posición declarada para el Sol |
| P013 | DuplicateEntity | Identificador duplicado (alias de E101) |
| P014 | UnknownBlock | Bloque desconocido |
| P015 | InvalidFlowType | Tipo de flujo inválido |

### Errores Topológicos (E001–E007)

| Código | Nombre | Descripción |
|--------|--------|-------------|
| E001 | OrbitViolation | Salto ilegal entre bandas (`usa` hacia órbita menor; flujo Interface → Core) |
| E002 | MassCollision | Dos nodos a distancia < 0.5 en la misma galaxia; dos galaxias a distancia < 1.0 |
| E003 | LightPathTooLong | Distancia supera `max_reach = 1000.0 / masa` |
| E004 | SpaghettiTensionExceeded | Cruces entre flujos > 10 (warning) |
| E005 | InvalidCoordinates | Radio negativo o ángulo no finito |
| E006 | BidirectionalLightForbidden | Conexión bidireccional declarada |
| E007 | DeepSpaceViolation | Entidad con radio > 100.0 |

### Errores Estructurales (E101–E112)

| Código | Nombre | Descripción |
|--------|--------|-------------|
| E101 | DuplicateIdentifier | Identificador duplicado |
| E102 | MissingRequiredField | Campo obligatorio ausente |
| E103 | InvalidEntityReference | Referencia a entidad inexistente |
| E104 | InvalidMetadata | Metadatos inválidos |
| E105 | SunOrStarAsReceiver | Sol o Star como receptor de flujo |
| E106 | SatelliteScopeViolation | Satélite fuera de su galaxia, fuera de la banda de su padre, o con padre incorrecto |
| E107 | GalaxyInCore | Galaxia con radio ≤ 10.0 |
| E108 | MultipleSun | Múltiples Sun declarados |
| E109 | FlowMatrixViolation | Flujo prohibido por la matriz de tipos |
| E110 | OrbitTypeMismatch | `PlanetType` incompatible con su banda |
| E111 | SatelliteMassRatioExceeded | Masa satelital total > 50% de la masa del planeta |
| E112 | EmptyUniverse | Universo sin galaxias |

### Errores Temporales y de Ciclo

| Código | Nombre | Descripción |
|--------|--------|-------------|
| E401 | FlowAuthorizationExpired | Fecha límite superada |
| E403 | CrossGalaxyAccess | Acceso entre galaxias prohibido |
| E404 | FlowCycle | Ciclo en grafo de flujo o de dependencias |
| E405 | MalformedDate | Fecha mal formada |
| W001 | TemporalAccessExpired | Flujo vencido (warning en modo lax) |

### Errores de Contrato (RFC-0008)

CT001 ContractNotFound, CT002 ContractMismatch, CT003 PurityViolation,
CT004 SignatureMismatch, CT005 ImportWithoutContract.

### Errores CIR (C001–C010)

Reservados para la representación binaria (RFC-0007 pendiente).
Ver RFC-0004-F §8.6.

## 2. Comportamiento del Compilador

### Modo Strict (Default)
Errores topológicos, estructurales y temporales bloquean la compilación.
Errores de parser bloquean siempre, en cualquier modo.

### Modo Lax
Todos los errores E y S se reportan como warnings y la compilación
continúa. Los errores de parser siguen bloqueando.

### Fases de validación (en orden)

1. **Parseo** — gramática Lark (P001–P015).
2. **Esquema** — existen universe, metadata y sun; `standard` es
   `"Cosmos"` (P003, P004, P005, E104).
3. **Topología** — E001–E007, E101–E112, E403, E404, E004 (warning).
4. **Temporal** — E401, E405 (E401 en strict, W001 en lax).

El compilador se detiene en la primera fase con errores en modo
strict. En modo lax, acumula y reporta todas las fases.

## 3. Reglas de Compilación

| Regla | Descripción |
|-------|-------------|
| RV-001 | Todo Universe debe tener exactamente un Sun |
| RV-002 | Todo Planet debe tener position, mass, type y @id |
| RV-003 | Toda referencia debe usar `@identificador` (salvo el Sol) |
| RV-004 | Las posiciones deben ser únicas dentro de una Galaxy (distancia ≥ 0.5) |
| RV-005 | Los flujos y las dependencias no pueden crear ciclos |
| RV-006 | Los flujos deben respetar alcance, matriz de tipos y dominio |
| RV-007 | Las dependencias deben respetar la regla hacia adentro |
| RV-008 | El tipo de cada planeta debe admitir su banda |
| RV-009 | La masa satelital total no supera el 50% de la masa del planeta |
| RV-010 | Ningún planeta en banda Core; ninguna galaxia en Core ni en DeepSpace |
| RV-011 | El universo tiene al menos una galaxia |
| RV-012 | Cruces entre flujos: más de 10 es warning |

## 4. Reglas de Ejecución

| Regla | Descripción |
|-------|-------------|
| RE-001 | Verificar `restringido_hasta` antes de cada flujo, contra la fecha real |
| RE-002 | Verificar alcance (`max_reach`) del emisor |
| RE-003 | Verificar que el receptor existe |

## 5. Estados de Auditoría

| Observación | Estado |
|-------------|--------|
| G-001 a G-007 | ✅ RESUELTAS |

---

> © 2026 Luis Alberto Burciaga Miker. Todos los derechos reservados.
> Este sistema y su lógica fundamental están protegidos por secreto comercial.
