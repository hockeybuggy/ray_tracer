# Ray Tracer

Rust implementation of *The Ray Tracer Challenge* (Jamis Buck). The book PDF
lives at the repo root (`the-ray-tracer-challenge_P1.0.pdf`); chapter N page
offsets are in the PDF bookmarks.

**Progress:** chapters 1–16 complete — all of the book's core material.
What remains is optional: the chapter 17 extensions (area lights, focal
blur, anti-aliasing, texture maps, …) and the appendix cover image.

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
- `cargo run --release --bin render -- scenes/<scene>.toml [--scale N]` —
  render a TOML-described scene to a PNG
- `cargo run --release --bin animate -- animations/<animation>.toml
  [--scale N]` — render animation frames; `./scripts/make_gif.sh
  <frames_dir> <output.gif>` assembles them with ffmpeg

## Architecture

Library crate; all modules exposed via `src/lib.rs`. One file per concept:
`tuple` (points/vectors), `color`, `canvas` (PPM/image output), `matrix`,
`transformation` (`Transform` trait), `ray`, `shape`, `intersection`,
`material`, `patterns`, `lighting`, `lights`, `world` (+ `WorldBuilder`),
`camera`, `obj_file` (Wavefront OBJ model import; sample models in
`object_files/`), `scene_file` (TOML scene/animation descriptions used by
the `render` and `animate` binaries in `src/bin/`).

Shapes are a single `Shape` struct with a private `ShapeType` enum
(`src/shape.rs`), dispatched by `match` — not a trait. Adding a shape means:
new enum variant, `default_<shape>()` constructor, and match arms in
`local_intersect` / `normal_at`.

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
