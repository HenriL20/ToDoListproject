use std::collections::VecDeque;
use std::io::{self, Write};

fn main() {
    let mut todo_list: VecDeque<String> = VecDeque::new();

    loop {
        println!("\n=== Todo List ===");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Remove Task");
        println!("4. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => add_task(&mut todo_list),
            "2" => view_tasks(&todo_list),
            "3" => remove_task(&mut todo_list),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn add_task(todo_list: &mut VecDeque<String>) {
    print!("Enter the task: ");
    io::stdout().flush().unwrap();

    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();
    let task = task.trim();

    if !task.is_empty() {
        todo_list.push_back(task.to_string());
        println!("Task added: \"{}\"", task);
    } else {
        println!("Task cannot be empty!");
    }
}

fn view_tasks(todo_list: &VecDeque<String>) {
    if todo_list.is_empty() {
        println!("No tasks in the list!");
    } else {
        println!("\nYour Tasks:");
        for (i, task) in todo_list.iter().enumerate() {
            println!("{}. {}", i + 1, task);
        }
    }
}

fn remove_task(todo_list: &mut VecDeque<String>) {
    if todo_list.is_empty() {
        println!("No tasks to remove!");
        return;
    }

    view_tasks(todo_list);

    print!("Enter the task number to remove: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= todo_list.len() => {
            let removed_task = todo_list.remove(index - 1).unwrap();
            println!("Removed task: \"{}\"", removed_task);
        }
        _ => println!("Invalid task number!"),
    }
}
