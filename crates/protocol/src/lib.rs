//! Provisional protocol types for the group's crates.
//!
//! Everything protocol.tex describes and `Common_Code` does not provide yet lives here, in one
//! place. Every crate of the group imports from this crate and never straight from
//! `Common_Code`: the day `Common_Code` publishes a type, its definition here becomes a `pub use`
//! and nothing else in the workspace changes.

#![deny(unsafe_code)]
#![deny(clippy::pedantic)]

use std::collections::{BTreeMap, BTreeSet};

pub use common::geometry::{Cell, DefaultMap, Direction, Map, MapError, Terrain, visible};
pub use common::shared::{IslandId, Params, Round, Seed, ShipId, TurnToken};

/// Placeholder until the `wind` crate reaches `Common_Code`'s `main`. It breaks the second law
/// of the wind (Main 2.4) on purpose: replace it, do not test against it.
#[must_use]
pub fn wind_at(_weather_seed: Seed, _round: Round) -> Direction {
    Direction::East
}

// ---------------------------------------------------------------- Protocol 6.1

/// A predicate over cells (Protocol 6.1).
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Clue {
    /// The Treasure is at most `radius` from `centre`.
    Within { centre: Cell, radius: u32 },
    /// The Treasure is further than `radius` from `centre`.
    Beyond { centre: Cell, radius: u32 },
    /// The Treasure's coordinate on `axis` respects `bound`.
    HalfPlane { axis: Axis, bound: Bound },
    /// The Treasure is strictly nearer `a` than `b`.
    Nearer { a: Cell, b: Cell },
}

/// One of the three hex axes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Axis {
    /// The `q` coordinate.
    Q,
    /// The `r` coordinate.
    R,
    /// The `s` coordinate.
    S,
}

/// A floor or a ceiling on one axis.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Bound {
    /// Coordinate `>=` the value.
    AtLeast(i32),
    /// Coordinate `<=` the value.
    AtMost(i32),
}

impl Clue {
    /// Whether `cell` satisfies this Clue. Pure and total on cells of a map.
    #[must_use]
    pub fn holds(self, cell: Cell) -> bool {
        match self {
            Clue::Within { centre, radius } => cell.distance(centre) <= radius,
            Clue::Beyond { centre, radius } => cell.distance(centre) > radius,
            Clue::HalfPlane { axis, bound } => {
                let v = match axis {
                    Axis::Q => cell.q,
                    Axis::R => cell.r,
                    Axis::S => cell.s(),
                };
                match bound {
                    Bound::AtLeast(k) => v >= k,
                    Bound::AtMost(k) => v <= k,
                }
            }
            Clue::Nearer { a, b } => cell.distance(a) < cell.distance(b),
        }
    }
}

// ---------------------------------------------------------------- Protocol 7.1, 11.1

/// One action per round (Protocol 7.1, Main 5.3).
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Action {
    /// Sail to a neighbouring cell.
    Move { to: Cell },
    /// Moor at an adjacent Island.
    Moor { island: IslandId },
    /// Ask the Island this Ship is moored at.
    AskClue,
    /// Fire on a visible Ship within range.
    Bombard { target: ShipId },
    /// Offer an alliance.
    ProposeAlliance { to: ShipId },
    /// Accept a standing proposal.
    AcceptAlliance { proposer: ShipId },
    /// End the current alliance.
    BreakAlliance,
    /// Claim the Treasure lies in this Ship's cell.
    Declare,
    /// Do nothing.
    Hold,
}

/// Why an action was refused (Protocol 11.1, still a sketch).
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Rejection {
    /// Admiralty: the submission names a round that is not open.
    WrongRound { open: Round },
    /// Admiralty: the token is not the one issued.
    StaleToken,
    /// Admiralty: the outbox is over the caps.
    TooMuchMail,
    /// An action this Chart does not handle (yet). Not in the Protocol: proposal B13.
    Unsupported,
    /// Move: the target is not adjacent.
    NotANeighbour,
    /// Move: the target is outside the sea.
    OutsideArchipelago,
    /// Move: the target is land.
    CellIsLand,
    /// Move: another Ship is there.
    CellOccupied { by: ShipId },
    /// Move: the course is shut by the wind.
    AgainstTheWind,
    /// Moor: no cell of that Island is adjacent.
    NotAdjacentToIsland,
    /// Moor: every berth is taken.
    NoBerth,
    /// Moor: the Island turned the Ship away.
    IslandRefused,
    /// `AskClue`: the Ship is not moored.
    NotMoored,
    /// Bombard: the target is beyond `bombard_range`.
    OutOfRange,
    /// Bombard: the target is not in sight.
    NotVisible,
    /// The named Ship is not on the roster, or not afloat.
    NoSuchShip,
    /// Alliance: already allied.
    AlreadyAllied,
    /// Alliance: no proposal from that Ship.
    NoSuchProposal,
    /// Alliance: nobody to break with.
    NotAllied,
    /// Bombard: the target has sunk.
    TargetHasSunk,
}

// ---------------------------------------------------------------- Protocol 13.3 (draft, #65)

/// What a Ship is told happened. Draft: the WG has not settled it (#65).
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Event {
    /// A Ship moved into `to`.
    Moved { ship: ShipId, to: Cell },
    /// A Ship entered a rock and was left with `hull`.
    StruckRock { ship: ShipId, hull: u32 },
    /// `by` fired at `target`. Told to the victim and to whoever sees its cell.
    Fired { by: ShipId, target: ShipId },
    /// The shot landed. Told to the attacker only.
    Hit { target: ShipId, hull: u32 },
    /// The shot missed. Told to the attacker only.
    Missed { target: ShipId },
    /// A Ship went down.
    Sunk { ship: ShipId },
    /// A Ship moored at an Island.
    Moored { ship: ShipId, island: IslandId },
    /// What an Island answered. Told to the asking Ship only.
    Answered { island: IslandId, answer: IslandAnswer },
    /// Public: told to everybody.
    AllianceFormed { a: ShipId, b: ShipId },
    /// Public: told to everybody.
    AllianceBroken { by: ShipId, with: ShipId },
    /// Public: told to everybody, with the cell named.
    Declared { ship: ShipId, cell: Cell },
}

/// One event and the Ships it is told to (proposal A4). The Cartographer is told every one.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Report {
    /// Who is told.
    pub to: BTreeSet<ShipId>,
    /// What happened.
    pub event: Event,
}

// ---------------------------------------------------------------- Protocol 13.4 and 13.6 (drafts)

/// What one Ship may know (proposal P1, #110).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShipView {
    /// The round this view is valid for.
    pub round: Round,
    /// The wind of that round.
    pub wind: Direction,
    /// This Ship's cell.
    pub cell: Cell,
    /// This Ship's hull.
    pub hull: u32,
    /// Everything this Ship holds.
    pub clues: BTreeSet<Clue>,
    /// Its ally, if any.
    pub ally: Option<ShipId>,
    /// Its outgoing proposal, if any.
    pub proposal: Option<ShipId>,
    /// The Island it is moored at, if any.
    pub moored_at: Option<IslandId>,
    /// Every other Ship in a cell this Ship or its ally can see.
    pub ships: BTreeMap<ShipId, Cell>,
}

/// One Ship as the spectator sees it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShipSnapshot {
    /// Its cell (the last one, if sunk).
    pub cell: Cell,
    /// Its hull.
    pub hull: u32,
    /// Its hold.
    pub clues: BTreeSet<Clue>,
    /// Its ally.
    pub ally: Option<ShipId>,
    /// Its outgoing proposal.
    pub proposal: Option<ShipId>,
    /// Where it is moored.
    pub moored_at: Option<IslandId>,
    /// Whether it is still afloat.
    pub afloat: bool,
}

/// Everything (draft, #52).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpectatorView {
    /// The open round.
    pub round: Round,
    /// Its wind.
    pub wind: Direction,
    /// The secret.
    pub treasure: Cell,
    /// Every Ship.
    pub ships: BTreeMap<ShipId, ShipSnapshot>,
    /// The Clue each Island was issued, at full strength.
    pub issued: BTreeMap<IslandId, Clue>,
}

/// How the game ended (proposal P2, #111).
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    /// Somebody declared correctly.
    Won { winner: ShipId, cell: Cell },
    /// Nobody did: round cap, or nobody afloat.
    Ranked { nobody_afloat: bool, ranking: Vec<Placing> },
}

/// One line of the ranking.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Placing {
    /// The Ship.
    pub ship: ShipId,
    /// Issued Clues it holds at full strength (recorded at sinking if sunk).
    pub full_strength: u32,
    /// Whether it sank.
    pub sunk: bool,
}

/// Why `generate` failed (#88).
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GenError {
    /// Step 2: `archipelago_radius` does not exceed `vision_range`.
    VisionRangeTooHigh,
    /// Steps 3-5 and 7: the sea is too crowded for `item`.
    CantPlace { item: GenErrItem },
    /// Step 6, after every attempt.
    WaterNotConnected,
    /// Step 9, after every attempt.
    Unsolvable,
}

/// What could not be placed.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GenErrItem {
    /// An Island footprint.
    Island,
    /// A cliff.
    Cliff,
    /// A rock.
    Rock,
    /// The Treasure.
    Treasure,
}

// ---------------------------------------------------------------- Protocol 13.5 (draft, #68)

/// What an Island answers to `AskClue`. None of the four is a rejection (Main 5.3).
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IslandAnswer {
    /// Its Clue, possibly weakened, always of the same kind (Main 4.2).
    Clue(Clue),
    /// Not yet: this many more rounds moored.
    Wait { rounds: u32 },
    /// Not this time.
    Refuse,
    /// Nothing further for this Ship.
    Done,
}

/// What the Chart tells an Island whenever it calls it.
#[derive(Clone, Copy, Debug)]
pub struct IslandContext {
    /// The current round.
    pub round: Round,
}

/// What the Chart hands an Island when it builds it (Main 4.1).
#[derive(Clone, Debug)]
pub struct IslandInit {
    /// Its identifier.
    pub id: IslandId,
    /// Its one true Clue, at full strength.
    pub clue: Clue,
    /// Its land cells.
    pub footprint: BTreeSet<Cell>,
    /// Its own randomness (Main 10.1).
    pub seed: Seed,
}

/// The trait every group's Island implements. Draft (#68). `Send` because the Chart moves
/// into the Admiralty's thread.
pub trait Island: Send {
    /// How many Ships may be moored at once; `None` for no limit.
    fn berths(&self) -> Option<u32>;
    /// Whether it accepts `ship` now. Asked from `validate`, so it must be pure.
    fn will_berth(&self, ship: ShipId, ctx: &IslandContext) -> bool;
    /// `ship` has moored.
    fn on_moor(&mut self, ship: ShipId, ctx: &IslandContext);
    /// `ship` left, by moving away or by sinking.
    fn on_leave(&mut self, ship: ShipId, ctx: &IslandContext);
    /// `ship`, moored here, asks for the Clue.
    fn ask_clue(&mut self, ship: ShipId, ctx: &IslandContext) -> IslandAnswer;
    /// Its turn at the end of every round.
    fn end_of_round(&mut self, ctx: &IslandContext);
}
