# **msdfont-rs**

* **WIP**

**Multi-channel signed distance field** (MSDF) generator for fonts implemented in pure Rust.

This crate will become a public library when the *SDF generation* and **pseudo-SDF** become **fully stable and starts working**.
Later **MSDF** and **MTSDF** will be implemented.

This project wouldn't exist without *Chlumskys* **[msdfgen](https://github.com/Chlumsky/msdfgen)** and other Rust libraries which implement his algorithm.

## Features

* SDF, pseudo-SDF generation for fonts

## Output Examples

### SDF

![Signed Distance Field of '#' character](https://github.com/Blatko1/msdfont-rs/blob/master/output/%23_char_SDF.png)
![Signed Distance Field of 'A' character](https://github.com/Blatko1/msdfont-rs/blob/master/output/A_char_SDF.png)
![Signed Distance Field of 'K' character](https://github.com/Blatko1/msdfont-rs/blob/master/output/K_char_SDF.png)
![Signed Distance Field of 'M' character](https://github.com/Blatko1/msdfont-rs/blob/master/output/M_char_SDF.png)

### pseudo-SDF

![Pseudo Signed Distance Field of '#' character](https://github.com/Blatko1/msdfont-rs/blob/master/output/%23_char_pseudo.png)
![Pseudo Signed Distance Field of 'A' character](https://github.com/Blatko1/msdfont-rs/blob/master/output/A_char_pseudo.png)
![Pseudo Signed Distance Field of 'K' character](https://github.com/Blatko1/msdfont-rs/blob/master/output/K_char_pseudo.png)
![Pseudo Signed Distance Field of 'M' character](https://github.com/Blatko1/msdfont-rs/blob/master/output/M_char_pseudo.png)
