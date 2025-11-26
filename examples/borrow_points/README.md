# Borrow Points

This example demonstrates how to borrow points instead of cloning them when creating plot lines. It shows how to use `PlotPoints::Borrowed` to avoid unnecessary allocations, which is useful for performance-critical applications or when you want to reuse the same data across multiple frames without copying.

## Running

Native
```sh
cargo run -p borrow_points
```

Web (WASM)
```sh
cd examples/borrow_points
trunk serve
```

![](screenshot.png)

