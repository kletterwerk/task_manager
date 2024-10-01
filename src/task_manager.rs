use chrono::Utc;
use crate::task::{Task, load_tasks, save_tasks};  

pub fn add_task(description: &str, category: &Option<String>, priority: &Option<String>) {
    let mut tasks = load_tasks().expect("failed to load task");

    let new_task = Task {
        id: if tasks.is_empty() { 1 } else { tasks.last().unwrap().id + 1 },
        description: description.to_string(),
        category: category.clone(),
        priority: priority.clone(),
        completed: false,
        created_at: Utc::now(),
    };

    tasks.push(new_task);
    save_tasks(&tasks).expect("failed to save task");

    println!("task added successfully.");
}

pub fn list_tasks(category_filter: &Option<String>) {
    let tasks = load_tasks().expect("failed to load tasks");

    let filtered_tasks: Vec<&Task> = if let Some(category) = category_filter {
        tasks.iter().filter(|task| task.category.as_deref() == Some(category.as_str())).collect()
    } else {
        tasks.iter().collect()
    };

    if filtered_tasks.is_empty() {
        println!("no tasks to display.");
    } else {
        for task in filtered_tasks {
            println!(
                "[{}] {} (Category: {}, Priority: {}, Status: {})",
                task.id,
                task.description,
                task.category.as_deref().unwrap_or("None"),
                task.priority.as_deref().unwrap_or("None"),
                if task.completed { "Completed" } else { "In Progress"
                }
            );
        }
    }
}

pub fn complete_task(id: u32) {
    let mut tasks = load_tasks().expect("failed to load tasks");

    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.completed = true;
        save_tasks(&tasks).expect("failed to save tasks");
        println!("Task with ID {} completed.", id);
    } else {
        println!("Task with ID {} not found.", id);
    }
}

pub fn delete_task(id: u32) {
    let mut tasks = load_tasks().expect("failed to load tasks");

    let initial_len = tasks.len();
    tasks.retain(|task| task.id != id);

    if tasks.len() == initial_len {
        println!("Task with ID {} not found.", id);
    } else {
        save_tasks(&tasks).expect("failed to save tasks");
        println!("Task with ID {} deleted.", id);
    }
}
