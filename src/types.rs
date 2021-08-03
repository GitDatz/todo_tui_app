use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

pub enum Event<I> {
    Press(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
pub enum Page {
    Home,
    Tasks,
}

impl From<Page> for usize {
    fn from(input: Page) -> usize {
        match input {
            Page::Home => 0,
            Page::Tasks => 2,
        }
    }
}
