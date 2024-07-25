// Author: Tom Powell
// Date: 2024-07-24
// Purpose: Main file of Recovery Dispersion Estimator.
mod distributions;
mod types;
mod wind;
use core::panic;
use std::{io::{self, Read}, process::exit};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use types::{float, v3, mag3};
#[derive(Debug, Serialize, Deserialize)]
enum DragElement {
    Parachute {
        diameter: f32,
        cd_par: f32,
        mass: f32,
    },
    Tube {
        diameter: f32,
        mass: f32,
    },
    Fins {
        width: f32,
        height: f32,
        count: u32,
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Sim {
    iterations: usize,
    starting_altitude: f32,
    wind: wind::Wind,
    drag_elements: Vec<DragElement>
}
impl Sim {
    fn f_drag (c_d: float, rho: float, u: v3, a_c: float) -> float {
        let u_abs = mag3(u);
        0.5 * c_d * rho * u_abs * u_abs * a_c
    }
    fn collapse_elements(&mut self) -> () {
        
    }

}


fn main() {
    print!("{}",types::logo);
    println!("Recovery Dispersion Estimator");
    println!("2024 Tom Powell");
    println!("Select operation");
    println!("[R]un file | [G]enerate template files | [E]xit");
    let mut op = String::new();
    io::stdin().read_line(& mut op).expect("Failed to read stdin");
    let mut filepath = String::new();
    match op.as_str() {
        "R" => {
            io::stdin().read_line(& mut filepath).expect("Must input a file path.");
            run(filepath);
            
        },
        "G" => {
            io::stdin().read_line(& mut filepath).expect("Must input a file path.");

        },
        "E" => {exit(0)},
        _ => {
            println!("Invalid operation.");
            exit(1)
        }
    }
    println!("Enter control file path:");
}
fn run(s: String) -> !{
    let mut file = match std::fs::File::open(s) {
        Ok(f) => f,
        Err(e) => panic!("{}",e)
    };
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    let simulation: Sim = match from_str::<Sim>(text.as_str()) {
        Ok(s) => s,
        Err(why) => panic!("{}",why),
    };

    exit(0);
}
