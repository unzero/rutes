use std::io::stdin;

use crate::core::task::execute;
use crate::core::task::Task;
use crate::core::user::User;


fn exec( active_user: & User, open_processes : & mut Vec<Task>, args : Vec<&str> ) {
    let child = execute(active_user, args[0], args[1..].to_vec());
    match child {
        Ok(new_child) => {
            println!("Appending new child to list");
            open_processes.push(new_child);
        },
        _ => println!("Unexpected error")
    }
}

fn show(open_processes: &mut Vec<Task> ){
    if open_processes.len() > 0 {
        println!("This are the running processes.");
        for p in open_processes.iter_mut() {
            println!("{}", p.to_string());
        }
    }else{
        println!("No running processes.")
    }
}

fn kill( index: usize, open_processes: &mut Vec<Task> ) {
    if index >= open_processes.len() {
        println!("Invalid index");
        return
    } 
    let mut process = open_processes.remove(index);
    let _ = process.kill();
}

fn view( index: usize, open_processes : &mut Vec<Task> ) {
    if index >= open_processes.len() {
        println!("Invalid index");
        return
    }
    let uuid = open_processes[index].get_uuid().unwrap();
    println!(
        "This are the last 10 lines for process with id {} and uuid     {}", 
        index, 
        uuid,
    );
    let _ = open_processes[index].view_tail();
    ()
}


pub fn run(){
    let mut open_processes : Vec<Task> = vec![];
    let user = User::new(String::from("tsukiko")).unwrap();

    loop {
        println!("Please enter a command:");
        let mut buffer = String::new();
        let _ = stdin().read_line(&mut buffer);
        let cmd : Vec<&str> = buffer.trim().split(" ").collect();
        println!("args: {:?}", cmd);
        match cmd[0] {
            "exec" => {
                exec(&user, &mut open_processes, cmd[1..].to_vec())
            }, 
            "show" => {
                show(&mut open_processes);
            },
            "kill" => {
                match cmd[1].parse::<usize>() {
                    Ok(index) => kill(index, &mut open_processes),
                    _ => println!("Invalid index"), 
                }
            }
            "view" => {
                match cmd[1].parse::<usize>() {
                    Ok(index) => view(index, &mut open_processes),
                    _ => println!("Invalid index"),
                }
            },
            _ => {
                println!("not a valid operation!")
            }
        }
    }
}