//! Sistema de Topología Espacial de Cosmos de la Lógica
//! ✅ Cumplimiento: Forma ↔ Semántica · Geometría = Regla de Arquitectura · Verificación Estática
//!
//! === HISTORIAL DE VERSIONES ===
//! v1.0 - Versión inicial con errores base
//! v2.0 - Consolidado con RFC-0004-C, RFC-0005, RFC-0006, RFC-0007
//!
//! === CONSTANTES GLOBALES ===
const EPSILON: f64 = 0.5;                    // Colisión mínima entre nodos
const GALACTIC_EPSILON: f64 = 1.0;           // Separación mínima entre galaxias
const SATELLITE_MASS_RATIO: f64 = 0.5;      // Proporción máxima masa satelital
const BASE_CONSTANT: f64 = 1000.0;           // Constante base para cálculo de reach
const TENSION_THRESHOLD: usize = 10;          // Umbral máximo de cruces


// ============================================================
// 1. ERRORES TOPOLÓGICOS PROHIBIDOS (E001-E007)
// Cada error corresponde a una violación formal de la especificación
// Consolidado desde RFC-0004-C
// ============================================================
#[derive(Debug, Clone, PartialEq)]
pub enum TopologyError {
    // === ERRORES BASE (E001-E007) ===
    
    /// E001: Salto ilegal entre bandas orbitales (ej: Interface → Core directo)
    OrbitViolation,

    /// E002: Dos nodos ocupan posición idéntica: colisión de responsabilidades
    MassCollision,

    /// E003: El flujo de luz/datos supera el alcance máximo del emisor
    LightPathTooLong {
        distance: f64,
        reach: f64,
    },

    /// E004: Demasiadas conexiones cruzadas: acoplamiento excesivo
    SpaghettiTensionExceeded {
        crossings: usize,
        threshold: usize,
    },

    /// E005: Coordenadas no válidas: radio negativo, ángulo infinito/NaN
    InvalidCoordinates,

    /// E006: Conexión bidireccional declarada como entidad única (C-003)
    BidirectionalLightForbidden,

    /// E007: Entidad en banda DeepSpace (>100.0)
    DeepSpaceViolation,
}


impl std::fmt::Display for TopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopologyError::OrbitViolation => 
                write!(f, "Violación orbital: no se puede acceder directamente a niveles internos saltando capas"),
            TopologyError::MassCollision => 
                write!(f, "Colisión: dos módulos comparten la misma posición semántica"),
            TopologyError::LightPathTooLong { distance, reach } => 
                write!(f, "Ruta demasiado larga: {:.2} > alcance {:.2}", distance, reach),
            TopologyError::SpaghettiTensionExceeded { crossings, threshold } => 
                write!(f, "Sobrecarga de conexiones: {} cruces > umbral {}", crossings, threshold),
            TopologyError::InvalidCoordinates => 
                write!(f, "Coordenadas no válidas: radio negativo o ángulo inválido"),
            TopologyError::BidirectionalLightForbidden => 
                write!(f, "Conexión bidireccional prohibida: usar dos flujos separados"),
            TopologyError::DeepSpaceViolation => 
                write!(f, "Entidad en banda prohibida: DeepSpace (>100.0)"),
        }
    }
}


impl std::error::Error for TopologyError {}


// ============================================================
// 2. ERRORES DE PARSER (P001-P015)
// Consolidado desde RFC-0005
// ============================================================
#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    /// P001: Token inesperado en la posición actual
    UnexpectedToken,
    
    /// P002: Fin de archivo inesperado
    UnexpectedEOF,
    
    /// P003: Falta declaración de Universe
    MissingUniverse,
    
    /// P004: Falta bloque de Metadata
    MissingMetadata,
    
    /// P005: Falta declaración del Sun
    MissingSun,
    
    /// P006: Campo position faltante
    MissingPosition,
    
    /// P007: Campo mass faltante
    MissingMass,
    
    /// P008: Campo type faltante
    MissingType,
    
    /// P009: Identificador mal formado
    InvalidIdentifier,
    
    /// P010: Número mal formado
    InvalidNumber,
    
    /// P011: Cadena mal formada
    InvalidString,
    
    /// P012: Intento de declarar posición del Sun (G-001)
    SunPositionOverride,
    
    /// P013: Identificador duplicado (E101)
    DuplicateEntity,
    
    /// P014: Bloque desconocido
    UnknownBlock,
    
    /// P015: Tipo de flujo inválido (Sync/Async/Event vs Primary/Asynchronous/Read)
    InvalidFlowType,
}


impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken => write!(f, "Token inesperado"),
            ParserError::UnexpectedEOF => write!(f, "Fin de archivo inesperado"),
            ParserError::MissingUniverse => write!(f, "Falta declaración 'universe'"),
            ParserError::MissingMetadata => write!(f, "Falta bloque 'metadata'"),
            ParserError::MissingSun => write!(f, "Falta declaración 'sun'"),
            ParserError::MissingPosition => write!(f, "Campo 'position' obligatorio"),
            ParserError::MissingMass => write!(f, "Campo 'mass' obligatorio"),
            ParserError::MissingType => write!(f, "Campo 'type' obligatorio"),
            ParserError::InvalidIdentifier => write!(f, "Identificador inválido"),
            ParserError::InvalidNumber => write!(f, "Número mal formado"),
            ParserError::InvalidString => write!(f, "Cadena mal formada"),
            ParserError::SunPositionOverride => write!(f, "Sun no puede tener posición declarada"),
            ParserError::DuplicateEntity => write!(f, "Identificador duplicado"),
            ParserError::UnknownBlock => write!(f, "Bloque desconocido"),
            ParserError::InvalidFlowType => write!(f, "Tipo de flujo inválido: usar Primary, Asynchronous o Read"),
        }
    }
}


impl std::error::Error for ParserError {}


// ============================================================
// 3. ERRORES SEMÁNTICOS (S001-S015)
// Consolidado desde RFC-0006
// ============================================================
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticError {
    // === ERRORES DE IDENTIDAD ===
    
    /// S001: ID de nodo duplicado
    DuplicateNodeId(u128),
    
    /// S002: Referencia a nodo inexistente
    MissingReference(u128),
    
    /// S003: Satellite no apunta al planeta correcto
    InvalidSatelliteParent {
        satellite: u128,
        expected: u128,
        found: u128,
    },
    
    // === ERRORES DE FLUJO ===
    
    /// S004: Flujo con origen inválido
    InvalidFlowSource(u128),
    
    /// S005: Flujo con destino inválido
    InvalidFlowTarget(u128),
    
    /// S006: Matriz de interacción violada
    FlowViolation {
        from: u128,
        to: u128,
        reason: String,
    },
    
    // === ERRORES DE MASA ===
    
    /// S007: Masa satelital excede proporción permitida (50%)
    MassMismatch {
        planet: u128,
        planet_mass: u32,
        satellites_mass: u32,
        max_allowed: u32,
    },
    
    // === ERRORES DE ÓRBITA ===
    
    /// S008: PlanetType incompatible con banda orbital
    OrbitTypeMismatch {
        planet: u128,
        planet_type: String,
        orbit: OrbitBand,
    },
    
    // === ERRORES ESTRUCTURALES ===
    
    /// S009: Universo vacío (sin galaxias)
    EmptyUniverse,
    
    /// S010: Posición inválida
    InvalidPosition(u128),
    
    /// S011: Violación de aislamiento galáctico
    GalacticIsolationViolation {
        from: u128,
        to: u128,
    },
    
    // === ERRORES DE CICLO ===
    
    /// S012: Bucle infinito sin condición de salida
    InfiniteOrbitLoop {
        cycle: Vec<u128>,
    },
    
    // === ERRORES DE VALIDACIÓN TOPOLÓGICA ===
    
    /// S013: Error topológico genérico
    Topology(TopologyError),
}


impl From<TopologyError> for SemanticError {
    fn from(error: TopologyError) -> Self {
        SemanticError::Topology(error)
    }
}


impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticError::DuplicateNodeId(id) => 
                write!(f, "ID duplicado: {:x}", id),
            SemanticError::MissingReference(id) => 
                write!(f, "Referencia inválida: {:x}", id),
            SemanticError::InvalidSatelliteParent { satellite, expected, found } => 
                write!(f, "Satélite {:x} apunta a {:x}, esperado {:x}", satellite, found, expected),
            SemanticError::InvalidFlowSource(id) => 
                write!(f, "Flujo con origen inválido: {:x}", id),
            SemanticError::InvalidFlowTarget(id) => 
                write!(f, "Flujo con destino inválido: {:x}", id),
            SemanticError::FlowViolation { from, to, reason } => 
                write!(f, "Flujo {:x} → {:x} violado: {}", from, to, reason),
            SemanticError::MassMismatch { planet, planet_mass, satellites_mass, max_allowed } => 
                write!(f, "Masa satelital {} > máximo {} (planeta: {})", 
                       satellites_mass, max_allowed, planet),
            SemanticError::OrbitTypeMismatch { planet, planet_type, orbit } => 
                write!(f, "PlanetType '{}' incompatible con órbita {:?}", planet_type, orbit),
            SemanticError::EmptyUniverse => write!(f, "Universo sin galaxias"),
            SemanticError::InvalidPosition(id) => write!(f, "Posición inválida para {:x}", id),
            SemanticError::GalacticIsolationViolation { from, to } => 
                write!(f, "Violación de aislamiento: {:x} → {:x}", from, to),
            SemanticError::InfiniteOrbitLoop { cycle } => {
                let ids: Vec<String> = cycle.iter().map(|x| format!("{:x}", x)).collect();
                write!(f, "Bucle infinito detectado: {}", ids.join(" → "))
            }
            SemanticError::Topology(e) => write!(f, "Error topológico: {}", e),
        }
    }
}


impl std::error::Error for SemanticError {}


// ============================================================
// 4. ERRORES CIR (C001-C010)
// Consolidado desde RFC-0007
// ============================================================
#[derive(Debug, Clone, PartialEq)]
pub enum CirError {
    /// C001: AST inválido recibido
    InvalidAst(Vec<SemanticError>),
    
    /// C002: Ruta de luz muy larga
    LightPathTooLong {
        from: u128,
        to: u128,
        distance: f64,
        reach: f64,
    },
    
    /// C003: Nodo en DeepSpace
    DeepSpaceViolation(u128),
    
    /// C004: Colisión de masa
    MassCollision {
        a: u128,
        b: u128,
        distance: f64,
    },
    
    /// C005: Tensión excesiva (demasiados cruces)
    SpaghettiTension {
        crossings: usize,
        threshold: usize,
    },
    
    /// C006: Bucle infinito sin salida
    InfiniteOrbitLoop {
        cycle: Vec<u128>,
    },
    
    /// C007: Error de serialización
    SerializationError(String),
    
    /// C008: Hash no coincide (corrupción)
    HashMismatch {
        expected: String,
        found: String,
    },
    
    /// C009: Versión incompatible
    VersionMismatch {
        expected: String,
        found: String,
    },
    
    /// C010: Entidad no encontrada
    EntityNotFound(u128),
}


impl std::fmt::Display for CirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CirError::InvalidAst(_) => write!(f, "AST inválido"),
            CirError::LightPathTooLong { from, to, distance, reach } => 
                write!(f, "Ruta {:x} → {:x} muy larga: {:.2} > {:.2}", 
                       from, to, distance, reach),
            CirError::DeepSpaceViolation(id) => 
                write!(f, "Entidad {:x} en DeepSpace", id),
            CirError::MassCollision { a, b, distance } => 
                write!(f, "Colisión {:x} - {:x}: distancia {:.4}", a, b, distance),
            CirError::SpaghettiTension { crossings, threshold } => 
                write!(f, "Tensión {} > umbral {}", crossings, threshold),
            CirError::InfiniteOrbitLoop { cycle } => {
                let ids: Vec<String> = cycle.iter().map(|x| format!("{:x}", x)).collect();
                write!(f, "Bucle infinito: {}", ids.join(" → "))
            }
            CirError::SerializationError(msg) => write!(f, "Error de serialización: {}", msg),
            CirError::HashMismatch { expected, found } => 
                write!(f, "Hash no coincide: esperado {}, encontrado {}", expected, found),
            CirError::VersionMismatch { expected, found } => 
                write!(f, "Versión incompatible: esperado {}, encontrado {}", expected, found),
            CirError::EntityNotFound(id) => write!(f, "Entidad no encontrada: {:x}", id),
        }
    }
}


impl std::error::Error for CirError {}


// ============================================================
// 5. SISTEMA DE COORDENADAS ESPACIALES
// Posición = Significado: radio = alcance/prioridad, ángulo = posición relativa
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolarPosition {
    pub radius: f64,   // Distancia al Sol central → Nivel de abstracción y alcance
    pub theta: f64,    // Ángulo en radianes → Posición dentro de la misma órbita
}


impl PolarPosition {
    /// Crea y valida una nueva posición polar
    pub fn new(radius: f64, theta: f64) -> Result<Self, TopologyError> {
        let pos = Self { radius, theta };
        pos.validate()?;
        Ok(pos)
    }

    /// Convierte a coordenadas cartesianas para renderizado visual  
    pub fn to_cartesian(&self) -> (f64, f64) {  
        let x = self.radius * self.theta.cos();  
        let y = self.radius * self.theta.sin();  
        (x, y)  
    }  
  
    /// Distancia geométrica entre dos puntos → también distancia semántica  
    pub fn distance(&self, other: &PolarPosition) -> f64 {  
        let (x1, y1) = self.to_cartesian();  
        let (x2, y2) = other.to_cartesian();  
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()  
    }  
  
    /// Validación formal: ninguna posición viola las reglas de geometría  
    pub fn validate(&self) -> Result<(), TopologyError> {  
        if self.radius < 0.0 || !self.theta.is_finite() {  
            return Err(TopologyError::InvalidCoordinates);  
        }  
        if self.radius > 100.0 {
            return Err(TopologyError::DeepSpaceViolation);
        }
        Ok(())  
    }
}


// ============================================================
// 6. JERARQUÍA ORBITAL (BANDAS DE ABSTRACCIÓN)
// La posición define automáticamente el rol y permisos del módulo
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrbitBand {
    Core,        // 0.0 - 10.0   : Lógica central, estado inmutable, reglas fundamentales
    Logic,       // 10.1 - 50.0  : Reglas de negocio, procesamiento principal
    Interface,   // 50.1 - 100.0 : Entradas/Salidas, UI, adaptadores externos
    DeepSpace,   // > 100.0      : Fuera de los límites del universo, no válido
}


impl OrbitBand {
    /// Deriva la banda semántica directamente de la distancia al centro
    pub fn from_radius(radius: f64) -> Self {
        match radius {
            r if r <= 10.0 => OrbitBand::Core,
            r if r <= 50.0 => OrbitBand::Logic,
            r if r <= 100.0 => OrbitBand::Interface,
            _ => OrbitBand::DeepSpace,
        }
    }

    /// Devuelve el rango de radio permitido para esta banda  
    pub fn radius_range(&self) -> std::ops::Range<f64> {  
        match self {  
            OrbitBand::Core => 0.0..10.1,  
            OrbitBand::Logic => 10.1..50.1,  
            OrbitBand::Interface => 50.1..100.1,  
            OrbitBand::DeepSpace => 100.1..f64::MAX,  
        }  
    }
}


// ============================================================
// 7. EL CONTRATO DE GEOMETRÍA
// Todo nodo en Cosmos debe cumplir que su forma define su significado
// ============================================================
pub trait TopologicalNode {
    /// Posición espacial (y semántica) del nodo
    fn position(&self) -> PolarPosition;

    /// Masa = cantidad de responsabilidades/alcance del nodo  
    fn mass(&self) -> u32;  
  
    /// Banda orbital se calcula automáticamente desde la posición  
    fn orbit(&self) -> OrbitBand {  
        OrbitBand::from_radius(self.position().radius)  
    }  
  
    /// Alcance máximo: distancia máxima a la que puede enviar luz/datos  
    fn max_reach(&self) -> f64 {
        BASE_CONSTANT / (self.mass().max(1) as f64)
    }
}


// ============================================================
// 8. VALIDADORES DEL FLUJO DE LUZ
// La luz = ejecución y datos; solo viaja si cumple las reglas de arquitectura
// ============================================================
pub trait LightFlowValidator {
    /// Distancia semántica entre dos nodos
    fn calculate_distance(node_a: &impl TopologicalNode, node_b: &impl TopologicalNode) -> f64 {
        node_a.position().distance(&node_b.position())
    }

    /// ✅ Regla de oro: la luz solo fluye si la arquitectura lo permite  
    fn can_transmit_light(emitter: &impl TopologicalNode, receiver: &impl TopologicalNode) -> Result<(), TopologyError> {  
        let distance = Self::calculate_distance(emitter, receiver);  
        let reach = emitter.max_reach();  

        // Regla 1: No se puede llegar más lejos que el alcance permitido  
        if distance > reach {  
            return Err(TopologyError::LightPathTooLong { distance, reach });  
        }  

        let orbit_emitter = emitter.orbit();  
        let orbit_receiver = receiver.orbit();  

        // Regla 2: Prohibido saltar capas de abstracción directamente al núcleo
        // Interface → Core requiere pasar por Logic obligatoriamente
        if orbit_emitter == OrbitBand::Interface && orbit_receiver == OrbitBand::Core {  
            return Err(TopologyError::OrbitViolation);  
        }  

        // Regla 3: Nodos en DeepSpace no pueden transmitir ni recibir  
        if orbit_emitter == OrbitBand::DeepSpace || orbit_receiver == OrbitBand::DeepSpace {  
            return Err(TopologyError::DeepSpaceViolation);  
        }  

        Ok(())  
    }  

    /// ✅ Detección de colisiones: dos nodos en la misma posición = mala arquitectura  
    fn check_collision(node_a: &impl TopologicalNode, node_b: &impl TopologicalNode) -> Result<(), TopologyError> {  
        if node_a.position().distance(&node_b.position()) < EPSILON {  
            return Err(TopologyError::MassCollision);  
        }  
        Ok(())  
    }
}


// ============================================================
// 9. DETECCIÓN DE CRUCES Y TENSIÓN
// ============================================================
pub struct CosmosTopology;


impl LightFlowValidator for CosmosTopology {}


impl CosmosTopology {
    /// Comprueba si dos líneas de conexión se cruzan en el espacio
    fn lines_intersect(
        a1: PolarPosition, a2: PolarPosition,
        b1: PolarPosition, b2: PolarPosition
    ) -> bool {
        let (x1, y1) = a1.to_cartesian();
        let (x2, y2) = a2.to_cartesian();
        let (x3, y3) = b1.to_cartesian();
        let (x4, y4) = b2.to_cartesian();

        // Fórmula estándar de intersección de segmentos  
        let denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);  
        if denom.abs() < 1e-9 {  
            return false; // Paralelas o coincidentes  
        }  

        let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denom;  
        let ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denom;  

        ua > 1e-9 && ua < 1.0 - 1e-9 && ub > 1e-9 && ub < 1.0 - 1e-9  
    }  

    /// Calcula el número total de cruces entre todas las conexiones  
    pub fn count_crossings(connections: &[(&impl TopologicalNode, &impl TopologicalNode)]) -> usize {  
        let mut crossings = 0;  
        let n = connections.len();  

        for i in 0..n {  
            for j in (i + 1)..n {  
                let (a, b) = connections[i];  
                let (c, d) = connections[j];  
                if Self::lines_intersect(a.position(), b.position(), c.position(), d.position()) {  
                    crossings += 1;  
                }  
            }  
        }  
        crossings  
    }  

    /// Evalúa la tensión de la red: cruces por conexión  
    pub fn check_tension(  
        connections: &[(&impl TopologicalNode, &impl TopologicalNode)],  
        threshold: usize,
    ) -> Result<(), TopologyError> {  
        let total = Self::count_crossings(connections);  
        if total > threshold {  
            return Err(TopologyError::SpaghettiTensionExceeded { 
                crossings: total, 
                threshold 
            });  
        }  
        Ok(())  
    }
}


// ============================================================
// 10. TIPOS DE FLUJO OFICIALES (RFC-0004-A)
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowType {
    /// Primary: Flujo principal de ejecución
    Primary,
    /// Asynchronous: Ejecución paralela, no bloqueante
    Asynchronous,
    /// Read: Solo lectura de datos
    Read,
}


impl FlowType {
    /// Convierte desde string (parser)
    pub fn from_str(s: &str) -> Result<Self, ParserError> {
        match s {
            "Primary" => Ok(FlowType::Primary),
            "Asynchronous" => Ok(FlowType::Asynchronous),
            "Read" => Ok(FlowType::Read),
            _ => Err(ParserError::InvalidFlowType),
        }
    }

    /// Convierte a string
    pub fn as_str(&self) -> &'static str {
        match self {
            FlowType::Primary => "Primary",
            FlowType::Asynchronous => "Asynchronous",
            FlowType::Read => "Read",
        }
    }
}


// ============================================================
// 11. TIPOS PLANETARIOS (RFC-0006)
// ============================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanetType {
    Storage,      // Almacenamiento de datos
    Operation,    // Operaciones de negocio
    Compute,      // Cálculo/procesamiento
    Interface,    // Interfaces de entrada/salida
}


impl PlanetType {
    /// Órbitas permitidas para cada tipo de planeta
    pub fn allowed_orbits(&self) -> Vec<OrbitBand> {
        match self {
            PlanetType::Storage => vec![OrbitBand::Logic],
            PlanetType::Operation => vec![OrbitBand::Logic],
            PlanetType::Compute => vec![OrbitBand::Logic],
            PlanetType::Interface => vec![OrbitBand::Interface],
        }
    }

    /// Verifica si una órbita es válida para este tipo
    pub fn is_valid_orbit(&self, orbit: OrbitBand) -> bool {
        self.allowed_orbits().contains(&orbit)
    }
}


// ============================================================
// 12. MATRIZ DE INTERACCIÓN DE FLUJO (RFC-0006)
// Ningún permiso implícito. Lo que no está escrito está prohibido.
// ============================================================
pub fn flow_allowed(emitter: PlanetType, receiver: PlanetType) -> bool {
    match (emitter, receiver) {
        // Datos salen del almacenamiento
        (PlanetType::Storage, PlanetType::Operation) => true,
        (PlanetType::Storage, PlanetType::Compute) => true,

        // Procesamiento
        (PlanetType::Operation, PlanetType::Compute) => true,
        (PlanetType::Operation, PlanetType::Interface) => true,

        // Cálculo hacia salida
        (PlanetType::Compute, PlanetType::Interface) => true,

        // Storage directo a Interface: PROHIBIDO
        (PlanetType::Storage, PlanetType::Interface) => false,

        // Todo lo demás prohibido
        _ => false,
    }
}


// ============================================================
// 13. GRAFO DE FLUJO (RFC-0006)
// ============================================================
#[derive(Debug, Clone)]
pub struct FlowGraph {
    edges: std::collections::HashMap<u128, Vec<u128>>,
}


impl FlowGraph {
    pub fn new() -> Self {
        Self {
            edges: std::collections::HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: u128, to: u128) {
        self.edges
            .entry(from)
            .or_default()
            .push(to);
    }

    /// Detecta ciclos usando DFS con stack de recursión
    pub fn has_cycle(&self) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut stack = std::collections::HashSet::new();

        for node in self.edges.keys() {
            if self.detect(*node, &mut visited, &mut stack) {
                return true;
            }
        }
        false
    }

    /// Retorna el ciclo encontrado si existe
    pub fn find_cycle(&self) -> Option<Vec<u128>> {
        let mut visited = std::collections::HashSet::new();
        let mut stack = std::collections::HashSet::new();
        let mut path = Vec::new();

        for node in self.edges.keys() {
            if self.detect_with_path(*node, &mut visited, &mut stack, &mut path) {
                return Some(path.clone());
            }
        }
        None
    }

    fn detect(&self, node: u128, visited: &mut std::collections::HashSet<u128>, stack: &mut std::collections::HashSet<u128>) -> bool {
        if stack.contains(&node) {
            return true;
        }
        if visited.contains(&node) {
            return false;
        }

        visited.insert(node);
        stack.insert(node);

        if let Some(children) = self.edges.get(&node) {
            for child in children {
                if self.detect(*child, visited, stack) {
                    return true;
                }
            }
        }

        stack.remove(&node);
        false
    }

    fn detect_with_path(&self, node: u128, visited: &mut std::collections::HashSet<u128>, stack: &mut std::collections::HashSet<u128>, path: &mut Vec<u128>) -> bool {
        if stack.contains(&node) {
            // Encontramos el inicio del ciclo
            if let Some(pos) = path.iter().position(|&x| x == node) {
                path.push(node); // Incluir el nodo de inicio para cerrar el ciclo
                path.rotate_left(pos);
            }
            return true;
        }
        if visited.contains(&node) {
            return false;
        }

        visited.insert(node);
        stack.insert(node);
        path.push(node);

        if let Some(children) = self.edges.get(&node) {
            for child in children {
                if self.detect_with_path(*child, visited, stack, path) {
                    return true;
                }
            }
        }

        stack.remove(&node);
        path.pop();
        false
    }
}


impl Default for FlowGraph {
    fn default() -> Self {
        Self::new()
    }
}


// ============================================================
// 14. UTILIDADES DE TOPOLOGÍA
// ============================================================

/// Calcula la masa total de satellites
pub fn calculate_satellites_mass(satellites: &[impl TopologicalNode]) -> u32 {
    satellites.iter().map(|s| s.mass()).sum()
}

/// Valida que la masa satelital no exceda el ratio permitido
pub fn validate_satellite_mass_ratio(planet_mass: u32, satellites_mass: u32) -> Result<(), SemanticError> {
    let max_mass = (planet_mass as f64 * SATELLITE_MASS_RATIO) as u32;
    if satellites_mass <= max_mass {
        Ok(())
    } else {
        Err(SemanticError::MassMismatch {
            planet: 0, // Se填补 en el contexto
            planet_mass,
            satellites_mass,
            max_allowed: max_mass,
        })
    }
}

/// Verifica si dos galaxias están suficientemente separadas
pub fn check_galactic_separation(a: &impl TopologicalNode, b: &impl TopologicalNode) -> Result<(), TopologyError> {
    if a.position().distance(&b.position()) < GALACTIC_EPSILON {
        return Err(TopologyError::MassCollision);
    }
    Ok(())
}


// ============================================================
// TESTS
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_band_from_radius() {
        assert_eq!(OrbitBand::from_radius(5.0), OrbitBand::Core);
        assert_eq!(OrbitBand::from_radius(25.0), OrbitBand::Logic);
        assert_eq!(OrbitBand::from_radius(75.0), OrbitBand::Interface);
        assert_eq!(OrbitBand::from_radius(150.0), OrbitBand::DeepSpace);
    }

    #[test]
    fn test_polar_position_validation() {
        // Válido
        assert!(PolarPosition::new(25.0, 1.57).is_ok());
        
        // Radio negativo
        assert_eq!(
            PolarPosition::new(-5.0, 0.0).unwrap_err(),
            TopologyError::InvalidCoordinates
        );
        
        // Theta infinito
        assert_eq!(
            PolarPosition::new(25.0, f64::INFINITY).unwrap_err(),
            TopologyError::InvalidCoordinates
        );
        
        // DeepSpace
        assert_eq!(
            PolarPosition::new(150.0, 0.0).unwrap_err(),
            TopologyError::DeepSpaceViolation
        );
    }

    #[test]
    fn test_flow_type_from_str() {
        assert_eq!(FlowType::from_str("Primary").unwrap(), FlowType::Primary);
        assert_eq!(FlowType::from_str("Asynchronous").unwrap(), FlowType::Asynchronous);
        assert_eq!(FlowType::from_str("Read").unwrap(), FlowType::Read);
        assert!(FlowType::from_str("Sync").is_err());
    }

    #[test]
    fn test_flow_matrix() {
        // Permitido
        assert!(flow_allowed(PlanetType::Storage, PlanetType::Operation));
        assert!(flow_allowed(PlanetType::Storage, PlanetType::Compute));
        assert!(flow_allowed(PlanetType::Operation, PlanetType::Compute));
        assert!(flow_allowed(PlanetType::Compute, PlanetType::Interface));
        
        // Prohibido
        assert!(!flow_allowed(PlanetType::Storage, PlanetType::Interface));
        assert!(!flow_allowed(PlanetType::Interface, PlanetType::Storage));
        assert!(!flow_allowed(PlanetType::Interface, PlanetType::Operation));
    }

    #[test]
    fn test_flow_graph_cycle() {
        let mut graph = FlowGraph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1); // Ciclo
        
        assert!(graph.has_cycle());
    }

    #[test]
    fn test_flow_graph_no_cycle() {
        let mut graph = FlowGraph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        
        assert!(!graph.has_cycle());
    }

    #[test]
    fn test_planet_type_orbits() {
        assert!(PlanetType::Storage.is_valid_orbit(OrbitBand::Logic));
        assert!(!PlanetType::Storage.is_valid_orbit(OrbitBand::Interface));
        
        assert!(PlanetType::Interface.is_valid_orbit(OrbitBand::Interface));
        assert!(!PlanetType::Interface.is_valid_orbit(OrbitBand::Logic));
    }

    #[test]
    fn test_topology_error_display() {
        let err = TopologyError::OrbitViolation;
        assert!(!format!("{}", err).is_empty());
        
        let err = TopologyError::LightPathTooLong { distance: 10.0, reach: 5.0 };
        assert!(format!("{}", err).contains("10.00"));
        
        let err = TopologyError::SpaghettiTensionExceeded { crossings: 15, threshold: 10 };
        assert!(format!("{}", err).contains("15"));
    }

    #[test]
    fn test_parser_error_display() {
        let err = ParserError::UnexpectedToken;
        assert!(!format!("{}", err).is_empty());
        
        let err = ParserError::InvalidFlowType;
        assert!(format!("{}", err).contains("Primary"));
    }

    #[test]
    fn test_semantic_error_display() {
        let err = SemanticError::DuplicateNodeId(42);
        assert!(format!("{}", err).contains("2a"));
        
        let err = SemanticError::InfiniteOrbitLoop { cycle: vec![1, 2, 3] };
        assert!(format!("{}", err).contains("→"));
    }

    #[test]
    fn test_semantic_error_from_topology() {
        let topo_err = TopologyError::OrbitViolation;
        let sem_err: SemanticError = topo_err.into();
        assert!(matches!(sem_err, SemanticError::Topology(_)));
    }

    #[test]
    fn test_flow_type_as_str() {
        assert_eq!(FlowType::Primary.as_str(), "Primary");
        assert_eq!(FlowType::Asynchronous.as_str(), "Asynchronous");
        assert_eq!(FlowType::Read.as_str(), "Read");
    }

    #[test]
    fn test_flow_type_from_str_valid() {
        assert_eq!(FlowType::from_str("Primary").unwrap(), FlowType::Primary);
        assert_eq!(FlowType::from_str("Asynchronous").unwrap(), FlowType::Asynchronous);
        assert_eq!(FlowType::from_str("Read").unwrap(), FlowType::Read);
    }

    #[test]
    fn test_flow_type_from_str_invalid() {
        assert!(FlowType::from_str("Invalid").is_err());
        assert!(FlowType::from_str("Sync").is_err());
        assert!(FlowType::from_str("").is_err());
    }

    #[test]
    fn test_flow_graph_find_cycle() {
        let mut graph = FlowGraph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);
        
        let cycle = graph.find_cycle();
        assert!(cycle.is_some());
        let c = cycle.unwrap();
        assert!(c.len() >= 3);
    }

    #[test]
    fn test_flow_graph_find_no_cycle() {
        let mut graph = FlowGraph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        
        assert!(graph.find_cycle().is_none());
    }

    #[test]
    fn test_planet_type_allowed_orbits() {
        let storage_orbits = PlanetType::Storage.allowed_orbits();
        assert!(storage_orbits.contains(&OrbitBand::Logic));
        assert!(!storage_orbits.contains(&OrbitBand::Interface));
        
        let interface_orbits = PlanetType::Interface.allowed_orbits();
        assert!(interface_orbits.contains(&OrbitBand::Interface));
        assert!(!interface_orbits.contains(&OrbitBand::Logic));
    }

    #[test]
    fn test_polar_position_distance() {
        let pos1 = PolarPosition { radius: 0.0, theta: 0.0 };
        let pos2 = PolarPosition { radius: 10.0, theta: 0.0 };
        let dist = pos1.distance(&pos2);
        assert!(dist > 0.0);
    }

    #[test]
    fn test_polar_position_to_cartesian() {
        let pos = PolarPosition { radius: 1.0, theta: 0.0 };
        let (x, y) = pos.to_cartesian();
        assert!((x - 1.0).abs() < 0.001);
        assert!(y.abs() < 0.001);
    }

    #[test]
    fn test_validate_satellite_mass_ratio_valid() {
        let result = validate_satellite_mass_ratio(100, 50); // 50% of 100
        assert!(result.is_ok());
        
        let result = validate_satellite_mass_ratio(100, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_satellite_mass_ratio_invalid() {
        let result = validate_satellite_mass_ratio(100, 51); // over 50%
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_satellites_mass() {
        struct MockNode { mass: u32 }
        impl TopologicalNode for MockNode {
            fn position(&self) -> PolarPosition {
                PolarPosition { radius: 0.0, theta: 0.0 }
            }
            fn mass(&self) -> u32 { self.mass }
        }
        
        let satellites = vec![
            MockNode { mass: 10 },
            MockNode { mass: 20 },
            MockNode { mass: 30 },
        ];
        
        let total = calculate_satellites_mass(&satellites);
        assert_eq!(total, 60);
    }

    #[test]
    fn test_cosmos_topology_check_tension() {
        struct MockNode { pos: PolarPosition }
        impl TopologicalNode for MockNode {
            fn position(&self) -> PolarPosition { self.pos }
            fn mass(&self) -> u32 { 1 }
        }
        
        // Create nodes in a line (no crosses)
        let n1 = MockNode { pos: PolarPosition { radius: 10.0, theta: 0.0 } };
        let n2 = MockNode { pos: PolarPosition { radius: 20.0, theta: 0.0 } };
        let n3 = MockNode { pos: PolarPosition { radius: 30.0, theta: 0.0 } };
        let n4 = MockNode { pos: PolarPosition { radius: 40.0, theta: 0.0 } };
        
        let connections: Vec<(&MockNode, &MockNode)> = vec![
            (&n1, &n2),
            (&n3, &n4),
        ];
        
        let result = CosmosTopology::check_tension(&connections, 10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cir_error_display() {
        let err = CirError::DeepSpaceViolation(42);
        assert!(format!("{}", err).contains("2a"));
        
        let err = CirError::SerializationError("test".to_string());
        assert!(format!("{}", err).contains("test"));
    }

    #[test]
    fn test_orbit_band_radius_range() {
        let range = OrbitBand::Core.radius_range();
        assert!(range.contains(&0.0));
        assert!(range.contains(&5.0));
        assert!(!range.contains(&15.0));
        
        let range = OrbitBand::Logic.radius_range();
        assert!(range.contains(&25.0));
        
        let range = OrbitBand::Interface.radius_range();
        assert!(range.contains(&75.0));
    }

    #[test]
    fn test_topological_node_trait() {
        struct TestNode;
        impl TopologicalNode for TestNode {
            fn position(&self) -> PolarPosition {
                PolarPosition { radius: 25.0, theta: 1.0 } // Logic band (10.1 - 50.0)
            }
            fn mass(&self) -> u32 { 100 }
        }
        
        let node = TestNode;
        assert_eq!(node.orbit(), OrbitBand::Logic);
        assert_eq!(node.max_reach(), 1000.0 / 100.0); // BASE_CONSTANT / mass
    }
}
