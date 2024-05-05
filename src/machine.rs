///Authors: Lauren , Chris
/// Date: 02 Apr 23
/// Class to represent finite state machine.
///
use rand::{distributions::Uniform, thread_rng, Rng};

/// A struct representing a finite state machine.
#[derive(Debug, Clone)]
pub struct Machine {
    pub unique_id: i32,
    curr_state: usize,
    nodes: usize,
    probabilites: Vec<Vec<f32>>,
}

impl Machine {
    /// Create a new machine.
    ///
    /// # Arguments
    ///
    /// * `start` - Start state for this machine.
    /// * `total` - Total number of states for this machine.
    /// * `probs` - 2D Matrix representing a weighted state diagram.
    /// * `id` - unique ID for this machine
    ///
    /// # Return Value
    ///
    /// A new machine
    pub const fn new(start: &usize, total: &usize, probs: Vec<Vec<f32>>, id: i32) -> Self {
        Machine {
            curr_state: *start,
            nodes: *total,
            probabilites: probs,
            unique_id: id,
        }
    }

    /// Run through state transitions until a stop point is determined.
    ///
    /// # Arguments
    ///
    /// * `num_of_cycles` - number of times to run through the state transitions.
    ///
    /// # Return Value
    ///
    /// A vector of floating point values representing the total probabilities of visting each node.
    pub fn cycle(&mut self, num_of_cycles: i32) -> Vec<f32> {
        let mut visits = vec![0.0; self.nodes];
        let mut probs: Vec<f32> = Vec::new();
        let num = num_of_cycles as f32;
        println("Number of cycles is {}", num_of_cycles);
        for _ in 0..num_of_cycles {
            let col = self.get_column();
            self.curr_state = self.get_probs(col);
            visits[self.curr_state] += 1.0;
        }
        for j in 0..visits.len() {
            probs.push(visits[j] / num);
        }
        //println!("Machine {} is done!\n", self.unique_id);
        //Uncomment this code to show concurrency.
        probs
    }

    /// Get a column of the 2D Matrix which is the current state.
    ///
    /// # Return Value
    ///
    /// A vector of floating point values representing the current states column.
    pub fn get_column(&self) -> Vec<f32> {
        let mut col: Vec<f32> = Vec::new();
        //Get the columns of the probabilities.
        for i in 0..self.probabilites.len() {
            col.push(self.probabilites[i][self.curr_state]);
        }
        col
    }

    /// Get the probablity of transitioning from one state to another.
    ///
    /// # Arguments
    ///
    /// * `col` - A vector of floating point values representing the probability of transitioning to another state.
    ///
    /// # Return Value
    ///
    /// A value representing the node we move to next.
    pub fn get_probs(&self, col: Vec<f32>) -> usize {
        let mut bool: bool = false;
        let mut node;
        let mut rang = rand::thread_rng();
        while !bool {
            let uni_dist = rang.sample(Uniform::new(0, self.nodes));
            let route = col[uni_dist] as f64;
            let mut rng = thread_rng();
            bool = rng.gen_bool(route);
            node = uni_dist;
            if bool {
                return node;
            }
        }
        return self.curr_state;
    }
}
