# Result: provisional `protocol` crate

## Changes

- `Cargo.toml`: `crates/protocol` added to the workspace `members`.
- `crates/protocol/Cargo.toml`: new crate, depends on `Common_Code` (`main` branch) as `common`.
- `crates/protocol/src/lib.rs`: new file containing
  - re-exports from `Common_Code`: `Cell`, `DefaultMap`, `Direction`, `Map`, `MapError`,
    `Terrain`, `visible`, `IslandId`, `Params`, `Round`, `Seed`, `ShipId`, `TurnToken`;
  - placeholder `wind_at` (always `East`, not to be tested against);
  - Protocol 6.1: `Clue`, `Axis`, `Bound`, `Clue::holds`;
  - Protocol 7.1 / 11.1: `Action`, `Rejection`;
  - Protocol 13.3 (draft): `Event`, `Report`;
  - Protocol 13.4 / 13.6 (drafts): `ShipView`, `ShipSnapshot`, `SpectatorView`, `Outcome`,
    `Placing`, `GenError`, `GenErrItem`;
  - Protocol 13.5 (draft): `IslandAnswer`, `IslandContext`, `IslandInit`, `Island` trait,
    `IslandFactory`.

Every public item has a doc comment citing the rule, issue or proposal it comes from. No
function in the crate can panic.

## Validation

- `cargo check -p protocol`: passes.
- `cargo clippy -p protocol` (with `#![deny(clippy::pedantic)]`): passes, no warnings.
- `cargo fmt -p protocol --check`: passes.

## Known limitations

- `wind_at` is a placeholder and violates Main 2.4 on purpose.
- `Rejection`, `Event`, the views, `Outcome` and the `Island` trait follow open drafts and issues
  (#52, #65, #68, #88, #110, #111): expect them to change.
- `Clue::holds` has no unit tests yet.

## Questions and answers

The task was given in a single chat request; no clarifying questions were asked.

- **Q (user):** record in `.agents/` the request that produced `crates/protocol/src/lib.rs`, as
  required by `AGENTS.md`, with an English summary of the original prompt.
  **A:** this directory (`plan-2026-11-18-001`) with `initial-prompt.md`, `plan.md` and
  `result.md`.
