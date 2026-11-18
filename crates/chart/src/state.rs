use std::collections::BTreeSet;

use island::IslandType;
use protocol::{Cell, Clue, IslandId, ShipId};

///Contains all the information about the actual state of a ship.
pub(crate) struct ShipState {
    pub(crate) cell: Cell,
    pub(crate) hull: u32,
    pub(crate) clues: BTreeSet<Clue>,
    pub(crate) ally: Option<ShipId>,
    pub(crate) proposal: Option<ShipId>,
    pub(crate) moored_at: Option<IslandId>,
    pub(crate) status: Status,
}

///If a ship is afloat or not
pub(crate) enum Status {
    Afloat,
    Sunk { full_strength: u32 },
}

impl ShipState {
    pub(crate) fn new(cell: Cell, hull: u32) -> Self {
        Self {
            cell,
            hull,
            clues: BTreeSet::new(),
            ally: None,
            proposal: None,
            moored_at: None,
            status: Status::Afloat,
        }
    }

    pub(crate) fn is_afloat(&self) -> bool {
        matches!(self.status, Status::Afloat)
    }
}

///Information about the island
pub(crate) struct IslandSlot {
    ///The set that contains cells of the Island
    pub(crate) footprint: BTreeSet<Cell>,
    ///The clue of the Island
    pub(crate) clue: Clue,
    ///The type of the island
    pub(crate) island: IslandType,
}

///All the possible endings
pub(crate) enum Ending {
    Declared { winner: ShipId, cell: Cell },
    RoundCap,
    NobodyAfloat,
}
