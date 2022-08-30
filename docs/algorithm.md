# Signed distance generator :gear:

In this `README`, I will explain how the algorithm for generating *signed distances* works and propose possible improvements.

One of the project's most complex sub-algorithms is the *correction of contour overlaps*. It performs for each texture's texel, which heavily affects the performance. I'll specifically focus on this part in the explanation because it is very error-prone.

## **The Algorithm**

First, it is essential to note that this algorithm is divided into five subparts:

- **Parse shape's instructions**
- **Check for intersections and store intersection data**
- **Generate the distance fields**
- ***Overlap Correction***
- **Convert distance fields to image data**

## `Parsing shape's instructions` && `Checking for intersections and storing intersection data`

The glyph shape instructions are parsed and stored in memory for later use in the first part of the algorithm. *Glyph's shape* is constructed from **multiple segments** (*lines*, *quadratic Bezier curves*, *cubic Bezier curves*), which are grouped into closed **contours**.

After the parsing, tests for finding intersections are run, and the results are stored. In particular, for the shape, each contour is tested against another contour to find intersection points.

Instructions for glyph's shape are:

- **start_at** - opens a new contour at the specified starting point

- **line_to** - draws a line from the last point to a specified end point

- **quad_to** - draws a *Quadratic Bézier Curve* from the last point to a specified end point with one control point

- **curve_to** - draws a *Cubic Bézier Curve* from the last point to a specified end point with two control points (:warning:**Warning**: *not yet supported*)

- **close** - indicates that there are no further instructions for the current contour, and a new one can be opened

<img title="" src="https://upload.wikimedia.org/wikipedia/commons/9/99/Bezier_grad123.svg" alt="Line, Quadratic Bézier Curve and Cubic Bézier Curve" width="428" data-align="inline">

In the *code*, there is the `Shape` object, the `Contour` object and the `Segment` object, which can represent a `line` or a `quadratic Bezier curve` or a `cubic Bezier curve`.

```mermaid
flowchart LR
    A[Shape] ==>|consists of multiple| B(Contour)
    B ==>|made of| C{Segments}
    C ==>|is a single| D[Line]
    C ==>|is a single| E[Quadratic Bézier Curve]
    C ==>|is a single| F[Cubic Bézier Curve]
```

### Object data

Now after clarifying from what components a shape is made of, the following is a clearer view of data which every object holds. Simply said, *Shape* holds *Contour data* which holds *Segment data*.

```mermaid
flowchart TB
    S((Shape))
    I[Intersection data]
    ID[Contour ID]
    W[Winding]
    B{{line or\n quadratic Bezier curve or\n cubic Bezier curve}}
    S --- I
    S ==> Contour
    subgraph Contour
        direction LR
        ID
        W
        Segment
        subgraph Segment
            direction LR
            B
            B --- P[Position Data]
        end
    end
```

- **Intersection data** - contains all intersection points between contours with *Contour ID* being stored instead; has stored contours which surround other contours with the same winding

- **Winding** - represents a *boolean* value where `true` value means that the contour is additive (clockwise) and `false` value means that the contour is subtractive (counterclockwise)

- **Contour ID** - a unique ID number which is used in intersections

- **Position Data** - position coordinates of a line or a quadratic Bezier curve or a cubic Bezier curve

### 

### Intersection testing

To find intersections, each contour's segment is tested against the other contour's segment and the result is stored. 

For example, finding intersection points between two lines (*linear functions*) is easily achieved by equalization of their functions.

Linear function example:

$$
f(t) = P_0 + t(P_1 - P_0)
$$

So the equalization of functions, for lines $L_1(t_1) = A + t_1(B - A)$ and $L_2(t_2) = C + t_2(D - C)$, would look like:

$$
A_{xy} + t_1(B_{xy} - A_{xy}) = C_{xy} + t_2(D_{xy} - C_{xy})
$$

or:

$$
A_x + t_1 (B_x - A_x) = C_x + t_2 (D_x - C_x)
$$

$$
A_y + t_1 (B_y - A_y) = C_y + t_2 (D_y - C_y)
$$

Finally, we'd easily extract $t_1$ and $t_2$ from one function, include them in the other, and get the correct expressions for each.

Including $t_1$ in the equation for the first line returns the intersection point. Since these lines go from $P_0$ to $P_1$, variable $t$ has a condition $t \in [0, 1]$, so $t_1$ and $t_2$ should be checked if they exceed the bounds. If they don't, then the lines intersect.

> :warning::warning::warning: **IMPORTANT:** For testing quadratic and cubic curves against each other , there doesn't exist any numerical solution (as far as I know), so the testing would implement the logic of creating *sub curves* and testing their covered area. 
> 
> Since cubic curves aren't supported yet, the only problem, for now, is quadratic - quadratic curve intersection.

---