# Ray Tracer

[![Build Status](https://api.cirrus-ci.com/github/hockeybuggy/ray_tracer.svg)](https://cirrus-ci.com/github/hockeybuggy/ray_tracer)

This repository is just me working through the exercises in ["The Ray Tracer
Challenge" by Jamis Buck](https://pragprog.com/book/jbtracer/the-ray-tracer-challenge).

Quite a bit of it is just me trying to get better at Rust.


## Running

To get started clone the repository then run:

    cargo test


## Possible improvements

- Only have one interface for transformations. Right now I export the
  functions, but it may may sense to just have them as methods on matrices.
- Extract the assert helper macros somewhere other than in the tests. The are
  starting to be duplicated.
- Experiment with different types for point tuples and vector tuples.
