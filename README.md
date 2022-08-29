# **msdfont**

[![Build Status](https://img.shields.io/github/workflow/status/Blatko1/msdfont-rs/Rust?logo=github)](https://github.com/Blatko1/msdfont-rs/actions)
[![License](https://img.shields.io/github/license/Blatko1/msdfont-rs?color=%23537aed)](https://github.com/Blatko1/msdfont-rs/blob/master/LICENSE)

* **WIP**

**Multi-channel signed distance field** (MSDF) generator for fonts implemented in pure Rust.

This crate will become a public library when the *SDF generation* and **pseudo-SDF** become **fully stable and starts working**.
Later **MSDF** and **MTSDF** will be implemented.

This project wouldn't exist without *Chlumskys* **[msdfgen](https://github.com/Chlumsky/msdfgen)** and other Rust libraries which implement his algorithm.

## TODO 📝

* [x] Add a tool for creating custom shapes
* [ ] Fix: Simple *Overlapping Contours* Correction - *WIP*
* [ ] Improve *Overlapping Contours* Correction to *perfection* - *WIP*
* [ ] Add a function for checking intersections for quadratic and cubic functions
* [ ] Add support for **Cubic Bézier Curves**
* [ ] Implement MSDF generation for fonts
* [ ] Implement MTSDF generation for fonts
* [ ] Add more showcase items
* [ ] Add a proper example
* **...rest of TODOs is in code...**
* [ ] Better organization of code
* [ ] Construct a complete user-friendly library

## The Algorithm 🖥️

To learn more about the algorithm read [`Algorithm`](./docs/algorithm.md).

## Output Examples

### SDF

![Signed Distance Field of '#' character](https://github.com/Blatko1/msdfont/blob/master/lib/examples/out/%23_char_SDF.png)
![Signed Distance Field of 'A' character](https://github.com/Blatko1/msdfont/blob/master/lib/examples/out/A_char_SDF.png)
![Signed Distance Field of 'K' character](https://github.com/Blatko1/msdfont/blob/master/lib/examples/out/K_char_SDF.png)
![Signed Distance Field of 'M' character](https://github.com/Blatko1/msdfont/blob/master/lib/examples/out/M_char_SDF.png)

### pseudo-SDF

![Pseudo Signed Distance Field of '#' character](https://github.com/Blatko1/msdfont/blob/master/lib/examples/out/%23_char_pseudo.png)
![Pseudo Signed Distance Field of 'A' character](https://github.com/Blatko1/msdfont/blob/master/lib/examples/out/A_char_pseudo.png)
![Pseudo Signed Distance Field of 'K' character](https://github.com/Blatko1/msdfont/blob/master/lib/examples/out/K_char_pseudo.png)
![Pseudo Signed Distance Field of 'M' character](https://github.com/Blatko1/msdfont/blob/master/lib/examples/out/M_char_pseudo.png)
