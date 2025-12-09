# Pins + Tooltip Combined Example

This example demonstrates using **both pins and tooltip** as separate, composable components.

## Key Concept

The hits are collected once and shared between both components:

```rust
// Collect hits once
let hits = plot_ui.collect_hits(50.0);

// Use with both components
plot_ui.show_pins_with_hits(&PinOptions::default(), &hits);
plot_ui.show_tooltip_with_hits(&TooltipOptions::default(), &hits);
```

## Components

- **Pins**: Handles P/U/Delete keys, draws pin rails and markers, shows pins panel
- **Tooltip**: Draws band fill, vertical guide, markers, and tooltip popup

## Hotkeys

- **P**: Pin the current crosshair position
- **U**: Remove the last pin
- **Delete**: Clear all pins

## Screenshot

![Pins + Tooltip Example](screenshot.png)

