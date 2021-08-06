use chrono::Utc;
use crossterm::{
    event::{ self, Event as CtEvent },
};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{ Duration, Instant };

use crate::constants;
use crate::data::task as model;
use crate::types;
use crate::ui;

pub fn presenter(arg: &str) {
    if arg.eq("add") {
      add_task();
    }
    else {
      start_main_application();
    }
}

pub fn add_task() {
    let mut name = String::new();
    let mut description = String::new();
    println!("Task Name");
    io::stdin().read_line(&mut name).expect("failed to read_line");
    println!("Task Description");
    io::stdin().read_line(&mut description).expect("failed to read_line");
    add_task_to_db(name, description).expect("could not add task to database");
}

pub fn start_main_application() {
    let (sender, receiver) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("polling") {
                if let CtEvent::Key(key) = event::read().expect("reading CtEvent") {
                    sender.send(types::Event::Press(key)).expect("sending Event::Press");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = sender.send(types::Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    ui::main_ui::render_main_ui(receiver).expect("render main ui");
}

pub fn add_task_to_db(name: String, description: String) -> Result<Vec<model::Task>, types::Error> {
    let db_content = fs::read_to_string(constants::DB_TEST_PATH)?;
    let mut parsed: Vec<model::Task> = serde_json::from_str(&db_content)?;
    let new_task = model::Task {
        name: name,
        description: description,
        date_added: Utc::now(),
    };
    parsed.push(new_task);
    fs::write(constants::DB_TEST_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

pub fn read_test_db() -> Result<Vec<model::Task>, types::Error> {
    let db_content = fs::read_to_string(constants::DB_TEST_PATH)?;
    let parsed: Vec<model::Task> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}
