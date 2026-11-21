#![deny(unsafe_code)]
#![allow(dead_code, unused_variables, clippy::missing_errors_doc)]

mod actions;
mod rounds;
mod state;

use std::collections::{BTreeMap, BTreeSet};

use island::{Generous, IslandFactory, IslandType};
use protocol::{
    Action, Axis, Bound, Cell, Chart, Clue, DefaultMap, Event, GenError, IslandAnswer, IslandId, IslandInit, Map,
    Outcome, Params, Rejection, Report, Round, Seed, ShipId, ShipView, SpectatorView, Terrain,
};

pub use actions::ValidatedAction;

use crate::state::Status;
use state::{Ending, IslandSlot, ShipState};

const ISLANDS: &[IslandFactory] = &[|_| IslandType::Generous(Generous)];

/// Generate the map, validate all the actions and contains all the state of the game.
pub struct DefaultChart {
    params: Params,
    map: DefaultMap,
    treasure: Cell,
    weather_seed: Seed,
    round: Round,
    ships: BTreeMap<ShipId, ShipState>,
    islands: BTreeMap<IslandId, IslandSlot>,
    winner: Option<ShipId>,
    ending: Option<Ending>,
}

impl DefaultChart {
    fn terrain_generator() -> Vec<(Cell, Terrain)> {
        vec![
            (Cell { q: -12, r: 0 }, Terrain::Clear),
            (Cell { q: -12, r: 1 }, Terrain::Clear),
            (Cell { q: -12, r: 2 }, Terrain::Clear),
            (Cell { q: -12, r: 3 }, Terrain::Clear),
            (Cell { q: -12, r: 4 }, Terrain::Clear),
            (Cell { q: -12, r: 5 }, Terrain::Clear),
            (Cell { q: -12, r: 6 }, Terrain::Clear),
            (Cell { q: -12, r: 7 }, Terrain::Cliff),
            (Cell { q: -12, r: 8 }, Terrain::Clear),
            (Cell { q: -12, r: 9 }, Terrain::Clear),
            (Cell { q: -12, r: 10 }, Terrain::Clear),
            (Cell { q: -12, r: 11 }, Terrain::Clear),
            (Cell { q: -12, r: 12 }, Terrain::Clear),
            (Cell { q: -11, r: -1 }, Terrain::Clear),
            (Cell { q: -11, r: 0 }, Terrain::Clear),
            (Cell { q: -11, r: 1 }, Terrain::Clear),
            (Cell { q: -11, r: 2 }, Terrain::Clear),
            (Cell { q: -11, r: 3 }, Terrain::Clear),
            (Cell { q: -11, r: 4 }, Terrain::Clear),
            (Cell { q: -11, r: 5 }, Terrain::Clear),
            (Cell { q: -11, r: 6 }, Terrain::Clear),
            (Cell { q: -11, r: 7 }, Terrain::Clear),
            (Cell { q: -11, r: 8 }, Terrain::Clear),
            (Cell { q: -11, r: 9 }, Terrain::Clear),
            (Cell { q: -11, r: 10 }, Terrain::Clear),
            (Cell { q: -11, r: 11 }, Terrain::Clear),
            (Cell { q: -11, r: 12 }, Terrain::Clear),
            (Cell { q: -10, r: -2 }, Terrain::Clear),
            (Cell { q: -10, r: -1 }, Terrain::Clear),
            (Cell { q: -10, r: 0 }, Terrain::Clear),
            (Cell { q: -10, r: 1 }, Terrain::Clear),
            (Cell { q: -10, r: 2 }, Terrain::Clear),
            (Cell { q: -10, r: 3 }, Terrain::Cliff),
            (Cell { q: -10, r: 4 }, Terrain::Cliff),
            (Cell { q: -10, r: 5 }, Terrain::Cliff),
            (Cell { q: -10, r: 6 }, Terrain::Clear),
            (Cell { q: -10, r: 7 }, Terrain::Clear),
            (Cell { q: -10, r: 8 }, Terrain::Clear),
            (Cell { q: -10, r: 9 }, Terrain::Clear),
            (Cell { q: -10, r: 10 }, Terrain::Clear),
            (Cell { q: -10, r: 11 }, Terrain::Clear),
            (Cell { q: -10, r: 12 }, Terrain::Clear),
            (Cell { q: -9, r: -3 }, Terrain::Clear),
            (Cell { q: -9, r: -2 }, Terrain::Clear),
            (Cell { q: -9, r: -1 }, Terrain::Clear),
            (Cell { q: -9, r: 0 }, Terrain::Island(IslandId(6))),
            (Cell { q: -9, r: 1 }, Terrain::Clear),
            (Cell { q: -9, r: 2 }, Terrain::Clear),
            (Cell { q: -9, r: 3 }, Terrain::Clear),
            (Cell { q: -9, r: 4 }, Terrain::Cliff),
            (Cell { q: -9, r: 5 }, Terrain::Clear),
            (Cell { q: -9, r: 6 }, Terrain::Clear),
            (Cell { q: -9, r: 7 }, Terrain::Clear),
            (Cell { q: -9, r: 8 }, Terrain::Clear),
            (Cell { q: -9, r: 9 }, Terrain::Clear),
            (Cell { q: -9, r: 10 }, Terrain::Island(IslandId(3))),
            (Cell { q: -9, r: 11 }, Terrain::Clear),
            (Cell { q: -9, r: 12 }, Terrain::Clear),
            (Cell { q: -8, r: -4 }, Terrain::Clear),
            (Cell { q: -8, r: -3 }, Terrain::Clear),
            (Cell { q: -8, r: -2 }, Terrain::Clear),
            (Cell { q: -8, r: -1 }, Terrain::Clear),
            (Cell { q: -8, r: 0 }, Terrain::Island(IslandId(6))),
            (Cell { q: -8, r: 1 }, Terrain::Island(IslandId(6))),
            (Cell { q: -8, r: 2 }, Terrain::Rock),
            (Cell { q: -8, r: 3 }, Terrain::Clear),
            (Cell { q: -8, r: 4 }, Terrain::Cliff),
            (Cell { q: -8, r: 5 }, Terrain::Clear),
            (Cell { q: -8, r: 6 }, Terrain::Clear),
            (Cell { q: -8, r: 7 }, Terrain::Clear),
            (Cell { q: -8, r: 8 }, Terrain::Clear),
            (Cell { q: -8, r: 9 }, Terrain::Clear),
            (Cell { q: -8, r: 10 }, Terrain::Island(IslandId(3))),
            (Cell { q: -8, r: 11 }, Terrain::Clear),
            (Cell { q: -8, r: 12 }, Terrain::Clear),
            (Cell { q: -7, r: -5 }, Terrain::Clear),
            (Cell { q: -7, r: -4 }, Terrain::Clear),
            (Cell { q: -7, r: -3 }, Terrain::Clear),
            (Cell { q: -7, r: -2 }, Terrain::Clear),
            (Cell { q: -7, r: -1 }, Terrain::Clear),
            (Cell { q: -7, r: 0 }, Terrain::Clear),
            (Cell { q: -7, r: 1 }, Terrain::Clear),
            (Cell { q: -7, r: 2 }, Terrain::Clear),
            (Cell { q: -7, r: 3 }, Terrain::Cliff),
            (Cell { q: -7, r: 4 }, Terrain::Cliff),
            (Cell { q: -7, r: 5 }, Terrain::Clear),
            (Cell { q: -7, r: 6 }, Terrain::Rock),
            (Cell { q: -7, r: 7 }, Terrain::Clear),
            (Cell { q: -7, r: 8 }, Terrain::Clear),
            (Cell { q: -7, r: 9 }, Terrain::Clear),
            (Cell { q: -7, r: 10 }, Terrain::Clear),
            (Cell { q: -7, r: 11 }, Terrain::Clear),
            (Cell { q: -7, r: 12 }, Terrain::Clear),
            (Cell { q: -6, r: -6 }, Terrain::Clear),
            (Cell { q: -6, r: -5 }, Terrain::Clear),
            (Cell { q: -6, r: -4 }, Terrain::Clear),
            (Cell { q: -6, r: -3 }, Terrain::Clear),
            (Cell { q: -6, r: -2 }, Terrain::Clear),
            (Cell { q: -6, r: -1 }, Terrain::Clear),
            (Cell { q: -6, r: 0 }, Terrain::Rock),
            (Cell { q: -6, r: 1 }, Terrain::Clear),
            (Cell { q: -6, r: 2 }, Terrain::Clear),
            (Cell { q: -6, r: 3 }, Terrain::Clear),
            (Cell { q: -6, r: 4 }, Terrain::Clear),
            (Cell { q: -6, r: 5 }, Terrain::Clear),
            (Cell { q: -6, r: 6 }, Terrain::Clear),
            (Cell { q: -6, r: 7 }, Terrain::Clear),
            (Cell { q: -6, r: 8 }, Terrain::Clear),
            (Cell { q: -6, r: 9 }, Terrain::Clear),
            (Cell { q: -6, r: 10 }, Terrain::Clear),
            (Cell { q: -6, r: 11 }, Terrain::Clear),
            (Cell { q: -6, r: 12 }, Terrain::Clear),
            (Cell { q: -5, r: -7 }, Terrain::Clear),
            (Cell { q: -5, r: -6 }, Terrain::Clear),
            (Cell { q: -5, r: -5 }, Terrain::Clear),
            (Cell { q: -5, r: -4 }, Terrain::Clear),
            (Cell { q: -5, r: -3 }, Terrain::Clear),
            (Cell { q: -5, r: -2 }, Terrain::Clear),
            (Cell { q: -5, r: -1 }, Terrain::Clear),
            (Cell { q: -5, r: 0 }, Terrain::Clear),
            (Cell { q: -5, r: 1 }, Terrain::Clear),
            (Cell { q: -5, r: 2 }, Terrain::Clear),
            (Cell { q: -5, r: 3 }, Terrain::Clear),
            (Cell { q: -5, r: 4 }, Terrain::Clear),
            (Cell { q: -5, r: 5 }, Terrain::Clear),
            (Cell { q: -5, r: 6 }, Terrain::Clear),
            (Cell { q: -5, r: 7 }, Terrain::Clear),
            (Cell { q: -5, r: 8 }, Terrain::Clear),
            (Cell { q: -5, r: 9 }, Terrain::Clear),
            (Cell { q: -5, r: 10 }, Terrain::Clear),
            (Cell { q: -5, r: 11 }, Terrain::Clear),
            (Cell { q: -5, r: 12 }, Terrain::Clear),
            (Cell { q: -4, r: -8 }, Terrain::Clear),
            (Cell { q: -4, r: -7 }, Terrain::Clear),
            (Cell { q: -4, r: -6 }, Terrain::Clear),
            (Cell { q: -4, r: -5 }, Terrain::Clear),
            (Cell { q: -4, r: -4 }, Terrain::Clear),
            (Cell { q: -4, r: -3 }, Terrain::Clear),
            (Cell { q: -4, r: -2 }, Terrain::Clear),
            (Cell { q: -4, r: -1 }, Terrain::Clear),
            (Cell { q: -4, r: 0 }, Terrain::Clear),
            (Cell { q: -4, r: 1 }, Terrain::Island(IslandId(1))),
            (Cell { q: -4, r: 2 }, Terrain::Clear),
            (Cell { q: -4, r: 3 }, Terrain::Clear),
            (Cell { q: -4, r: 4 }, Terrain::Clear),
            (Cell { q: -4, r: 5 }, Terrain::Clear),
            (Cell { q: -4, r: 6 }, Terrain::Clear),
            (Cell { q: -4, r: 7 }, Terrain::Rock),
            (Cell { q: -4, r: 8 }, Terrain::Clear),
            (Cell { q: -4, r: 9 }, Terrain::Clear),
            (Cell { q: -4, r: 10 }, Terrain::Clear),
            (Cell { q: -4, r: 11 }, Terrain::Clear),
            (Cell { q: -4, r: 12 }, Terrain::Clear),
            (Cell { q: -3, r: -9 }, Terrain::Clear),
            (Cell { q: -3, r: -8 }, Terrain::Clear),
            (Cell { q: -3, r: -7 }, Terrain::Clear),
            (Cell { q: -3, r: -6 }, Terrain::Clear),
            (Cell { q: -3, r: -5 }, Terrain::Clear),
            (Cell { q: -3, r: -4 }, Terrain::Clear),
            (Cell { q: -3, r: -3 }, Terrain::Clear),
            (Cell { q: -3, r: -2 }, Terrain::Clear),
            (Cell { q: -3, r: -1 }, Terrain::Island(IslandId(1))),
            (Cell { q: -3, r: 0 }, Terrain::Island(IslandId(1))),
            (Cell { q: -3, r: 1 }, Terrain::Clear),
            (Cell { q: -3, r: 2 }, Terrain::Clear),
            (Cell { q: -3, r: 3 }, Terrain::Clear),
            (Cell { q: -3, r: 4 }, Terrain::Cliff),
            (Cell { q: -3, r: 5 }, Terrain::Clear),
            (Cell { q: -3, r: 6 }, Terrain::Clear),
            (Cell { q: -3, r: 7 }, Terrain::Clear),
            (Cell { q: -3, r: 8 }, Terrain::Clear),
            (Cell { q: -3, r: 9 }, Terrain::Clear),
            (Cell { q: -3, r: 10 }, Terrain::Clear),
            (Cell { q: -3, r: 11 }, Terrain::Island(IslandId(7))),
            (Cell { q: -3, r: 12 }, Terrain::Clear),
            (Cell { q: -2, r: -10 }, Terrain::Clear),
            (Cell { q: -2, r: -9 }, Terrain::Clear),
            (Cell { q: -2, r: -8 }, Terrain::Clear),
            (Cell { q: -2, r: -7 }, Terrain::Clear),
            (Cell { q: -2, r: -6 }, Terrain::Clear),
            (Cell { q: -2, r: -5 }, Terrain::Rock),
            (Cell { q: -2, r: -4 }, Terrain::Clear),
            (Cell { q: -2, r: -3 }, Terrain::Clear),
            (Cell { q: -2, r: -2 }, Terrain::Clear),
            (Cell { q: -2, r: -1 }, Terrain::Clear),
            (Cell { q: -2, r: 0 }, Terrain::Clear),
            (Cell { q: -2, r: 1 }, Terrain::Clear),
            (Cell { q: -2, r: 2 }, Terrain::Clear),
            (Cell { q: -2, r: 3 }, Terrain::Cliff),
            (Cell { q: -2, r: 4 }, Terrain::Cliff),
            (Cell { q: -2, r: 5 }, Terrain::Clear),
            (Cell { q: -2, r: 6 }, Terrain::Clear),
            (Cell { q: -2, r: 7 }, Terrain::Clear),
            (Cell { q: -2, r: 8 }, Terrain::Clear),
            (Cell { q: -2, r: 9 }, Terrain::Clear),
            (Cell { q: -2, r: 10 }, Terrain::Clear),
            (Cell { q: -2, r: 11 }, Terrain::Island(IslandId(7))),
            (Cell { q: -2, r: 12 }, Terrain::Clear),
            (Cell { q: -1, r: -11 }, Terrain::Clear),
            (Cell { q: -1, r: -10 }, Terrain::Clear),
            (Cell { q: -1, r: -9 }, Terrain::Clear),
            (Cell { q: -1, r: -8 }, Terrain::Clear),
            (Cell { q: -1, r: -7 }, Terrain::Clear),
            (Cell { q: -1, r: -6 }, Terrain::Clear),
            (Cell { q: -1, r: -5 }, Terrain::Clear),
            (Cell { q: -1, r: -4 }, Terrain::Clear),
            (Cell { q: -1, r: -3 }, Terrain::Clear),
            (Cell { q: -1, r: -2 }, Terrain::Clear),
            (Cell { q: -1, r: -1 }, Terrain::Clear),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: -1, r: 2 }, Terrain::Clear),
            (Cell { q: -1, r: 3 }, Terrain::Clear),
            (Cell { q: -1, r: 4 }, Terrain::Clear),
            (Cell { q: -1, r: 5 }, Terrain::Clear),
            (Cell { q: -1, r: 6 }, Terrain::Clear),
            (Cell { q: -1, r: 7 }, Terrain::Clear),
            (Cell { q: -1, r: 8 }, Terrain::Clear),
            (Cell { q: -1, r: 9 }, Terrain::Clear),
            (Cell { q: -1, r: 10 }, Terrain::Island(IslandId(7))),
            (Cell { q: -1, r: 11 }, Terrain::Clear),
            (Cell { q: -1, r: 12 }, Terrain::Clear),
            (Cell { q: 0, r: -12 }, Terrain::Clear),
            (Cell { q: 0, r: -11 }, Terrain::Clear),
            (Cell { q: 0, r: -10 }, Terrain::Clear),
            (Cell { q: 0, r: -9 }, Terrain::Rock),
            (Cell { q: 0, r: -8 }, Terrain::Clear),
            (Cell { q: 0, r: -7 }, Terrain::Island(IslandId(4))),
            (Cell { q: 0, r: -6 }, Terrain::Clear),
            (Cell { q: 0, r: -5 }, Terrain::Clear),
            (Cell { q: 0, r: -4 }, Terrain::Clear),
            (Cell { q: 0, r: -3 }, Terrain::Clear),
            (Cell { q: 0, r: -2 }, Terrain::Rock),
            (Cell { q: 0, r: -1 }, Terrain::Clear),
            (Cell { q: 0, r: 0 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 2 }, Terrain::Clear),
            (Cell { q: 0, r: 3 }, Terrain::Clear),
            (Cell { q: 0, r: 4 }, Terrain::Clear),
            (Cell { q: 0, r: 5 }, Terrain::Clear),
            (Cell { q: 0, r: 6 }, Terrain::Clear),
            (Cell { q: 0, r: 7 }, Terrain::Clear),
            (Cell { q: 0, r: 8 }, Terrain::Clear),
            (Cell { q: 0, r: 9 }, Terrain::Clear),
            (Cell { q: 0, r: 10 }, Terrain::Clear),
            (Cell { q: 0, r: 11 }, Terrain::Clear),
            (Cell { q: 0, r: 12 }, Terrain::Clear),
            (Cell { q: 1, r: -12 }, Terrain::Clear),
            (Cell { q: 1, r: -11 }, Terrain::Clear),
            (Cell { q: 1, r: -10 }, Terrain::Clear),
            (Cell { q: 1, r: -9 }, Terrain::Clear),
            (Cell { q: 1, r: -8 }, Terrain::Clear),
            (Cell { q: 1, r: -7 }, Terrain::Island(IslandId(4))),
            (Cell { q: 1, r: -6 }, Terrain::Clear),
            (Cell { q: 1, r: -5 }, Terrain::Clear),
            (Cell { q: 1, r: -4 }, Terrain::Clear),
            (Cell { q: 1, r: -3 }, Terrain::Clear),
            (Cell { q: 1, r: -2 }, Terrain::Clear),
            (Cell { q: 1, r: -1 }, Terrain::Clear),
            (Cell { q: 1, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: 1 }, Terrain::Clear),
            (Cell { q: 1, r: 2 }, Terrain::Clear),
            (Cell { q: 1, r: 3 }, Terrain::Clear),
            (Cell { q: 1, r: 4 }, Terrain::Clear),
            (Cell { q: 1, r: 5 }, Terrain::Clear),
            (Cell { q: 1, r: 6 }, Terrain::Clear),
            (Cell { q: 1, r: 7 }, Terrain::Clear),
            (Cell { q: 1, r: 8 }, Terrain::Clear),
            (Cell { q: 1, r: 9 }, Terrain::Clear),
            (Cell { q: 1, r: 10 }, Terrain::Clear),
            (Cell { q: 1, r: 11 }, Terrain::Clear),
            (Cell { q: 2, r: -12 }, Terrain::Clear),
            (Cell { q: 2, r: -11 }, Terrain::Clear),
            (Cell { q: 2, r: -10 }, Terrain::Clear),
            (Cell { q: 2, r: -9 }, Terrain::Clear),
            (Cell { q: 2, r: -8 }, Terrain::Clear),
            (Cell { q: 2, r: -7 }, Terrain::Island(IslandId(4))),
            (Cell { q: 2, r: -6 }, Terrain::Clear),
            (Cell { q: 2, r: -5 }, Terrain::Clear),
            (Cell { q: 2, r: -4 }, Terrain::Clear),
            (Cell { q: 2, r: -3 }, Terrain::Clear),
            (Cell { q: 2, r: -2 }, Terrain::Clear),
            (Cell { q: 2, r: -1 }, Terrain::Clear),
            (Cell { q: 2, r: 0 }, Terrain::Clear),
            (Cell { q: 2, r: 1 }, Terrain::Clear),
            (Cell { q: 2, r: 2 }, Terrain::Clear),
            (Cell { q: 2, r: 3 }, Terrain::Clear),
            (Cell { q: 2, r: 4 }, Terrain::Clear),
            (Cell { q: 2, r: 5 }, Terrain::Clear),
            (Cell { q: 2, r: 6 }, Terrain::Rock),
            (Cell { q: 2, r: 7 }, Terrain::Clear),
            (Cell { q: 2, r: 8 }, Terrain::Rock),
            (Cell { q: 2, r: 9 }, Terrain::Clear),
            (Cell { q: 2, r: 10 }, Terrain::Clear),
            (Cell { q: 3, r: -12 }, Terrain::Clear),
            (Cell { q: 3, r: -11 }, Terrain::Clear),
            (Cell { q: 3, r: -10 }, Terrain::Clear),
            (Cell { q: 3, r: -9 }, Terrain::Clear),
            (Cell { q: 3, r: -8 }, Terrain::Clear),
            (Cell { q: 3, r: -7 }, Terrain::Clear),
            (Cell { q: 3, r: -6 }, Terrain::Clear),
            (Cell { q: 3, r: -5 }, Terrain::Clear),
            (Cell { q: 3, r: -4 }, Terrain::Clear),
            (Cell { q: 3, r: -3 }, Terrain::Clear),
            (Cell { q: 3, r: -2 }, Terrain::Clear),
            (Cell { q: 3, r: -1 }, Terrain::Clear),
            (Cell { q: 3, r: 0 }, Terrain::Clear),
            (Cell { q: 3, r: 1 }, Terrain::Clear),
            (Cell { q: 3, r: 2 }, Terrain::Clear),
            (Cell { q: 3, r: 3 }, Terrain::Clear),
            (Cell { q: 3, r: 4 }, Terrain::Clear),
            (Cell { q: 3, r: 5 }, Terrain::Clear),
            (Cell { q: 3, r: 6 }, Terrain::Clear),
            (Cell { q: 3, r: 7 }, Terrain::Clear),
            (Cell { q: 3, r: 8 }, Terrain::Clear),
            (Cell { q: 3, r: 9 }, Terrain::Clear),
            (Cell { q: 4, r: -12 }, Terrain::Clear),
            (Cell { q: 4, r: -11 }, Terrain::Cliff),
            (Cell { q: 4, r: -10 }, Terrain::Clear),
            (Cell { q: 4, r: -9 }, Terrain::Clear),
            (Cell { q: 4, r: -8 }, Terrain::Clear),
            (Cell { q: 4, r: -7 }, Terrain::Clear),
            (Cell { q: 4, r: -6 }, Terrain::Clear),
            (Cell { q: 4, r: -5 }, Terrain::Clear),
            (Cell { q: 4, r: -4 }, Terrain::Clear),
            (Cell { q: 4, r: -3 }, Terrain::Clear),
            (Cell { q: 4, r: -2 }, Terrain::Clear),
            (Cell { q: 4, r: -1 }, Terrain::Clear),
            (Cell { q: 4, r: 0 }, Terrain::Clear),
            (Cell { q: 4, r: 1 }, Terrain::Rock),
            (Cell { q: 4, r: 2 }, Terrain::Clear),
            (Cell { q: 4, r: 3 }, Terrain::Cliff),
            (Cell { q: 4, r: 4 }, Terrain::Clear),
            (Cell { q: 4, r: 5 }, Terrain::Clear),
            (Cell { q: 4, r: 6 }, Terrain::Clear),
            (Cell { q: 4, r: 7 }, Terrain::Clear),
            (Cell { q: 4, r: 8 }, Terrain::Clear),
            (Cell { q: 5, r: -12 }, Terrain::Clear),
            (Cell { q: 5, r: -11 }, Terrain::Cliff),
            (Cell { q: 5, r: -10 }, Terrain::Clear),
            (Cell { q: 5, r: -9 }, Terrain::Clear),
            (Cell { q: 5, r: -8 }, Terrain::Clear),
            (Cell { q: 5, r: -7 }, Terrain::Clear),
            (Cell { q: 5, r: -6 }, Terrain::Clear),
            (Cell { q: 5, r: -5 }, Terrain::Clear),
            (Cell { q: 5, r: -4 }, Terrain::Clear),
            (Cell { q: 5, r: -3 }, Terrain::Clear),
            (Cell { q: 5, r: -2 }, Terrain::Clear),
            (Cell { q: 5, r: -1 }, Terrain::Clear),
            (Cell { q: 5, r: 0 }, Terrain::Clear),
            (Cell { q: 5, r: 1 }, Terrain::Clear),
            (Cell { q: 5, r: 2 }, Terrain::Cliff),
            (Cell { q: 5, r: 3 }, Terrain::Clear),
            (Cell { q: 5, r: 4 }, Terrain::Clear),
            (Cell { q: 5, r: 5 }, Terrain::Clear),
            (Cell { q: 5, r: 6 }, Terrain::Clear),
            (Cell { q: 5, r: 7 }, Terrain::Clear),
            (Cell { q: 6, r: -12 }, Terrain::Clear),
            (Cell { q: 6, r: -11 }, Terrain::Cliff),
            (Cell { q: 6, r: -10 }, Terrain::Clear),
            (Cell { q: 6, r: -9 }, Terrain::Cliff),
            (Cell { q: 6, r: -8 }, Terrain::Cliff),
            (Cell { q: 6, r: -7 }, Terrain::Cliff),
            (Cell { q: 6, r: -6 }, Terrain::Cliff),
            (Cell { q: 6, r: -5 }, Terrain::Cliff),
            (Cell { q: 6, r: -4 }, Terrain::Clear),
            (Cell { q: 6, r: -3 }, Terrain::Clear),
            (Cell { q: 6, r: -2 }, Terrain::Clear),
            (Cell { q: 6, r: -1 }, Terrain::Island(IslandId(5))),
            (Cell { q: 6, r: 0 }, Terrain::Clear),
            (Cell { q: 6, r: 1 }, Terrain::Cliff),
            (Cell { q: 6, r: 2 }, Terrain::Cliff),
            (Cell { q: 6, r: 3 }, Terrain::Clear),
            (Cell { q: 6, r: 4 }, Terrain::Island(IslandId(2))),
            (Cell { q: 6, r: 5 }, Terrain::Clear),
            (Cell { q: 6, r: 6 }, Terrain::Clear),
            (Cell { q: 7, r: -12 }, Terrain::Cliff),
            (Cell { q: 7, r: -11 }, Terrain::Clear),
            (Cell { q: 7, r: -10 }, Terrain::Rock),
            (Cell { q: 7, r: -9 }, Terrain::Clear),
            (Cell { q: 7, r: -8 }, Terrain::Clear),
            (Cell { q: 7, r: -7 }, Terrain::Clear),
            (Cell { q: 7, r: -6 }, Terrain::Clear),
            (Cell { q: 7, r: -5 }, Terrain::Clear),
            (Cell { q: 7, r: -4 }, Terrain::Clear),
            (Cell { q: 7, r: -3 }, Terrain::Clear),
            (Cell { q: 7, r: -2 }, Terrain::Island(IslandId(5))),
            (Cell { q: 7, r: -1 }, Terrain::Clear),
            (Cell { q: 7, r: 0 }, Terrain::Cliff),
            (Cell { q: 7, r: 1 }, Terrain::Cliff),
            (Cell { q: 7, r: 2 }, Terrain::Clear),
            (Cell { q: 7, r: 3 }, Terrain::Island(IslandId(2))),
            (Cell { q: 7, r: 4 }, Terrain::Clear),
            (Cell { q: 7, r: 5 }, Terrain::Clear),
            (Cell { q: 8, r: -12 }, Terrain::Clear),
            (Cell { q: 8, r: -11 }, Terrain::Clear),
            (Cell { q: 8, r: -10 }, Terrain::Clear),
            (Cell { q: 8, r: -9 }, Terrain::Clear),
            (Cell { q: 8, r: -8 }, Terrain::Clear),
            (Cell { q: 8, r: -7 }, Terrain::Clear),
            (Cell { q: 8, r: -6 }, Terrain::Clear),
            (Cell { q: 8, r: -5 }, Terrain::Clear),
            (Cell { q: 8, r: -4 }, Terrain::Clear),
            (Cell { q: 8, r: -3 }, Terrain::Clear),
            (Cell { q: 8, r: -2 }, Terrain::Clear),
            (Cell { q: 8, r: -1 }, Terrain::Clear),
            (Cell { q: 8, r: 0 }, Terrain::Clear),
            (Cell { q: 8, r: 1 }, Terrain::Clear),
            (Cell { q: 8, r: 2 }, Terrain::Clear),
            (Cell { q: 8, r: 3 }, Terrain::Clear),
            (Cell { q: 8, r: 4 }, Terrain::Clear),
            (Cell { q: 9, r: -12 }, Terrain::Clear),
            (Cell { q: 9, r: -11 }, Terrain::Island(IslandId(0))),
            (Cell { q: 9, r: -10 }, Terrain::Island(IslandId(0))),
            (Cell { q: 9, r: -9 }, Terrain::Clear),
            (Cell { q: 9, r: -8 }, Terrain::Clear),
            (Cell { q: 9, r: -7 }, Terrain::Clear),
            (Cell { q: 9, r: -6 }, Terrain::Clear),
            (Cell { q: 9, r: -5 }, Terrain::Clear),
            (Cell { q: 9, r: -4 }, Terrain::Clear),
            (Cell { q: 9, r: -3 }, Terrain::Rock),
            (Cell { q: 9, r: -2 }, Terrain::Clear),
            (Cell { q: 9, r: -1 }, Terrain::Clear),
            (Cell { q: 9, r: 0 }, Terrain::Clear),
            (Cell { q: 9, r: 1 }, Terrain::Clear),
            (Cell { q: 9, r: 2 }, Terrain::Clear),
            (Cell { q: 9, r: 3 }, Terrain::Clear),
            (Cell { q: 10, r: -12 }, Terrain::Clear),
            (Cell { q: 10, r: -11 }, Terrain::Clear),
            (Cell { q: 10, r: -10 }, Terrain::Island(IslandId(0))),
            (Cell { q: 10, r: -9 }, Terrain::Clear),
            (Cell { q: 10, r: -8 }, Terrain::Clear),
            (Cell { q: 10, r: -7 }, Terrain::Clear),
            (Cell { q: 10, r: -6 }, Terrain::Clear),
            (Cell { q: 10, r: -5 }, Terrain::Clear),
            (Cell { q: 10, r: -4 }, Terrain::Clear),
            (Cell { q: 10, r: -3 }, Terrain::Clear),
            (Cell { q: 10, r: -2 }, Terrain::Clear),
            (Cell { q: 10, r: -1 }, Terrain::Clear),
            (Cell { q: 10, r: 0 }, Terrain::Clear),
            (Cell { q: 10, r: 1 }, Terrain::Clear),
            (Cell { q: 10, r: 2 }, Terrain::Clear),
            (Cell { q: 11, r: -12 }, Terrain::Clear),
            (Cell { q: 11, r: -11 }, Terrain::Clear),
            (Cell { q: 11, r: -10 }, Terrain::Clear),
            (Cell { q: 11, r: -9 }, Terrain::Clear),
            (Cell { q: 11, r: -8 }, Terrain::Clear),
            (Cell { q: 11, r: -7 }, Terrain::Clear),
            (Cell { q: 11, r: -6 }, Terrain::Clear),
            (Cell { q: 11, r: -5 }, Terrain::Clear),
            (Cell { q: 11, r: -4 }, Terrain::Clear),
            (Cell { q: 11, r: -3 }, Terrain::Clear),
            (Cell { q: 11, r: -2 }, Terrain::Clear),
            (Cell { q: 11, r: -1 }, Terrain::Clear),
            (Cell { q: 11, r: 0 }, Terrain::Clear),
            (Cell { q: 11, r: 1 }, Terrain::Clear),
            (Cell { q: 12, r: -12 }, Terrain::Clear),
            (Cell { q: 12, r: -11 }, Terrain::Clear),
            (Cell { q: 12, r: -10 }, Terrain::Clear),
            (Cell { q: 12, r: -9 }, Terrain::Clear),
            (Cell { q: 12, r: -8 }, Terrain::Clear),
            (Cell { q: 12, r: -7 }, Terrain::Clear),
            (Cell { q: 12, r: -6 }, Terrain::Clear),
            (Cell { q: 12, r: -5 }, Terrain::Clear),
            (Cell { q: 12, r: -4 }, Terrain::Clear),
            (Cell { q: 12, r: -3 }, Terrain::Clear),
            (Cell { q: 12, r: -2 }, Terrain::Clear),
            (Cell { q: 12, r: -1 }, Terrain::Clear),
            (Cell { q: 12, r: 0 }, Terrain::Clear),
        ]
    }

    /// Generate the `Chart` and the `Map` starting from params and a seed.
    pub fn generate_with(seed: Seed, params: &Params, islands: &[IslandFactory]) -> Result<DefaultChart, GenError> {
        let terrain = Self::terrain_generator();

        // Footprint of each Island, read off the hard-coded terrain.
        let mut footprints: BTreeMap<IslandId, BTreeSet<Cell>> = BTreeMap::new();
        for &(cell, t) in &terrain {
            if let Terrain::Island(id) = t {
                footprints.entry(id).or_default().insert(cell);
            }
        }

        let map = DefaultMap::try_new(12, terrain).expect("hard-coded terrain is a valid radius-12 map");
        let treasure = Cell { q: 3, r: 1 };

        // One Ship per corner, all at full hull.
        let ships = ShipId::ALL
            .into_iter()
            .zip(map.corners())
            .map(|(id, cell)| (id, ShipState::new(cell, params.hull_strength)))
            .collect();

        // Placeholder Clue: true by construction, since it is centred on the Treasure.
        let clues = [
            Clue::Within {
                centre: Cell { q: 6, r: -1 },
                radius: 5,
            },
            Clue::Beyond {
                centre: Cell { q: -7, r: 3 },
                radius: 8,
            },
            Clue::HalfPlane {
                axis: Axis::S,
                bound: Bound::AtLeast(-5),
            },
            Clue::Nearer {
                a: Cell { q: 5, r: 2 },
                b: Cell { q: 6, r: -5 },
            },
            Clue::Within {
                centre: Cell { q: -2, r: 4 },
                radius: 5,
            },
            Clue::HalfPlane {
                axis: Axis::R,
                bound: Bound::AtLeast(1),
            },
            Clue::Within {
                centre: Cell { q: 0, r: -2 },
                radius: 6,
            },
            Clue::Beyond {
                centre: Cell { q: -2, r: 3 },
                radius: 4,
            },
        ];

        let island_slots = if islands.is_empty() {
            BTreeMap::new()
        } else {
            footprints
                .into_iter()
                .enumerate()
                .map(|(i, (id, footprint))| {
                    let factory = islands[i % islands.len()];
                    let island = factory(IslandInit {
                        id,
                        clue: *clues.get(i).unwrap(),
                        footprint: footprint.clone(),
                        seed: seed.wrapping_add(u64::from(id.0)),
                    });
                    (
                        id,
                        IslandSlot {
                            footprint,
                            clue: *clues.get(i).unwrap(),
                            island,
                        },
                    )
                })
                .collect()
        };

        Ok(DefaultChart {
            params: *params,
            map,
            treasure,
            weather_seed: seed,
            round: 1,
            ships,
            islands: island_slots,
            winner: None,
            ending: None,
        })
    }

    ///Return all the id of all the afloat ship
    #[must_use]
    pub fn afloat(&self) -> BTreeSet<ShipId> {
        self.ships
            .iter()
            .filter(|(_, s)| s.is_afloat())
            .map(|(&id, _)| id)
            .collect()
    }

    pub fn learn(&mut self, ship: ShipId, clues: &[Clue]) {
        todo!()
    }

    /// Moves `ship` into `to`.
    fn do_move(&mut self, ship: ShipId, to: Cell) -> Vec<Report> {
        let ship_state = self.ships.get_mut(&ship).unwrap();
        ship_state.cell = to;
        ship_state.moored_at = None;
        vec![Report {
            to: BTreeSet::from([ship]),
            event: Event::Moved { ship, to },
        }]
    }

    fn do_moor(&mut self, ship: ShipId, island: IslandId) -> Vec<Report> {
        let ship_state = self.ships.get_mut(&ship).unwrap();
        ship_state.moored_at = Some(island);
        vec![Report {
            to: BTreeSet::from([ship]),
            event: Event::Moored { ship, island },
        }]
    }

    fn give_clue(&mut self, ship: ShipId) -> Vec<Report> {
        //right now give to the ship the full strenght clue.
        //TODO: when the island is developed, this function will use
        // an `island.ask_clue()` function
        let ship_state = self.ships.get_mut(&ship).unwrap();
        if let Some(island_id) = ship_state.moored_at {
            let clue = self.islands.get(&island_id).unwrap().clue;
            ship_state.clues.insert(clue);

            vec![Report {
                to: BTreeSet::from([ship]),
                event: Event::Answered {
                    island: island_id,
                    answer: IslandAnswer::Clue(clue),
                },
            }]
        } else {
            Vec::new()
        }
    }

    fn hitted(&mut self, ship: ShipId, target: ShipId) -> Vec<Report> {
        // Used this order (calculating new_hull and full_strength -> update the actual hull and set the status)
        // because of the mutable borrowing needed fot target_state.

        let new_hull = self.ships.get(&target).unwrap().hull.saturating_sub(1);
        let full_strength = if new_hull == 0 {
            self.full_strength(self.ships.get(&target).unwrap())
        } else {
            0
        };

        let target_state = self.ships.get_mut(&target).unwrap();
        target_state.hull = new_hull;

        if new_hull == 0 {
            target_state.status = Status::Sunk { full_strength };
        }

        vec![Report {
            to: BTreeSet::from([ship]),
            event: Event::Hit {
                target,
                hull: target_state.hull,
            },
        }]
    }

    fn alliance_propose(&mut self, ship: ShipId, target: ShipId) -> Vec<Report> {
        todo!()
    }

    fn alliance_accept(&mut self, ship: ShipId, proposer: ShipId) -> Vec<Report> {
        todo!()
    }

    fn alliance_break(&mut self, ship: ShipId) -> Vec<Report> {
        todo!()
    }

    fn do_declare(&mut self, ship: ShipId) -> Vec<Report> {
        let target_state = self.ships.get(&ship).unwrap();
        let declared_cell = target_state.cell;
        if self.treasure == declared_cell {
            self.winner = Some(ship);
        }

        vec![Report {
            to: self.ships.keys().copied().collect(),
            event: Event::Declared {
                ship,
                cell: declared_cell,
            },
        }]
    }
}

impl Chart for DefaultChart {
    type ValidatedAction = ValidatedAction;

    fn generate(seed: Seed, params: &Params) -> Result<Self, GenError> {
        Self::generate_with(seed, params, ISLANDS)
    }

    fn params(&self) -> &Params {
        &self.params
    }

    fn map(&self) -> &DefaultMap {
        &self.map
    }

    fn weather_seed(&self) -> Seed {
        self.weather_seed
    }

    fn round(&self) -> Round {
        self.round
    }

    ///Validate an action
    fn validate(&self, ship: ShipId, action: &Action) -> Result<ValidatedAction, Rejection> {
        // right now every action is valid
        // TODO: write an actual validate

        Ok(ValidatedAction::new(ship, *action))
    }

    ///Apply a validated action
    fn apply(&mut self, action: ValidatedAction) -> Vec<Report> {
        match action.into_parts() {
            (ship, Action::Move { to }) => self.do_move(ship, to),
            (ship, Action::Moor { island }) => self.do_moor(ship, island),
            (ship, Action::AskClue) => self.give_clue(ship),
            (ship, Action::Bombard { target }) => self.hitted(ship, target),
            (ship, Action::ProposeAlliance { to }) => self.alliance_propose(ship, to),
            (ship, Action::BreakAlliance) => self.alliance_break(ship),
            (ship, Action::Declare) => self.do_declare(ship),
            (ship, Action::Hold) => Vec::new(),
            _ => panic!(),
        }
    }

    ///Took the game to the next round
    fn advance_round(&mut self) -> Vec<Report> {
        todo!()
    }

    fn view_for(&self, ship: ShipId) -> Option<ShipView> {
        todo!()
    }

    fn spectator_view(&self) -> SpectatorView {
        todo!()
    }

    fn outcome(&self) -> Option<Outcome> {
        todo!()
    }
}
