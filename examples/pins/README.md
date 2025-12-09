# Pins Example

This example demonstrates the **pins-only** feature for marking and comparing values at specific X positions.

## Features

- **Pre-existing pins**: The demo starts with 3 pins already placed at x = 1.0, 3.0, and 5.0
- **Pin markers**: Visual markers on each series at pinned locations
- **Pin rails**: Vertical lines marking each pin position
- **Pins panel**: A floating panel showing all pinned values

## Hotkeys

- **P**: Pin the current crosshair position
- **U**: Remove the last pin
- **Delete**: Clear all pins

## Usage

```rust
Plot::new("my_plot").show(ui, |plot_ui| {
    plot_ui.line(Line::new("series", data));

    // Pins only (no tooltip)
    plot_ui.show_pins(&PinOptions::default());
});
```

## See Also

- `default_tooltip` - Tooltip-only example
- `pins_with_tooltip` - Combined pins + tooltip example
