# Default Tooltip Example

This example demonstrates the **tooltip-only** feature for comparing values across multiple series.

## Features

- Hover over the plot to see nearest points per series within a vertical band
- Visual markers show the exact points being compared
- Highlighted lines when pointer is near a series
- Band fill and vertical guide line

## Usage

```rust
Plot::new("my_plot").show(ui, |plot_ui| {
    plot_ui.line(Line::new("series1", data1));
    plot_ui.line(Line::new("series2", data2));

    // Tooltip only (no pins)
    plot_ui.show_tooltip(&TooltipOptions::default());
});
```

## See Also

- `pins` - Pins-only example
- `pins_with_tooltip` - Combined pins + tooltip example
