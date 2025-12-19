# Linked Axes Example

This example demonstrates how to link axes and cursors across multiple plots. When you zoom, pan, or move the cursor in one plot, the linked plots will synchronize their view, useful for comparing data across different visualizations.

## Running

Native
```sh
cargo run -p linked_axes
```

Web (WASM)
```sh
cd examples/linked_axes
trunk serve
```

![](screenshot.png)
