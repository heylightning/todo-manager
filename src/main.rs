
use std::io::{self, Write, BufRead};

struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input {
            "add" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();

                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                let description = description.trim().to_string();

                let task = Task {
                    description,
                    completed: false,
                };

                tasks.push(task);
                println!("Task added!");
            }
            "complete" => {
                print!("Enter the index of the task to complete: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let index = input.trim().parse::<usize>().unwrap();

                if index < tasks.len() {
                    tasks[index].completed = true;
                    println!("Task marked as completed!");
                } else {
                    println!("Invalid index. No task founf at index {}", index);
                }
            }
            "list" => {
                println!("Todo List: ");
                for (index, task) in tasks.iter().enumerate() {
                    let status = if task.completed { "[x]" } else { "[ ]" };
                    println!("{} {} - {}", index, status, task.description);
                }
            }
            "remove" => {
                print!("Enter the index of the task to remove: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let index = input.trim().parse::<usize>().unwrap();

                if index < tasks.len() {
                    tasks.remove(index);
                    println!("Task removed!");
                } else {
                    println!("Invalid index. No task found at index {}.", index);
                }
            }
            "quit" => {
                println!("Goodbye!");
                break;
            }
            "save" => {
                print!("Enter filename to save tasks: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let filename = input.trim();

                match save_tasks(&tasks, filename) {
                    Ok(()) => println!("Tasks saved successfully!"),
                    Err(err) => eprintln!("Error saving tasks: {}", err),
                }
            }
            "load" => {
                print!("Enter filename to load tasks: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let filename = input.trim();

                match load_tasks(filename) {
                    Ok(loaded_tasks) => {
                        tasks = loaded_tasks;
                        println!("Tasks loaded successfully!");
                    }
                    Err(err) => eprintln!("Error loading tasks: {}", err),
                }
            }
            _ => {
                println!("Invalid command. Try again. Available commands: add, complete, list, remove, save, load, quit");
            }
        }
    }
}

fn save_tasks(tasks: &[Task], filename: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(filename)?;
    for task in tasks {
        writeln!(file, "{};{}", task.description, task.completed)?;
    }
    Ok(())
}

fn load_tasks(filename: &str) -> std::io::Result<Vec<Task>> {
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let mut tasks = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() == 2 {
            let description = parts[0].to_string();
            let completed = parts[1].parse::<bool>().unwrap();
            let task = Task { description, completed };
            tasks.push(task);
        }
    }

    Ok(tasks)
}