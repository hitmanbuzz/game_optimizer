use std::collections::HashSet;
use std::io::{self, stdout, Result, Write};
use std::{
    fs::{self},
    process::exit,
};
use sysinfo::{Process, System};

/// Removing Duplicates from Vec<String> types
fn remove_duplicates(vec: &mut Vec<String>) {
    let set: HashSet<_> = vec.drain(..).collect();
    vec.extend(set.into_iter());
}

#[allow(dead_code)]
/// Display all the current running processess
fn display_processes(sys: &System) -> Vec<String> {
    let mut all_processess = Vec::new();

    for (_, process) in sys.processes() {
        if let Some(process_name) = process.name().to_str() {
            all_processess.push(process_name.to_string()); // Converting &str to String
        }
    }
    remove_duplicates(&mut all_processess);
    return all_processess;
}

#[allow(dead_code)]
/// Get all the warn targets before the program is ran
///
/// Implementing soon...
fn warning_display(target_app: &str) -> (Vec<&str>, bool) {
    let target_file = "warning.txt";
    let read_targets = fs::read_to_string(target_file).unwrap();
    let mut warn_targets = Vec::new();
    let mut warn_target_found = false;

    for targets in read_targets.lines() {
        if target_app == targets {
            warn_targets.push(target_app);
            warn_target_found = true;
        }
    }
    return (warn_targets, warn_target_found);
}

/// Kill the process that should be killed
fn kill_process(
    process: &Process,
    target: &str,
    counter: &mut u32,
    min_size: &mut u32,
    delete_memory: &mut u64,
    dead_counter: &mut Option<u32>,
    alive_counter: &mut Option<u32>,
) {
    if process.name().to_str().unwrap() == target {
        if counter <= min_size {
            println!("Minimum Size is less than counter!!!");
            exit(0); // Exiting the program
        }
        if process.kill() {
            println!(
                "Process Kill: {:?} | Memory Used: {:?} MB",
                process.name(),
                process.memory() / 1048576
            );
            *counter -= 1;
            *delete_memory += process.memory() / 1048576; // Converting from bytes for megabytes

            // Increase this counter for those processess which got killed
            if let Some(ref mut dc) = *dead_counter {
                *dc += 1;
            } else {
                *dead_counter = Some(0);
            }
            // *dead_counter = Some(dead_counter.unwrap_or(0) + 1);
        } else {
            // println!("Process {:?} cannot be kill!!!", process.name());

            // Increase this counter for those processess which didn't get killed
            if let Some(ref mut ac) = *alive_counter {
                *ac += 1;
            } else {
                *alive_counter = Some(0);
            }
            // *alive_counter = Some(alive_counter.unwrap_or(0) + 1);
        }
    }
}

/// Use for optimizing games (you can also use it for non-games app)
fn game_optimize(game_file: &str) -> u64 {
    let read_targets = fs::read_to_string(game_file).unwrap();
    let mut delete_memory = 0;
    let mut min_size = 0; // Just so as to quit the whole program
    let mut counter = 0;
    let sys = System::new_all();

    for (_, process) in sys.processes() {
        for game in read_targets.lines() {
            let running_processess = display_processes(&sys);
            for executable_process in running_processess {
                // cmd & powershell will be the shell so it cannot be killed
                if executable_process.contains(".exe")
                    && executable_process != game
                    && executable_process != "powershell.exe"
                    && executable_process != "cmd.exe"
                {
                    // Optimization will be done here
                    counter += 1;
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    sys.refresh_memory();
                    let system_remaining_memory = sys.free_memory() / 1048576; // In Megabytes
                    let system_total_memory = sys.total_memory() / 1048576; // In Megabytes
                                                                            // println!("Remaining System Memory: {}", system_remaining_memory);
                    if system_remaining_memory >= system_total_memory / 2 {
                        // Real Optimiztion done here

                        kill_process(
                            process,
                            &executable_process,
                            &mut counter,
                            &mut min_size,
                            &mut delete_memory,
                            &mut None,
                            &mut None,
                        );
                    } else {
                        println!("Remaining System Memory is not enough for the game to optimize");
                        println!("Decrease the memory requirement for the optimization to work");
                        exit(0);
                    }
                }
            }
        }
    }
    return delete_memory;
}

/// This is the function where everthing is used (For example: kill_process() function is used here)
fn optimize_procesess(target_app: &str) -> Result<(u64, u32, u32)> {
    let mut sys = System::new_all();
    let mut counter: u32 = 0; // counter -> Check of number of processess for the app found
    let mut min_size = 20; // Minimum App Processes Allowed Until It Kills
    sys.refresh_all();

    let target_split: Vec<&str> = target_app.split('|').collect();
    let target = target_split[0];
    let target_index: i32 = target_split[1].parse().unwrap();

    let mut delete_memory = 0;
    let mut dead_counter: Option<u32> = Some(0);
    let mut alive_counter: Option<u32> = Some(0);

    for (_, process) in sys.processes() {
        if process.name().to_str().unwrap() == target {
            // Increase this counter for the processess found from the target.txt file
            counter += 1;
        }
    }

    // Checking if target value number is 0 | 1, if 1 then change the min_size value to 0
    if target_index == 1 {
        min_size = 0;
    }

    if counter > min_size {
        for (_, process) in sys.processes() {
            let (warn_app, warn_found) = warning_display(process.name().to_str().unwrap());
            if warn_found {
                println!("Warning Processess Found!!!");
                println!("App: {:?}\n", warn_app);
                let mut str = String::new();
                print!("Are you sure you want to continue (y/n): ");
                stdout().flush().unwrap();
                io::stdin().read_line(&mut str).unwrap();
                let str = str.trim();
                if str == "n" {
                    println!("Program Exited");
                    (exit)(0);
                }
            }
            kill_process(
                process,
                target,
                &mut counter,
                &mut min_size,
                &mut delete_memory,
                &mut dead_counter,
                &mut alive_counter,
            );
        }
    }
    Ok((delete_memory, dead_counter.unwrap(), alive_counter.unwrap()))
}

fn main() {
    // Target file
    let target_file = "target.txt";
    let read_targets = fs::read_to_string(target_file).unwrap();

    // Game file
    let game_file = "game.txt";

    // Storing Counter & Memory Stuff
    let mut delete_memory = 0;
    let mut dead_counter = 0;
    let mut alive_counter = 0;

    // Looping through each targets from the target.txt
    for target in read_targets.lines() {
        let (_delete_memory, _dead_counter, _alive_counter) = optimize_procesess(target).unwrap();
        let delete_memory_ = game_optimize(game_file);
        delete_memory += _delete_memory + delete_memory_;
        dead_counter += _dead_counter;
        alive_counter += _alive_counter;
    }

    println!("Deleted Memory: {} MB", delete_memory);
    println!("Dead Processess Counter: {}", dead_counter);
    println!("Alive Processess Counter: {}", alive_counter);

    //display_processes();
}
