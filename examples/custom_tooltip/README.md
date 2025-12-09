# Custom Tooltip Example

This example demonstrates custom tooltip UI with the tooltip feature.

## Features

- Custom tooltip UI using `show_tooltip_custom()`
- Support for series with different point counts (mismatched x-sampling)
- Visual markers show the nearest points per series

## Usage

```rust
Plot::new("my_plot").show(ui, |plot_ui| {
    plot_ui.line(Line::new("series1", data1));
    plot_ui.line(Line::new("series2", data2));

    plot_ui.show_tooltip_custom(
        &TooltipOptions::default(),
        |ui, hits| {
            // Custom tooltip rendering
            ui.strong("My custom tooltip");
            for h in hits {
                ui.label(format!("{}: ({:.3}, {:.3})", h.series_name, h.value.x, h.value.y));
            }
        },
    );
});
```

## See Also

- `default_tooltip` - Default tooltip UI
- `pins_with_tooltip` - Combined pins + tooltip
