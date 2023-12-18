use rust_xlsxwriter::*; 

//Constants for Adams-Bashforth method found here: fncbook.github.io/fnc/ivp/multistep.html
const B_0:f64 = -0.375;
const B_1:f64 = 1.541666667;
const B_2:f64 = -2.45833333;
const B_3:f64 = 2.291666667;

//Time constants
const dt:f64 = 0.005;
const END_TIME:f64 = 10.0;

//Initial conditions and constants (spring = K, damping = C)
const INITIAL_VELOCITY: f64 = 0.0;
const INITIAL_POSITION: f64 = 1.0;
const K: f64 = 10.0;
const MASS: f64 = 1.0;
const C: f64 = 1.0;

//Sets the damping function for the simulation
fn damping(velocity: &f64) -> f64{
    return -C * velocity;
}

//Solves the diff eq using Euler's method
fn euler_method (pos: &mut Vec<f64>, velo: &mut Vec<f64>, accel: &Vec<f64>) {
    //Gets the change in velocity, adds to the previous velocity, and then appends the velocity vector
    let dv = accel[accel.len() - 1] * dt;
    let v_0 = velo[velo.len() - 1];
    velo.push(v_0 + dv);

    //Gets the change in position and previous position, appends their sum
    let dx = velo[velo.len() - 1] * dt;
    let x_0 = pos[pos.len() - 1];
    pos.push(x_0 + dx);
}

//Solves the diff eq using Adams-Bashforth
fn ab_method (pos: &mut Vec<f64>, velo: &mut Vec<f64>, accel: &Vec<f64>) {
    //Gets the last four elements of the acceleration vector and computes the new velocity using AB formula
    let last_four: Vec<_> = accel.iter().rev().take(4).collect::<Vec<_>>().iter().rev().map(|v| **v).collect();
    let v_0 = velo.last().unwrap();
    velo.push(v_0 + dt * (B_0 * last_four[0] + B_1 * last_four[1] + B_2 * last_four[2] + B_3 * last_four[3]));

    //Gets the last four elements of the velocity vector and computes the new postions with the formula
    let last_four_velo: Vec<_> = velo.iter().rev().take(4).collect::<Vec<_>>().iter().rev().map(|v| **v).collect();
    let x_0 = pos.last().unwrap();
    pos.push(x_0 + dt * (B_0 * last_four_velo[0] + B_1 * last_four_velo[1] + B_2 * last_four_velo[2] + B_3 * last_four_velo[3]));
}

fn main() {
    //Vectors for storage
    let mut accel: Vec<f64> = Vec::new();
    let mut velo: Vec<f64> = Vec::new();
    let mut pos: Vec<f64> = Vec::new();
    let mut curr_time: f64 = 0.0;

    //Reserves size for all steps in array
    let num_steps: usize = (END_TIME/dt).ceil() as usize; 
    accel.reserve(num_steps);
    velo.reserve(num_steps);
    pos.reserve(num_steps);

    //Applying initial Conditions
    velo.push(INITIAL_VELOCITY);
    pos.push(INITIAL_POSITION);    

    while curr_time < END_TIME{
        //Takes the last position if it exists and uses Hooke's law to add the spring force to the total force
        let last_pos: Option<&f64> = pos.last();
        let mut tot_force = 0.0;
        match last_pos{
            Some(x) => tot_force = tot_force + -K * x,
            None => tot_force = tot_force,
        }

        //Takes the last velocity if it exists and uses the defined damping formula to add the damping force
        let last_vel: Option<&f64> = velo.last();
        match last_vel{
            Some(v) => tot_force = tot_force + damping(v),
            None => tot_force = tot_force,
        }

        //updates acceleration vector
        let acc: f64 = tot_force/MASS;
        accel.push(acc);
        
        if accel.len() < 4{
            euler_method(&mut pos, &mut velo, &accel);
        }
        else{
            ab_method(&mut pos, &mut velo, &accel);
        }

        curr_time = curr_time + dt;
    }

    //Creates the output xlsx file
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    for i in 0..(pos.len() - 1){
        worksheet.write((i + 1) as u32, 0, pos[i]).unwrap();
        worksheet.write((i + 1) as u32, 1, velo[i]).unwrap();
        worksheet.write((i + 1) as u32, 2, accel[i]).unwrap();
    }

    workbook.save("kinematics_output.xlsx").unwrap();
}
