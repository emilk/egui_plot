# Custom Tooltip Example

This example demonstrates custom tooltip UI with the band tooltip feature, showcasing mismatched x-sampling across multiple series.

## Features

- Custom tooltip UI using `show_tooltip_across_series_with()`
- Support for series with different point counts (mismatched x-sampling)
- Visual markers show the nearest points per series
- Press **P** to pin the current selection
- Press **U** to remove the last pin
- Press **Delete** to clear all pins
- Collapsible pin display in the tooltip

## Usage

```rust
Plot::new("my_plot").show(ui, |plot_ui| {
    plot_ui.line(Line::new("series1", data1));
    plot_ui.line(Line::new("series2", data2));

    plot_ui.show_tooltip_across_series_with(
        &TooltipOptions::default(),
        |ui, hits, pins| {
            // Custom tooltip rendering
            ui.strong("My custom tooltip");
            for h in hits {
                ui.label(format!("{}: ({:.3}, {:.3})", h.series_name, h.value.x, h.value.y));
            }
        },
    );
});
```

