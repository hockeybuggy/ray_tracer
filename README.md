# Ray Tracer

[![Build and Test Status](https://img.shields.io/github/workflow/status/hockeybuggy/ray_tracer/Build%20and%20Test/master.svg?label=Build%20%26%20Test)](https://github.com/hockeybuggy/ray_tracer/actions?query=workflow%3A%22Build+and+Test%22+branch%3Amaster)
[![Check Formatting Status](https://img.shields.io/github/workflow/status/hockeybuggy/ray_tracer/Check%20Formatting/master.svg?label=Check%20Formatting)](https://github.com/hockeybuggy/ray_tracer/actions?query=workflow%3A%22Check+Formatting%22+branch%3Amaster)

This repository is just me working through the exercises in ["The Ray Tracer
Challenge" by Jamis Buck](https://pragprog.com/book/jbtracer/the-ray-tracer-challenge).

Quite a bit of it is just me trying to get better at Rust.

![My first sphere](images/first_sphere.jpg)


## Running

To get started clone the repository then run:

    cargo test


## Possible improvements

- Add multiple light sources.
- Update the test that write files to use tempfiles.
- Use a more compact (and more supported format than PPM).
- Builder interface for worlds.
- Builder interface for shapes.
