# Default Tooltip Example

This example demonstrates the default band tooltip feature for comparing values across multiple series.

## Features

- Hover over the plot to see nearest points per series within a vertical band
- Visual markers show the exact points being compared
- Press **P** to pin the current selection
- Press **U** to remove the last pin
- Press **Delete** to clear all pins
- Pinned values persist across zoom/pan operations

## Usage

```rust
Plot::new("my_plot").show(ui, |plot_ui| {
    plot_ui.line(Line::new("series1", data1));
    plot_ui.line(Line::new("series2", data2));

    plot_ui.show_tooltip_with_options(&TooltipOptions::default());
});
```

