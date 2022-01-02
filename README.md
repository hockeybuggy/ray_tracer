# Ray Tracer

[![Build and Test Status](https://img.shields.io/github/workflow/status/hockeybuggy/ray_tracer/Build%20and%20Test/master.svg?label=Build%20%26%20Test)](https://github.com/hockeybuggy/ray_tracer/actions?query=workflow%3A%22Build+and+Test%22+branch%3Amaster)
[![Check Formatting Status](https://img.shields.io/github/workflow/status/hockeybuggy/ray_tracer/Check%20Formatting/master.svg?label=Check%20Formatting)](https://github.com/hockeybuggy/ray_tracer/actions?query=workflow%3A%22Check+Formatting%22+branch%3Amaster)

This repository is just me working through the exercises in ["The Ray Tracer
Challenge" by Jamis Buck](https://pragprog.com/book/jbtracer/the-ray-tracer-challenge).

Quite a bit of it is just me trying to get better at Rust.

![My first sphere](images/first_sphere.jpg)


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
           alt="A black and white sphere with a ring pattern"
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

</table>


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

- Add multiple light sources (note for this on 138 of the book).
- Builder interface for shapes.
- Update setting of transformation matrices
- Add a file input method for world description
- Create animated gifs
