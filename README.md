# Rusty Crub

This repository contains the project for the course Advanced Programming, A.Y. 2026/2027

## Members
 - Enea D'Angio
 - Laura Prandina
 - Ivan Nedeljkovic
 - Michele Porcellato

## Development

This repo uses [pre-commit](https://github.com/pre-commit/pre-commit) to ensure code quality and to enforce formatting standards.
Before committing anything, follow the [install guide](https://pre-commit.com/#install) (you can also do `brew install pre-commit
`) and run `pre-commit install` to install all the pre-commit hooks.

# Project Template

This repository is a template for **group and individual contributions** for the Advanced Programming course project, A.Y. 2026/2027.

## Getting Started

This repository **must be forked**.
Do not create a new repository from scratch.
You can rename the fork afterward if desired.

When initializing a library or binary crate, you must add the following crate-level attributes to the crate root:

- For a library crate, add them to `src/lib.rs`.
- For a binary crate, add them to `src/main.rs`.

```
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
```

These rules are part of the project's coding standards and must be present from the beginning of development.

## README Requirements

You may modify this `README.md` to document your project.
At a minimum, clearly specify:

* The **group name**.
* Which **component(s)** your repository implements.
* Who **contributed to which component(s)**.

Feel free to add any additional information that helps explain the project, its structure, or how to use it.

## AGENTS.md

You can modify `AGENTS.md`, but the current content must be kept as is.

## CLAUDE.md

`CLAUDE.md` must import `@AGENTS.md` at the top.
Do not remove it.

## Rules for code in this repository

These are the project's coding principles, and here they are enforced rather than encouraged.

- **No `unsafe`.**
- **No undocumented panics.** If a function can panic, say so in its doc comment and say when. Most things in here should not panic at all.
- **rustfmt.** Configure your editor.
- Before committing, `cargo fmt --all` must pass. If it does not pass, and the latest commit is not authored by the current user, make a single commit: `style: format code`.
