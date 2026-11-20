use protocol::{Action, ShipId};

///A valid action with the relative ship id
#[derive(Debug)]
pub struct ValidatedAction {
    ship: ShipId,
    action: Action,
}

impl ValidatedAction {
    pub(crate) fn new(ship: ShipId, action: Action) -> Self {
        Self { ship, action }
    }

    pub(crate) fn into_parts(self) -> (ShipId, Action) {
        (self.ship, self.action)
    }
}
