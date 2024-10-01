use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub category: Option<String>,
    pub priority: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

use std::fs::File;
use std::io::{self, BufReader};

pub fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let file = File::create("tasks.json")?;
    serde_json::to_writer_pretty(file, &tasks)?;
    Ok(())
}

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let file = File::open("tasks.json");
    
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let tasks: Vec<Task> = serde_json::from_reader(reader)?;
            Ok(tasks)
        }
        Err(_) => Ok(Vec::new()),
    }
}
