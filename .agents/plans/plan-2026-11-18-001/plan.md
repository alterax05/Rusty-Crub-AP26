# Plan: provisional `protocol` crate

## Goal

Give every crate of the group a single place to import protocol types from, so that work on the
Chart, Admiralty, Ship and Island can start before `Common_Code` publishes the missing types.

## Sources

- `project_docs/main.pdf` and `project_docs/protocol.pdf` (rules cited as "Main x.y" and
  "Protocol x.y").
- `project_docs/chart-minimale.md`: the type hypotheses, in particular its appendix with a draft
  of this crate.
- `project_docs/results-2026-11-13-errori-mancanti.md`: proposals referenced as A4, B13, P1, P2.
- Open issues of the protocol repository: #52, #65, #68, #73, #88, #110, #111.
- `Common_Code` `main`: what is already defined (`Cell`, `Direction`, `Map`, `DefaultMap`,
  `MapError`, `Terrain`, `visible`, `IslandId`, `Params`, `Round`, `Seed`, `ShipId`, `TurnToken`).

## Design

1. New workspace member `crates/protocol`, depending on `Common_Code` as a git dependency
   (`ap-2026-common-code`, branch `main`), renamed `common`.
2. Re-export with `pub use` everything `Common_Code` already provides. No crate of the group
   imports from `Common_Code` directly.
3. Define locally everything still missing, grouped by the Protocol section it comes from:
   - 6.1: `Clue`, `Axis`, `Bound`, and `Clue::holds`.
   - 7.1 / 11.1: `Action`, `Rejection` (still a sketch; `Unsupported` is proposal B13).
   - 13.3 (draft, #65): `Event`, `Report` (proposal A4).
   - 13.4 / 13.6 (drafts): `ShipView` (P1, #110), `ShipSnapshot`, `SpectatorView` (#52),
     `Outcome` and `Placing` (P2, #111), `GenError` and `GenErrItem` (#88).
   - 13.5 (draft, #68): `IslandAnswer`, `IslandContext`, `IslandInit`, the `Island` trait and
     `IslandFactory`.
4. A placeholder `wind_at` that always returns `Direction::East`, documented as breaking the
   second law of the wind on purpose, so that it is never tested against.
5. Enums that the WG may still extend are `#[non_exhaustive]`. All collections are `BTree*` for
   determinism (Main 10.1).

## Replacement strategy

When `Common_Code` publishes one of these types, its local definition is deleted and replaced by
a `pub use` from `common`. Nothing else in the workspace has to change.

## Validation

- `cargo check -p protocol`
- `cargo clippy -p protocol` with `#![deny(clippy::pedantic)]`
- `cargo fmt --all --check`
