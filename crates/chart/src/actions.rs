use protocol::{Action, ShipId};

///A valid action with the relative ship id
#[derive(Debug)]
pub struct ValidatedAction {
    ship: ShipId,
    action: Action,
}

impl ValidatedAction {
    fn into_parts(self) -> (ShipId, Action) {
        (self.ship, self.action)
    }
}
