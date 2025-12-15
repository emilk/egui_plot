use eframe::egui;
use eframe::egui::Response;
use egui_plot::Legend;
use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;
use polars::prelude::*;

pub struct PolarsExample {
    df: DataFrame,
    x_column: String,
    y_column: String,
    city: String,
}

impl Default for PolarsExample {
    fn default() -> Self {
        let df = df![
            "time"     => &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                           1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0],
            "temp"     => &[10.5, 12.3, 15.1, 14.8, 16.2, 18.5, 20.1, 22.3, 19.8, 17.5, 15.2, 13.8, 12.1, 11.5, 10.9,
                           8.2, 9.5, 11.3, 13.7, 16.1, 19.2, 22.5, 24.8, 23.1, 20.5, 18.3, 15.9, 13.2, 10.8, 9.1],
            "humidity" => &[65.0, 68.5, 72.3, 70.1, 67.8, 65.2, 62.5, 60.8, 63.2, 66.5, 69.8, 73.2, 75.5, 77.8, 80.1,
                           55.3, 58.7, 62.1, 60.5, 57.9, 54.2, 51.8, 49.5, 52.3, 55.8, 59.2, 62.8, 66.5, 70.1, 73.8],
            "city" => &["London", "London", "London", "London", "London", "London", "London", "London", "London", "London", "London", "London", "London", "London", "London",
                       "Paris", "Paris", "Paris", "Paris", "Paris", "Paris", "Paris", "Paris", "Paris", "Paris", "Paris", "Paris", "Paris", "Paris", "Paris"],
        ]
        .unwrap();

        Self {
            df,
            x_column: "time".to_owned(),
            y_column: "temp".to_owned(),
            city: "London".to_owned(),
        }
    }
}

impl PolarsExample {
    /// Get all float column names from the DataFrame.
    fn get_float_columns(&self) -> Vec<String> {
        self.df
            .get_columns()
            .iter()
            .filter(|col| col.dtype().is_float())
            .map(|col| col.name().to_string())
            .collect()
    }

    fn get_cities(&self) -> Vec<String> {
        let mut cities: Vec<String> = self
            .df
            .column("city")
            .ok()
            .and_then(|col| col.str().ok())
            .and_then(|s| s.unique().ok())
            .map(|unique| unique.into_iter().flatten().map(str::to_owned).collect())
            .unwrap_or_default();
        cities.sort();
        cities
    }

    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> Response {
        let numerical_columns = self.get_float_columns();

        ui.horizontal(|ui| {
            egui::ComboBox::new("x_column", "X column")
                .selected_text(&self.x_column)
                .show_ui(ui, |ui| {
                    for col in &numerical_columns {
                        ui.selectable_value(&mut self.x_column, col.clone(), col);
                    }
                });

            ui.separator();

            egui::ComboBox::new("y_column", "Y column")
                .selected_text(&self.y_column)
                .show_ui(ui, |ui| {
                    for col in &numerical_columns {
                        ui.selectable_value(&mut self.y_column, col.clone(), col);
                    }
                });

            ui.separator();
            egui::ComboBox::new("city", "City")
                .selected_text(self.city.as_str())
                .show_ui(ui, |ui| {
                    for city in self.get_cities() {
                        ui.selectable_value(&mut self.city, city.clone(), city);
                    }
                });
        })
        .response
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> Response {
        // Filter DataFrame by selected city using lazy API
        let filtered_df = self
            .df
            .clone()
            .lazy()
            .filter(col("city").eq(lit(self.city.as_str())))
            .collect()
            .unwrap_or_else(|_| self.df.clone());

        // Extract x and y columns as continuous slices
        let xs: Option<&[f64]> = filtered_df
            .column(&self.x_column)
            .ok()
            .and_then(|col| col.f64().ok())
            .and_then(|ca| ca.cont_slice().ok());
        let ys: Option<&[f64]> = filtered_df
            .column(&self.y_column)
            .ok()
            .and_then(|col| col.f64().ok())
            .and_then(|ca| ca.cont_slice().ok());

        // Create plot points from x and y values
        let points: Option<PlotPoints<'_>> = xs
            .zip(ys)
            .map(|(xs, ys)| xs.iter().zip(ys.iter()).map(|(&x, &y)| [x, y]).collect());

        Plot::new("PolarsExample")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                if let Some(points) = points {
                    plot_ui.line(Line::new(&self.y_column, points));
                }
            })
            .response
    }
}
