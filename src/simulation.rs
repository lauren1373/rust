///Authors: Lauren , Chris
/// Date: 02 Apr 23
/// Struct to represent simulation of machines running on threads.

/// A struct that represents a simulation of machines running in multiple threads.
#[derive(Debug, Clone)]
pub struct Simulation {
    pub num_machines: i32,
    pub num_cycles: i32,
    pub num_threads: i32,
    pub filename: String,
}

impl Simulation {
    /// Create a new simulation.
    ///
    /// # Return Value
    ///
    /// A simulation
    pub const fn new() -> Self {
        Simulation {
            num_machines: 0,
            num_cycles: 0,
            num_threads: 0,
            filename: String::new(),
        }
    }
}
