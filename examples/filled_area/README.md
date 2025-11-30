# Filled Area Example

This example demonstrates the `FilledArea` plot item which fills the area between two lines.

## Features

- Plots a sine wave with an adjustable confidence band
- Interactive controls to adjust upper and lower bounds
- Shows how to visualize uncertainty and ranges

## Usage

The example shows `sin(x)` with adjustable bounds:
- **δ lower**: offset for the lower boundary (sin(x) - δ_lower)
- **δ upper**: offset for the upper boundary (sin(x) + δ_upper)
- **points**: number of sampling points

## Running

```bash
cargo run -p filled_area
```
