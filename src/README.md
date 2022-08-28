# Signed distance generator

One of the most complex algorithms in this project is the correction of contour overlaps.
It performs for each texture's texel so it heavily affects the generator's performance.
In this `README` I will explain how does the algorithm work, take notes and propose
some possible improvements.

## The Algorithm

First, it is important to note that this algorithm is divided into 5 subparts:

- **Parse shape's instructions**
- **Check for intersections and store intersection data**
- **Generate the distance fields**
- ***Overlap Correction***
- **Convert distance fields to image data**

- WIP
