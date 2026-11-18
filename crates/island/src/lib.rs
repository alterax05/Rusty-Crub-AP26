#![deny(unsafe_code)]
#![deny(clippy::pedantic)]

use protocol::IslandInit;

pub type IslandFactory = fn(IslandInit) -> IslandType;

#[derive(Debug)]
pub enum IslandType {
    Generous(Generous),
}

#[derive(Debug)]
pub struct Generous;
