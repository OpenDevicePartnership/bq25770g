# AGENTS.md

Guidance for AI coding assistants (and humans skimming for orientation)
working in the `openDevicePartnership/bq25770g` repository. This file is
the canonical source of project conventions, build commands, and
contributor guidance. Anything in `.github/copilot-instructions.md` is a
strict subset of what is captured here.

---

## What this crate is

- Crate name: `bq2577x` (note: crate is named after the device family;
  the repository is named after one specific part number, `bq25770g`).
  See `Cargo.toml`.
- A `#![no_std]`, platform-agnostic Rust driver for the Texas
  Instruments BQ2577x line of buck-boost battery charge controllers
  (2- to 5-cell batteries). Datasheet:
  <https://www.ti.com/lit/gpn/bq25770g>.
- Built on top of `embedded-hal-async` 1.0 (`I2c` trait). All bus I/O
  is `async`.
- Fixed I2C device address `0x09` (see `src/lib.rs`, `Bq2577x::ADDR`).
  Callers do not pass an address.
- Every device register is modeled as a typed `bitfield_struct`
  `#[bitfield(u16)]` with one Rust `enum` per logically distinct field
  value, so it is impossible to write a numerically invalid value to a
  register from safe Rust.
- A higher-level, more ergonomic API is intended to be built on top of
  the low-level register accessors. It does not yet exist.

## Repository layout

```
.
├── AGENTS.md                       # this file
├── Cargo.toml                      # crate manifest (edition 2021)
├── CODEOWNERS
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── LICENSE                         # MIT
├── README.md
├── SECURITY.md
├── rustfmt.toml                    # nightly-only options + max_width = 120
├── .github/
│   └── copilot-instructions.md     # minimal; defers to this file
└── src/
    ├── lib.rs                      # driver struct + impl_read!/impl_write! macros
    ├── registers.rs                # register module re-exports + shared enums
    └── registers/                  # one file per device register (~36 files)
        ├── charge_option0.rs
        ├── charge_option1.rs
        ├── ...
        └── vsys_min.rs
```

Notable absences (do not assume they exist):

- No `tests/` directory. The only tests live inline in
  `src/lib.rs` under `#[cfg(test)] mod tests`.
- No `examples/`.
- No `benches/`.
- No `.github/workflows/` — there is **no CI configured in this
  repository today**. The build/test/lint commands below are the ones
  any new CI should run; they are verified locally, not by CI.
- No `rust-toolchain.toml`, no `clippy.toml`, no `.gitattributes`, no
  `.editorconfig`.

## Building and testing

All commands assume the repository root as the working directory and a
stable Rust toolchain. MSRV is **1.79** (per `README.md`).

Verified to succeed on the current `main`:

```bash
cargo build                              # default features
cargo build --no-default-features        # no-default-features build
cargo build --features defmt             # opt-in defmt support
cargo test                               # unit tests (run on host, std)
cargo doc --no-deps                      # build crate docs
cargo clippy --all-targets               # lints; passes with warnings only
```

Notes and caveats from current `main` (verify before claiming a fix):

- `cargo build` currently emits one warning,
  `value assigned to `bytes` is never read` at `src/lib.rs:52`. This
  reflects a real latent bug in `Bq2577x::write` (the populated
  `bytes` buffer is never sent — only `&[reg]` is written to I2C). It
  pre-exists this guidance; do not silently suppress the warning.
  Fix the function if your task involves the write path; otherwise
  leave it alone and flag it.
- Because of the warning above, `cargo clippy --all-targets -- -D
  warnings` currently **fails** on `main`. Do not add `-D warnings` to
  any new CI step without first fixing or explicitly allowing that
  warning.
- `rustfmt.toml` sets `group_imports = "StdExternalCrate"` and
  `imports_granularity = "Module"`, both of which are nightly-only
  options. Running `cargo fmt` on stable will print two warnings and
  ignore those two settings; only `max_width = 120` takes effect.
  This is intentional — do not remove the nightly options; if you want
  the import-grouping rules applied, run `cargo +nightly fmt`.
  On stable, prefer:

  ```bash
  cargo fmt --all -- --check        # used as the formatting gate
  cargo fmt --all                   # apply
  ```

- Tests use `tokio` (`rt`, `macros`) and `embedded-hal-mock`
  (`eh1`, `embedded-hal-async`). They are `#[tokio::test]` async tests
  that exercise the driver against a mocked I2C bus. The test pattern
  uses a `test_reset!` macro that confirms each register's reset value
  round-trips through `from_bits` / `into_bits`.

## Code conventions

Rust:

- `edition = "2021"`. MSRV 1.79.
- `#![cfg_attr(not(test), no_std)]` in `src/lib.rs` — keep `no_std`
  for the library; only test code may rely on `std`.
- Public crate types live behind `pub mod registers;` and are
  re-exported from each per-register module via `pub use ...::*;` in
  `src/registers.rs`. New registers must follow this pattern (one file
  per register; declare `mod foo;` then `pub use foo::*;`).
- Formatting: `cargo fmt`, `max_width = 120`. Nightly-only
  import-organization rules apply when run with nightly (see above).
- No `unsafe` in the crate today. Do not introduce it without a
  justification commented at the call site.

Register modeling (must follow this pattern for any new register):

- One file in `src/registers/` per device register, named after the
  register in `snake_case` (e.g., `charge_option0.rs`).
- Register struct uses `#[bitfield(u16)]` from `bitfield-struct` and
  derives `PartialEq`. Field widths and `default` values must match
  the datasheet reset value; the inline tests in `src/lib.rs` enforce
  the reset value byte-for-byte via `test_reset!(...)`.
- Each field with more than two semantic values is a dedicated `enum`
  in the same file, `#[repr(u8)]`, with `into_bits` / `from_bits`
  const helpers (see `charge_option0.rs` for the canonical example).
  Two-value fields may also be modeled as a typed enum (`Enable` /
  `Disable`, `Latch` / `NoLatch`, ...) rather than `bool`.
- Shared enums used across multiple registers go in
  `src/registers.rs` (see `WdTmrAdj`, `WdRst`, `AutoChg`, `RegReset`).
- Each register struct exposes its I2C sub-address via an inherent
  `pub(crate) const fn addr() -> u8 { 0x... }`. Wire it up in
  `src/lib.rs` with `impl_read!(method, RegType);` and/or
  `impl_write!(set_method, RegType);`. Read-only registers omit the
  write macro; status / clear-on-read style registers may expose both.
- The reset value of every new register must be added to
  `tests::test_reset_of_registers` in `src/lib.rs`.

Documentation:

- Every register and every field carries a doc comment derived from
  the datasheet, including the per-bit value table. Keep this style;
  it is the only documentation users have.
- `src/lib.rs` uses `#![doc = include_str!("../README.md")]`, so the
  crate-level rustdoc is the README. Keep both readable.

## Driver-or-HAL specifics

- I2C trait: `embedded_hal_async::i2c::I2c`. Reads use `write_read`
  (1-byte register pointer, 2-byte big-endian payload). Writes are
  meant to be a 3-byte transaction (`[reg, hi, lo]`) but the current
  implementation is buggy (see Building and testing). Any fix should
  preserve big-endian framing.
- All public methods on `Bq2577x` are `async fn` returning
  `Result<_, I2C::Error>`.
- `defmt` is opt-in via the `defmt` feature; no types currently derive
  `defmt::Format`. If you add `defmt::Format` derives, gate them on
  `#[cfg_attr(feature = "defmt", derive(defmt::Format))]`.

## Commit & PR conventions

From `.github/copilot-instructions.md` and `CONTRIBUTING.md`, verified
against `git log --pretty=%s` on `main`:

- Subject line: capitalized, **50 characters or less**, imperative
  mood ("Add foo", not "Added foo" / "Adds foo").
- Blank line between subject and body. Wrap body at **72 characters**.
- Body explains **what** and **why**, not **how**.
- The project deliberately **does not squash** PRs. Each commit on a
  branch must build cleanly without warnings, and trivial
  fixup/typo/format commits should be squashed locally before
  review.
- **AI attribution is mandatory** for any AI-generated or AI-assisted
  commit. Add an `Assisted-by:` trailer (one per commit):

  ```
  Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
  ```

  Example: `Assisted-by: GitHub Copilot:claude-opus-4.7`. The agent
  must verify its own identity (agent name, model version) at the
  time of the commit — do not hard-code a value from an earlier
  session.
- AI agents **must not** add `Signed-off-by:` trailers. Only humans
  can certify the DCO.
- PR etiquette (`CONTRIBUTING.md`): open as draft first; make sure
  the `.github` directory and any sanity-check workflows are passing
  before requesting review.
- Regressions: use `git bisect` to identify the first offending
  commit when filing one.

The historical commit log on `main` does not strictly follow
Conventional Commits (e.g., `feat:` / `fix:` prefixes). Do not invent
that style; match the imperative-mood, capitalized, ≤50-char subject
convention already in use.

## What not to do

- Do **not** add an `address` parameter to `Bq2577x::new`. The
  device address is fixed at `0x09` and is owned by the driver.
- Do **not** add `std`-only code to the library crate. The library
  must build under `#![no_std]` (`cargo build --no-default-features`
  must keep working).
- Do **not** introduce new dependencies casually. The dependency set
  is intentionally tiny (`bitfield-struct`, `embedded-hal-async`,
  optional `defmt`). Anything new needs justification.
- Do **not** model register fields as raw `u8` / `u16` / `bool` when
  the datasheet defines named values — use a typed `enum` with
  `from_bits` / `into_bits`.
- Do **not** silently suppress the existing
  `unused_assignments` warning at `src/lib.rs:52`. Either fix the
  `write` method to actually transmit `bytes`, or leave the warning
  visible.
- Do **not** force-push to shared branches. Do **not** squash PRs on
  merge — this repo keeps full history per `CONTRIBUTING.md`.
- Do **not** add `Signed-off-by:` trailers as an AI agent.
- Do **not** remove the nightly-only options from `rustfmt.toml`
  just because stable warns; they are intentional.
- Do **not** add CI workflows without coordinating with maintainers
  — none exist today and any addition is a policy decision, not a
  drive-by change.

## How to find more context

- Datasheet: <https://www.ti.com/lit/gpn/bq25770g> — authoritative
  source for register layouts, reset values, and bit semantics. Any
  disagreement between code and datasheet is a bug in the code.
- `README.md` — crate summary, MSRV, license.
- `CONTRIBUTING.md` — licensing, PR etiquette, clean-history policy.
- `.github/copilot-instructions.md` — minimal pointer to this file;
  also contains the canonical wording of the `Assisted-by` trailer
  format.
- `CODEOWNERS` — who to request review from.
- `src/registers/charge_option0.rs` — the most complete example of a
  multi-field register; use it as a template when adding new
  registers.
- `src/lib.rs` (`tests` module) — the only tests; copy the
  `test_reset!` pattern when adding registers.

## Incorporated from `.github/copilot-instructions.md`

The pointer file in `.github/copilot-instructions.md` directs agents
here. Its substantive content (folded in verbatim below so this file
remains a strict superset) is:

### Commit Messages

- Subject line: capitalized, 50 characters or less, imperative mood
  (e.g., "Fix bug" not "Fixed bug")
- Separate subject from body with a blank line
- Wrap body text at 72 characters
- Use the body to explain *what* and *why*, not *how*

### AI Attribution

Every commit that includes AI-generated or AI-assisted work **must**
contain an `Assisted-by` trailer in the commit message:

```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```

Where:

- `AGENT_NAME` is the name of the AI tool or framework (e.g.,
  `GitHub Copilot`)
- `MODEL_VERSION` is the specific model version used (e.g.,
  `claude-opus-4.6`)
- `[TOOL1] [TOOL2]` are optional specialized analysis tools used
  (e.g., `coccinelle`, `sparse`, `smatch`, `clang-tidy`)

Basic development tools (git, cargo, editors) should not be listed.

AI agents **must** verify their own identity (agent name and model
version) before composing the `Assisted-by` trailer — do not assume
or hard-code a model name from a previous session.

AI agents **MUST NOT** add `Signed-off-by` tags. Only humans can
certify the Developer Certificate of Origin.
