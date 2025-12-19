# Custom Plot Manipulation

This example demonstrates how to implement custom plot manipulation controls using raw input events. It shows how to create alternative pan and zoom behaviors, such as inverting the default Ctrl key behavior, customizing zoom and scroll speeds, and locking axes. This is useful for building specialized interaction patterns that differ from the default `egui_plot` controls.

## Running

Native
```sh
cargo run -p custom_plot_manipulation
```

Web (WASM)
```sh
cd examples/custom_plot_manipulation
trunk serve
```

![](screenshot.png)

