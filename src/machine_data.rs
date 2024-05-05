///Authors: Lauren , Chris
/// Date: 02 Apr 23
/// Struct to represent finite state machine data.

/// The machine data for a finite state machine
#[derive(Debug, Clone)]
pub struct MachineData {
    pub id: String,
    pub start: usize,
    pub total_num_nodes: usize,
    pub probabilites: Vec<Vec<f32>>,
}

impl MachineData {
    /// Create a new finite state machine.
    ///
    /// # Return Value
    ///
    /// A finite state machine.
    pub const fn new() -> Self {
        MachineData {
            id: String::new(),
            start: 0,
            total_num_nodes: 0,
            probabilites: Vec::new(),
        }
    }
}
