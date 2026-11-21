//! This file wrap all the chart function between two rounds in a game.

use protocol::{Outcome, Placing, Report, Round};

use crate::DefaultChart;
use crate::state::{Ending, ShipState};

impl DefaultChart {
    /// advances the rounds
    pub(crate) fn close_round(&mut self) -> Vec<Report> {
        todo!()
    }

    /// call every island at the end of the round
    fn islands_turn(&mut self) -> Vec<Report> {
        todo!()
    }

    /// check if the game is over
    fn check_ending(&self, closed: Round) -> Option<Ending> {
        todo!()
    }

    /// describe the ending retuning an `Outcome`
    pub(crate) fn describe(&self, ending: &Ending) -> Outcome {
        todo!()
    }

    /// Build the ranking
    fn ranking(&self) -> Vec<Placing> {
        todo!()
    }

    /// Return the score of a ship after it was sunk
    pub(crate) fn full_strength(&self, ship: &ShipState) -> u32 {
        0
    }
}
