use crossterm::{
    event::{ self, Event as CtEvent },
};
use std::fs;
use std::sync::mpsc;
use std::thread;
use std::time::{ Duration, Instant };

use crate::constants;
use crate::data::task as model;
use crate::types;
use crate::ui;

pub fn presenter() {
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

pub fn read_test_db() -> Result<Vec<model::Task>, types::Error> {
    let db_content = fs::read_to_string(constants::DB_TEST_PATH)?;
    let parsed: Vec<model::Task> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}
