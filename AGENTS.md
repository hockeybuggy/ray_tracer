# Ray Tracer

Rust implementation of *The Ray Tracer Challenge* (Jamis Buck). The book PDF
lives at the repo root (`the-ray-tracer-challenge_P1.0.pdf`); chapter N page
offsets are in the PDF bookmarks.

**Progress:** chapters 1–12 complete. Next up: chapter 13 (Cylinders).

## Working agreement for book chapters

Douglas is working through the book to learn. For new chapter material:

- The agent translates the book's Cucumber scenarios into failing Rust tests
  and explains the algorithm's intuition in conversation.
- **Douglas writes the implementation.** The agent reviews and gives hints
  only when asked, and must not write the implementation unless explicitly
  asked to.
- Refactors of already-learned material may be done by the agent, with
  Douglas reviewing the diff.

## Commands

- `cargo test` — unit + render tests
- `cargo fmt --all -- --check` — enforced by CI
- `cargo bench` — criterion benchmark in `benches/simple_world.rs`
- `./scripts/check_gallery.sh` — every fixture PNG must appear in the README
  gallery (enforced by CI)

## Architecture

Library crate; all modules exposed via `src/lib.rs`. One file per concept:
`tuple` (points/vectors), `color`, `canvas` (PPM/image output), `matrix`,
`transformation` (`Transform` trait), `ray`, `shape`, `intersection`,
`material`, `patterns`, `lighting`, `lights`, `world` (+ `WorldBuilder`),
`camera`.

Shapes are a single `Shape` struct with a private `ShapeType` enum
(`src/shape.rs`), dispatched by `match` — not a trait. Adding a shape means:
new enum variant, `default_<shape>()` constructor, and match arms in
`local_intersect` / `local_normal_at` (both operate in object space;
`Shape::intersect` and `normal_at` handle the world/object conversions).

Transform chaining left-multiplies: `.scaling(..).translation(..)` builds
`T * S`, i.e. book order — the unit shape is scaled first, then translated.

## Conventions

- Unit tests are inline `#[cfg(test)] mod <topic>_tests` blocks at the bottom
  of each module; test fns named `test_...` in descriptive snake_case.
- Approximate float assertions: `assert_tuple_approx_eq!`,
  `assert_color_approx_eq!`, `assert_matrix_approx_eq!` from
  `src/test_helpers.rs` (tolerance 1e-5).
- Render tests in `tests/` build a scene with `WorldBuilder`, render it, and
  compare against `tests/fixtures/*.png` (helpers in
  `tests/shared_test_helpers.rs`). On mismatch the new render is written to
  the repo root for inspection; promote it to `tests/fixtures/` only after
  visually verifying it.
- Each render test has a `SCALE` constant for rendering higher-res versions.
- New fixtures must be added to the README gallery or CI fails.

## Render fixture gotchas

- Fixture comparison is exact byte equality, fixtures are rendered on macOS,
  and CI runs Linux — any float that rounds differently across platforms
  breaks CI even when the render passes locally.
- The main culprit: checker patterns on planes. Intersection points straddle
  the pattern-space y=0 cell boundary within float error, and macOS/Linux
  resolve the noise differently. Scenes nudge the pattern with a small y
  translation; the offset must be a *non-integer* number of pattern cells
  (half a cell is safest) or it lands back on a boundary.
- When a render test fails on CI, the workflow uploads the differing render
  as a `differing-renders` artifact: `gh run download <run-id> --name
  differing-renders`, then diff it against the fixture pixel-by-pixel to see
  whether it's a pattern flip (clustered large deltas) or float drift (±1-2).
- The fixture PNGs are tiny; upscale with nearest-neighbour (e.g. Pillow)
  before visually inspecting a render.
