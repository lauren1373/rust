///Authors: Lauren , Chris
/// Date: 02 Apr 23
/// Start the simulation
pub mod machine;
pub mod machine_data;
pub mod simulation;

use crate::simulation::Simulation;
use machine::Machine;
use machine_data::MachineData;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::env::args;
use std::sync::mpsc::channel;
use std::time::Instant;
use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
};
use threadpool::ThreadPool;

/// Starting point of the program.
fn main() {
    //Get command line arguments.
    let args: Vec<String> = args().collect();

    //Get the user input.
    let sim = user_input();

    //Populate the machine data.
    let machine = populate_machine_data(&sim, args);

    let now = Instant::now();
    {
        //Call function to start the simulation.
        run_simulation(&sim, &machine);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

/// Starts the simulation
///
/// # Arguments
///
/// * `sim` - A struct that represents the simulation of machines running on threads.
/// * `machine` - A class to represent finite state machine.
///
fn run_simulation(sim: &Simulation, machine: &MachineData) {
    let mut total: Vec<f32> = vec![0.0; machine.total_num_nodes];
    let float = sim.num_machines as f32;

    let number_of_threads = sim.num_threads as usize;
    println("{} thread in pool");
    let pool = ThreadPool::new(number_of_threads);
    let (sender, reciever) = channel();

    for id in 0..sim.num_machines {
        let var1 = sim.clone();
        let var2 = machine.clone();
        let tx = sender.clone();
        pool.execute(move || {
            let mut data = Machine::new(
                &var2.start,
                &var2.total_num_nodes,
                var2.probabilites.clone(),
                id,
            );
            let mut return_vec = data.cycle(var1.num_cycles);
            return_vec.insert(0, data.unique_id as f32);
            tx.send(return_vec).err();
        });
    }

    drop(sender);

    print!("Steady state results: \n");
    for recvd in reciever {
        for n in 1..recvd.len() {
            total[n - 1] += recvd[n];
        }
        //println!("Recieved {}!\n", recvd[0]);   // Uncomment to show concurrency.
    }
    //pool.join();

    for l in 0..total.len() {
        total[l] = total[l] / float;
        println!("State {} : {}% ", l, total[l]);
    }
    //pool.join();
    print!("\n");
}

/// Get the user input and build the simulation struct.
///
/// # Return Value
///
/// A populated Simulation struct.
fn user_input() -> Simulation {
    //Create a simulation struct and put data in it.
    let mut simulation = Simulation::new();

    //Buffer for user input.
    let mut input_buffer = String::new();

    //Put the number of finite state machines into simulation struct.
    println!("How many Finite State Machines to create? > ");
    stdout().flush().expect("Failed to flush stdout");
    stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read user input");
    simulation.num_machines = input_buffer
        .trim()
        .parse::<i32>()
        .expect("Failed to parse input");

    //Put the number of cycles into simulation struct.
    println!("How many iterations for each machine? > ");
    stdout().flush().expect("Failed to flush stdout");
    input_buffer = String::new();
    stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read user input");
    simulation.num_cycles = input_buffer
        .trim()
        .parse::<i32>()
        .expect("Failed to parse input");

    //Put the number of threads into simulation struct.
    println!("How many threads? > ");
    stdout().flush().expect("Failed to flush stdout");
    input_buffer = String::new();
    stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read user input");
    simulation.num_threads = input_buffer
        .trim()
        .parse::<i32>()
        .expect("Failed to parse input");

    if simulation.num_threads > simulation.num_machines{
        print!("Number of threads is greater than # of machines, this is inefficient and wastes space.\n");
    }

    //Put the filename into the simulation struct.
    println!("Please enter input filename? > ");
    stdout().flush().expect("Failed to flush stdout");
    input_buffer = String::new();
    stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read user input");
    let file = input_buffer.trim();
    simulation.filename = file.to_string();

    simulation
}

///Build the machine Data struct using the input file.
///
/// # Arguments
///
/// * `simulation` - A struct that represents the simulation of machines running on threads.
/// * `arguments` - Array of command line arguments.
///
/// # Return Value
///
/// A populated Simulation struct.
fn populate_machine_data(simulation: &Simulation, arguments: Vec<String>) -> MachineData {
    let mut data = MachineData::new();

    let file = File::open(&simulation.filename).expect("Failed to open file");

    //Add the total number of nodes to the struct Machine Data
    let mut read = BufReader::new(&file);
    let mut buf = String::new();
    read.read_line(&mut buf).expect("Failed to read file");
    let new_str = buf.trim();
    data.total_num_nodes = new_str.parse::<usize>().unwrap();

    //Set start state for finite state machine.
    if arguments.len() > 1 {
        //Make sure user input start state is within the bounds of the 2d matrix
        assert!(
            arguments[1].parse::<usize>().expect("Failed to parse!") < data.total_num_nodes,
            "Number is larger than the matrix"
        );

        data.start = arguments[1].parse::<usize>().expect("Failed to parse!");
    } else {
        println!("No start state given, starting from a random state\n");
        let mut small_rng = SmallRng::from_entropy();
        data.start = small_rng.gen_range(0..data.total_num_nodes);
    }
    //Set Machine Data id.
    data.id = "1".to_string();

    //loop through the file and create 2d matrix.
    for line in read.lines() {
        let line_str = line.unwrap();
        let s = line_str.trim().rsplit(" ");
        let mut f32_vec: Vec<f32> = Vec::new();
        //loop through a line character by character and add the character to our Vec<f32> if it is not a space.
        for char in s {
            if !char.is_empty() {
                f32_vec.insert(0, char.parse::<f32>().unwrap());
            }
        }
        //Check the matrix is a 2d square.
        assert!(
            data.total_num_nodes == f32_vec.len(),
            "Matrix is not square!"
        );

        //Push a vector of floats into our matrix.
        data.probabilites.push(f32_vec);
    }
    //println!("{:}\n {:}\n", data.total_num_nodes, data.probabilites.len());

    data
}
