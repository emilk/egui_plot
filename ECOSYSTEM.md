# Plotting libraries in Rust

I searched over published crates on crates.io in November 2025 and found some interesting libraries.
I briefly describe their functionality/main limitations here. 

Note that as far as I've looked, `egui_plot` is the only plotting library that supports plot interactions straight through Rust, without going through Javascript or other binding layers.
Also, `egui_plot` produces a list of painting commands that are sent to a backend renderer, which can be GPU accelerated and easily integrated into other GUIs.
Therefore, `egui_plot` can efficiently render large number of (interactive) plot items.
See [`egui`](https://github.com/emilk/egui) for more info about GPU integration.

## Rust rendering libraries

| Name                                                        | Description                                                                     |
|-------------------------------------------------------------|---------------------------------------------------------------------------------|
| [`plotters`](https://crates.io/crates/plotters)             | Pure Rust, interesting! But interactivity seems to be done via Javascript only. |
| [`graplot`](https://crates.io/crates/graplot)               | Pure Rust, but inactive                                                         |
| [`quill`](https://crates.io/crates/quill)                   | Pure Rust, SVG, basic plots                                                     |
| [`plotlib`](https://crates.io/crates/plotlib)               | Pure Rust, SVG/text, basic features, looks abandoned                            |
| [`rustplotlib`](https://crates.io/crates/rustplotlib)       | Pure Rust, inactive                                                             |
| [`criterion-plot`](https://crates.io/crates/criterion-plot) | not maintained                                                                  |
| [`runmat-plot`](https://crates.io/crates/runmat-plot)       | Uses matplotlib-like DSL                                                        |
| [`cgrustplot`](https://crates.io/crates/cgrustplot)         | terminal plotting                                                               |
| [`termplot`](https://crates.io/crates/termplot)             | terminal plotting                                                               |
| [`lowcharts`](https://crates.io/crates/lowcharts)           | terminal plotting                                                               |

## Wrappers around other plotting libraries

Following crates wrap other plotting libraries:

| Name                                                      | Description                                                                                                                                                                   |
|-----------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [`dear-imgui-rs`](https://crates.io/crates/dear-imgui-rs) | C/C++ bindings to https://github.com/ocornut/imgui. IMGUI is probably the most interesting library out there, as it is also immediate-mode based like `egui` and `egui_plot`. |
| [`gnuplot`](https://crates.io/crates/gnuplot)             | C/C++ bindings to http://www.gnuplot.info/                                                                                                                                    |
| [`plotly`](https://crates.io/crates/plotly)               | JS wrapper                                                                                                                                                                    |
| [`charming`](https://crates.io/crates/charming)           | JS wrapper                                                                                                                                                                    |
| [`charts-rs`](https://crates.io/crates/charts-rs)         | JS wrapper                                                                                                                                                                    |
| [`plotpy`](https://crates.io/crates/plotpy)               | Python wrapper                                                                                                                                                                |
| [`poloto`](https://crates.io/crates/poloto)               | SVG, no interaction                                                                                                                                                           |
