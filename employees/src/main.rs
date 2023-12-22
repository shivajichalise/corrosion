use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Write},
    ops::Index,
    process::exit,
};

fn take_user_input(shell: &mut String) {
    io::stdout().flush().unwrap();

    if io::stdin()
        .read_line(shell)
        .expect("Failed to read the input.")
        == 0
    {
        println!();
        println!("EOF detected. Exiting....");
        exit(0);
    }
}

fn check_string_validity(string: &String) -> bool {
    let pattern = Regex::new(r"^Add (\w+) to (\w+)$").unwrap();
    let Some(_) = pattern.captures(string) else {
        // not valid
        return false;
    };

    true
}

fn check_if_quit_command(string: &String) -> bool {
    match string.trim().parse::<char>() {
        Ok('q') => return true,
        Ok(_) => return false,
        Err(_) => false,
    }
}

// fn add_name_to_hashmap(list: &mut HashMap<String, Vec<String>>, string: &String) {
//     let pattern = Regex::new(r"^Add (?<department>\w+) to (?<employee>\w+)").unwrap();
//
//     let Some(captured) = pattern.captures(string) else {
//         println!("Invalid command");
//         return;
//     };
//
//     let mut employees = Vec::new();
//     employees.push(captured["employee"].to_string());
//
//     list.insert(captured["department"].to_string(), employees);
//
//     println!("{:?}", list);
// }

fn add_name_to_hashmap<'a>(
    list: &'a mut HashMap<String, Vec<String>>,
    string: &'a String,
) -> &'a mut HashMap<String, Vec<String>> {
    let pattern = Regex::new(r"^Add (?<employee>\w+) to (?<department>\w+)").unwrap();

    if let Some(captured) = pattern.captures(string) {
        let employees = list.get_mut(&captured["department"].to_string());
        match employees {
            Some(employees) => {
                employees.push(captured["employee"].to_string());
            }
            None => {
                let mut employee = Vec::new();
                employee.push(captured["employee"].to_string());
                list.insert(captured["department"].to_string(), employee);
            }
        }
    } else {
        println!();
        println!("--------------------------");
        println!("invalid input command!");
        println!("--------------------------");
        println!();
    };

    dbg!(&list);
    list
}

fn take_choice_input() -> u32 {
    let mut choice = String::new();

    loop {
        choice.clear();

        println!("1. List Departments.");
        println!("2. List Employees.");
        print!("Choose either 1 or 2: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice.");

        match choice.trim().parse::<u32>() {
            Ok(1) => return 1,
            Ok(2) => return 2,
            Ok(_) => {
                println!("Please give either 1 or 2");
            }
            Err(_) => {
                println!("Please give either 1 or 2");
            }
        }
    }
}

fn take_department_input() -> String {
    let mut department = String::new();

    department.clear();

    print!("Enter the name of the desired department (case sensitive!!): ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read choice.");

    let department = department.trim().to_string();
    return department;
}

fn list_keys(list: &HashMap<String, Vec<String>>) {
    for (mut i, (key, _)) in list.iter().enumerate() {
        i += 1;
        println!("{}. {}", i, key);
    }
}

fn print_hashmap(list: &HashMap<String, Vec<String>>) {
    println!();
    println!("-------------------------------------------------------");
    for (mut i, (k, v)) in list.iter().enumerate() {
        i += 1;
        println!("{}. {} -> {:?}", i, k, v);
    }
    println!("-------------------------------------------------------");
    println!();
}

fn main() {
    println!("-------------------------------------------------------");
    println!("Input formart: 'Add <employee_name> to <department_name>'");
    println!("eg. Add Sally to Engineering or Add Amir to Sales");
    println!("-------------------------------------------------------");
    println!();

    let mut input = String::new();
    let mut list = HashMap::new();

    loop {
        input.clear();

        print!("Please enter your employee (enter q to exit): ");
        take_user_input(&mut input);

        if check_if_quit_command(&input) {
            break;
        }

        if check_string_validity(&input) {
            break;
        }

        add_name_to_hashmap(&mut list, &input);
    }

    print_hashmap(&list);

    let choice = take_choice_input();
    match choice {
        1 => list_keys(&list),
        2 => {
            list_keys(&list);
            let department = take_department_input();

            if let Some(employees) = list.get(&department) {
                let mut to_sort = employees.clone();
                to_sort.sort();

                println!("Emmployees of department {} are : ", department);
                for e in to_sort.into_iter() {
                    println!("{}", e);
                }
            } else {
                println!("No employees");
            };
        }
        _ => {
            println!("done");
        }
    }
}
