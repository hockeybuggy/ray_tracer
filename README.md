# Ray Tracer

[![Build and Test Status](https://img.shields.io/github/workflow/status/hockeybuggy/ray_tracer/Build%20and%20Test/master.svg?label=Build%20%26%20Test)](https://github.com/hockeybuggy/ray_tracer/actions?query=workflow%3A%22Build+and+Test%22+branch%3Amaster)
[![Check Formatting Status](https://img.shields.io/github/workflow/status/hockeybuggy/ray_tracer/Check%20Formatting/master.svg?label=Check%20Formatting)](https://github.com/hockeybuggy/ray_tracer/actions?query=workflow%3A%22Check+Formatting%22+branch%3Amaster)

This repository is just me working through the exercises in ["The Ray Tracer
Challenge" by Jamis Buck](https://pragprog.com/book/jbtracer/the-ray-tracer-challenge).

Quite a bit of it is just me trying to get better at Rust.

![The book's cover image rendered by this ray tracer](images/cover.png)


## Getting started

Start off by cloning the repo. You'll also need a version of Rust and Cargo
(consider using `rustup` to install this).


## Running tests

To run the unit tests and the end to end tests.

```sh
cargo test
```

If one of the end to end tests produces a different result from the reference
image in the `tests/fixtures` directory the result will be written into the
root of the project directory. The mechanism can be use for rendering higher
resolution version of the scenes in the tests by changing the `SCALE` constant
at the top of the test then running the test.


## Gallery

These are some of the scenes the ray tracer can render. The size of these
images are small because these images are used as fixtures that tests use to
compare to the results they generate and smaller images are faster to render.

### The basics

<table>
  <tr>
    <th>
      <p>Name</p>
      <img width="300" height="1" />
    </th>
    <th>
      <p>Image</p>
    </th>
  <tr>
  <tr>
    <td>Simple Circle</td>
    <td>
      <img src="tests/fixtures/simple_circle_test.png"
           alt="A black and white sphere with a checkered pattern"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Simple Sphere</td>
    <td>
      <img src="tests/fixtures/simple_sphere.png"
           alt="A pink sphere on a black background. The sphere is shiny and fades to dark."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Translated Sphere</td>
    <td>
      <img src="tests/fixtures/translated_sphere.png"
           alt="A pink sphere with a translated pattern"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Simple Cube</td>
    <td>
      <img src="tests/fixtures/simple_cube.png"
           alt="A blue cube rotated so three of its faces are visible"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Simple Cylinder</td>
    <td>
      <img src="tests/fixtures/simple_cylinder.png"
           alt="A capped teal cylinder seen from slightly above"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Open Cylinder</td>
    <td>
      <img src="tests/fixtures/open_cylinder.png"
           alt="An uncapped purple cylinder seen from above, showing its hollow inside"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Simple Cone</td>
    <td>
      <img src="tests/fixtures/simple_cone.png"
           alt="An orange cone with its wide base at the bottom and its tip pointing up"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Open Cone</td>
    <td>
      <img src="tests/fixtures/open_cone.png"
           alt="An uncapped red cone opening upward like a funnel, showing its hollow inside"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Simple Plane</td>
    <td>
      <img src="tests/fixtures/simple_plane.png"
           alt="A blue plane stretching to the horizon under a black sky"
           width="100px"
           height="100px"
           >
    </td>
  </tr>
</table>


### Patterns

<table>
  <tr>
    <th>
      <p>Name</p>
      <img width="300" height="1" />
    </th>
    <th>
      <p>Image</p>
    </th>
  </tr>

  <tr>
    <td>Checkered Sphere</td>
    <td>
      <img src="tests/fixtures/checkered_sphere.png"
           alt="A black and white sphere with a checkered pattern"
           width="100px"
           height="50px"
           >
    </td>
  </tr>

  <tr>
    <td>Gradient Sphere</td>
    <td>
      <img src="tests/fixtures/gradient_sphere.png"
           alt="A black and white sphere with a gradient pattern"
           width="100px"
           height="50px"
           >
    </td>
  </tr>

  <tr>
    <td>Ring Sphere</td>
    <td>
      <img src="tests/fixtures/ring_sphere.png"
           alt="Two spheres with black and white bullseye ring patterns"
           width="100px"
           height="50px"
           >
    </td>
  </tr>

  <tr>
    <td>Ring Cylinder</td>
    <td>
      <img src="tests/fixtures/ring_cylinder.png"
           alt="Two cylinders with ring patterns, one with rings arcing around its barrel and one with a bullseye on its top cap"
           width="100px"
           height="50px"
           >
    </td>
  </tr>

  <tr>
    <td>Stripe Sphere</td>
    <td>
      <img src="tests/fixtures/stripe_sphere.png"
           alt="A black and white sphere with a stripe pattern"
           width="100px"
           height="50px"
           >
    </td>
  </tr>

  <tr>
    <td>Checkered Cube</td>
    <td>
      <img src="tests/fixtures/checkered_cube.png"
           alt="Two black and white cubes with checkered patterns of different sizes"
           width="100px"
           height="50px"
           >
    </td>
  </tr>

  <tr>
    <td>Stripe Cube</td>
    <td>
      <img src="tests/fixtures/stripe_cube.png"
           alt="Two black and white cubes, one with vertical stripes and one with horizontal stripes"
           width="100px"
           height="50px"
           >
    </td>
  </tr>
</table>

### Texture mapping

<table>
  <tr>
    <th>
      <p>Name</p>
      <img width="300" height="1" />
    </th>
    <th>
      <p>Image</p>
    </th>
  </tr>

  <tr>
    <td>UV Checkered Sphere</td>
    <td>
      <img src="tests/fixtures/uv_checkered_sphere.png"
           alt="A sphere with a green and white checkerboard texture mapped onto its surface"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>UV Checkered Plane</td>
    <td>
      <img src="tests/fixtures/uv_checkered_plane.png"
           alt="A green and white checkerboard plane stretching to the horizon"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>UV Checkered Cylinder</td>
    <td>
      <img src="tests/fixtures/uv_checkered_cylinder.png"
           alt="A cylinder wrapped in a green and white checkerboard texture"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Align Check Plane</td>
    <td>
      <img src="tests/fixtures/align_check_plane.png"
           alt="A plane tiled with white squares that have colored corner markers"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>UV Mapped Cube</td>
    <td>
      <img src="tests/fixtures/uv_mapped_cube.png"
           alt="Eight rotated cubes with align-check textures showing seamless corners"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Earth</td>
    <td>
      <img src="tests/fixtures/earth.png"
           alt="An Earth-textured globe on a pedestal above a reflective floor"
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Skybox</td>
    <td>
      <img src="tests/fixtures/skybox.png"
           alt="A reflective sphere floating inside a photographic chapel skybox"
           width="100px"
           height="100px"
           >
    </td>
  </tr>
</table>

### Worlds

<table>
  <tr>
    <th>
      <p>Name</p>
      <img width="300" height="1" />
    </th>
    <th>
      <p>Image</p>
    </th>
  </tr>

  <tr>
    <td>World floor</td>
    <td>
      <img src="tests/fixtures/world_with_plane.png"
           alt="A scene of three spheres on a beige plane. The spheres are various sizes and colours."
           width="100px"
           height="50px"
           >
    </td>
  </tr>

  <tr>
    <td>Simple World</td>
    <td>
      <img src="tests/fixtures/simple_world.png"
           alt="A scene of three spheres in a beige corner of what could be a room. The spheres are various sizes and colours."
           width="100px"
           height="50px"
           >
    </td>
  </tr>


  <tr>
    <td>World with non reflective checkered floor</td>
    <td>
      <img src="tests/fixtures/world_with_non_reflective_checkered_floor.png"
           alt="A world with green sphere sitting on a matte checkered floor."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>World with reflective checkered floor</td>
    <td>
      <img src="tests/fixtures/world_with_reflective_checkered_floor.png"
           alt="A world with green sphere sitting on a reflective checkered floor."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>World with reflective floor</td>
    <td>
      <img src="tests/fixtures/reflection.png"
           alt="A world with three spheres of different colours sitting on a reflective floor."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>World with reflective floor and spheres</td>
    <td>
      <img src="tests/fixtures/very_reflection.png"
           alt="A world with three reflective spheres of different colours sitting on a reflective floor."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>World glass sphere</td>
    <td>
      <img src="tests/fixtures/glass_sphere.png"
           alt="A world with a glass spheres sitting on a patterned floor."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>World made of cubes</td>
    <td>
      <img src="tests/fixtures/cubes.png"
           alt="A room containing a table made of cubes, with boxes on the table and floor."
           width="150px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Cylinders and cones</td>
    <td>
      <img src="tests/fixtures/cylinders_and_cones.png"
           alt="A capped cylinder, a hollow pipe, and an ice cream cone with a strawberry scoop."
           width="150px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Multiple lights</td>
    <td>
      <img src="tests/fixtures/multiple_lights.png"
           alt="A white sphere lit by a warm light and a cool light, casting two tinted shadows."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Hexagon</td>
    <td>
      <img src="tests/fixtures/hexagon.png"
           alt="A teal hexagonal ring built from grouped spheres and cylinders, tipped forward above a white floor."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Teapot</td>
    <td>
      <img src="tests/fixtures/teapot.png"
           alt="A smooth shaded white teapot, imported from a low resolution OBJ model, casting a shadow on a gray floor."
           width="100px"
           height="75px"
           >
    </td>
  </tr>

  <tr>
    <td>Teapot (high resolution)</td>
    <td>
      <img src="tests/fixtures/teapot_high.png"
           alt="The same white teapot imported from a high resolution OBJ model, with a cleaner silhouette and lid."
           width="100px"
           height="75px"
           >
    </td>
  </tr>

  <tr>
    <td>Cow</td>
    <td>
      <img src="tests/fixtures/cow.png"
           alt="A faceted white cow imported from an OBJ model without vertex normals, standing on a gray floor."
           width="100px"
           height="75px"
           >
    </td>
  </tr>

  <tr>
    <td>Teddy</td>
    <td>
      <img src="tests/fixtures/teddy.png"
           alt="A faceted white teddy bear imported from an OBJ model without vertex normals, facing the camera."
           width="100px"
           height="75px"
           >
    </td>
  </tr>

  <tr>
    <td>Constructive solid geometry</td>
    <td>
      <img src="tests/fixtures/csg_shapes.png"
           alt="A two-tone lens made from intersecting spheres, a yellow cube with red spherical hollows carved from its faces, and a ringed Saturn."
           width="150px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Dice</td>
    <td>
      <img src="tests/fixtures/dice.png"
           alt="An ivory die and a red die resting on green felt, each carved from CSG operations: rounded cubes with dished pips."
           width="150px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Area light (soft shadows)</td>
    <td>
      <img src="tests/fixtures/area_light.png"
           alt="A white sphere lit by a rectangular area light, casting a soft shadow with a penumbra instead of a sharp edge."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>The book cover</td>
    <td>
      <img src="tests/fixtures/cover.png"
           alt="The cover image of The Ray Tracer Challenge: a glassy dark sphere resting in a tilted cascade of red, blue, and white cubes."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Spotlight</td>
    <td>
      <img src="tests/fixtures/spotlight.png"
           alt="Three spheres in the dark under a spotlight aimed at the middle one, its beam pooling on the floor and fading at the edges."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Spotlight (narrow beam)</td>
    <td>
      <img src="tests/fixtures/spotlight_narrow.png"
           alt="The spotlight scene with a tight, hard-edged beam lighting little more than the top of the middle sphere."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Spotlight (wide beam)</td>
    <td>
      <img src="tests/fixtures/spotlight_wide.png"
           alt="The spotlight scene with a wide, soft beam lighting all three spheres, the floor darkening toward the corners."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

  <tr>
    <td>Spotlight (side lit)</td>
    <td>
      <img src="tests/fixtures/spotlight_side.png"
           alt="The spotlight scene lit at an angle from the left, the beam stretched into an ellipse with shadows thrown right."
           width="100px"
           height="100px"
           >
    </td>
  </tr>

</table>


## Rendering scenes

Scenes can also be described in TOML files instead of Rust (see
`scenes/*.toml` for examples). The `render` binary reads one and writes a
PNG:

```sh
cargo run --release --bin render -- scenes/<scene>.toml
```

`--scale N` multiplies the scene's base resolution, so the same file can
render small while iterating and large for a shareable image. `--output
PATH` picks the output path (the default is `<name>.png`).

A scene file has a `[scene]` table naming the render and giving its base
size, a `[camera]`, and any number of `[[lights]]` and `[[objects]]`:

```toml
[scene]
name = "example"
width = 100
height = 100

[camera]
field_of_view = 60.0     # degrees
from = [0.0, 2.5, -7.0]  # where the camera sits
to = [0.0, 1.0, 0.0]     # what it looks at
up = [0.0, 1.0, 0.0]

[[lights]]
position = [-8.0, 6.0, -6.0]
intensity = [1.0, 1.0, 1.0]  # rgb

# Objects have a unique name; a kind (`plane`, `sphere`, `cube`, or `obj`
# with a `file` path to an OBJ model); an optional list of transform
# steps; and optional material overrides.
[[objects]]
name = "ball"
kind = "sphere"
transform = [{ scale = [0.5, 0.5, 0.5] }, { translate = [0.0, 1.0, 0.0] }]
material = { color = [0.9, 0.2, 0.2], specular = 0.3 }
```

Transform steps (`rotate_x`/`rotate_y`/`rotate_z` in degrees, `scale`, and
`translate`) apply in list order, each in world space after the ones
before it. Material overrides apply on top of the default material:
`color`, `ambient`, `diffuse`, `specular`, `shininess`, `reflective`,
`transparency`, and `refractive_index`.


## Rendering animations

An animation file (see `animations/*.toml`) uses the same format with an
`[animation]` table in place of `[scene]`, followed by a list of
`[[frames]]`. Each frame restates how it differs from the base scene:
extra transform steps appended after an object's base transform, or
replacement camera coordinates. An empty `[[frames]]` entry renders the
base scene unchanged.

```toml
[[frames]]

[[frames]]
objects.teapot.transform = [{ rotate_y = 10.0 }]

[[frames]]
camera.from = [0.694593, 2.0, -3.939231]
```

The `animate` binary renders every frame, in parallel, to a numbered PNG:

```sh
cargo run --release --bin animate -- animations/<animation>.toml --scale 4
```

and `scripts/make_gif.sh` assembles the frames into a looping gif with
ffmpeg:

```sh
./scripts/make_gif.sh animations/frames/<animation> animations/<animation>.gif
```

For example, these are the frames of `tests/scenes/sphere_moves.toml`: the
first frame is the base scene, the second slides the sphere to the right,
and the third slides it further while pulling the camera up and back.

<img src="tests/fixtures/sphere_moves_0.png"
     alt="A green sphere resting on a white floor."
     width="100px"
     height="75px"
     > <img src="tests/fixtures/sphere_moves_1.png"
     alt="The same sphere slid to the right."
     width="100px"
     height="75px"
     > <img src="tests/fixtures/sphere_moves_2.png"
     alt="The sphere slid further right, seen from a higher, more distant camera."
     width="100px"
     height="75px"
     >

The two animations in `animations/` put the high-poly Utah teapot through
a full turn — first spinning the teapot in place, then orbiting the camera
around it while the light stays fixed:

<img src="images/teapot_spin.gif"
     alt="The Utah teapot spinning in place through a full turn."
     width="400px"
     height="300px"
     > <img src="images/teapot_orbit.gif"
     alt="The camera circling the teapot, which passes in and out of its own shadow side."
     width="400px"
     height="300px"
     >


## Running benchmarks

This project contains benchmarks which measure the performance and allow you to
compare how quickly the ray tracer is able to render scenes.

```sh
cargo bench
```

This will output something like:

```
render simple world     time:   [48.247 ms 48.335 ms 48.427 ms]
                        change: [-1.7688% -1.3561% -0.9619%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
```

This will compare a bench mark to the previous results. This allows you to make
a change that you expect to improve performance and know if it worked on not.


## Possible improvements

- Parallel rendering: the render loop casts each pixel's ray
  independently, so rows could be rendered across threads (e.g. with
  `rayon`) for a near-linear speedup on multi-core machines.
- Soft shadow example with the teapot
- Performance testing of the teapot render
- Higher resolution rendering of the earth scene
- Skybox and texture support for the scene definition DSL
- The dragon scene from the bounding box bonus chapter
- Focal blur
- Anti-aliasing
- Normal perturbations
- Torus primitive
