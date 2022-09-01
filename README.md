# **msdfont**

[![Build Status](https://img.shields.io/github/workflow/status/Blatko1/msdfont-rs/Rust?logo=github)](https://github.com/Blatko1/msdfont-rs/actions)
[![License](https://img.shields.io/github/license/Blatko1/msdfont-rs?color=%23537aed)](https://github.com/Blatko1/msdfont-rs/blob/master/LICENSE)

> **WIP** - Developer currently suffering because glyphs with overlapping contours exist. (but not for long!)

**Multi-channel signed distance field** (`M`[`SDF`](https://prideout.net/blog/distance_fields/)) generator for fonts implemented in pure Rust.

This crate will soon become a public library. Later **MSDF** and **MTSDF** will be implemented.

This project wouldn't exist without *Chlumsky's* **[`msdfgen`](https://github.com/Chlumsky/msdfgen)** and other Rust libraries which implement his algorithm.

## TODO :memo:

* [x] Add a tool for creating custom shapes
* [ ] Fix: Simple *Overlapping Contours* Correction - ***WIP***
*  Add a proper example - ***WIP***
* [ ] Improve *Overlapping Contours* Correction to *perfection*
* [ ] Add a function for checking intersections for quadratic and cubic functions
* [ ] Add support for **Cubic Bézier Curves**
* [ ] Implement MSDF generation for fonts
* [ ] Implement MTSDF generation for fonts
* [ ] Add more showcase items
* **rest of the TODOs is in code...**
* [ ] Better organization of code
* [ ] Construct a completely user-friendly library

## The Algorithm :desktop_computer:

To learn more about the algorithm, read [`Algorithm`](docs/algorithm.md).

## Output Examples

### SDF

![Signed Distance Field of '#' character](examples/out/%23_char_SDF.png)
![Signed Distance Field of 'A' character](examples/out/A_char_SDF.png)
![Signed Distance Field of 'K' character](examples/out/K_char_SDF.png)
![Signed Distance Field of 'M' character](examples/out/M_char_SDF.png)

### pseudo-SDF

![Pseudo Signed Distance Field of '#' character](examples/out/%23_char_pseudo.png)
![Pseudo Signed Distance Field of 'A' character](examples/out/A_char_pseudo.png)
![Pseudo Signed Distance Field of 'K' character](examples/out/K_char_pseudo.png)
![Pseudo Signed Distance Field of 'M' character](examples/out/M_char_pseudo.png)
