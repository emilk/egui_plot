use eframe::egui;

/// Trait for examples that can be displayed in the demo gallery.
pub trait PlotExample {
    /// The name of the example. Should match directory name.
    fn name(&self) -> &'static str;

    /// The title of the example.
    fn title(&self) -> &'static str;

    /// The description of the example.
    fn description(&self) -> &'static str;

    /// The tags of the example.
    fn tags(&self) -> &'static [&'static str];

    /// The thumbnail image of the example.
    /// Should be 192x192 pixels. It is automatically generated from the
    /// screenshot of the example.
    fn thumbnail_bytes(&self) -> &'static [u8];

    /// The code of the example.
    fn code_bytes(&self) -> &'static [u8];

    /// The UI of the example.
    fn show_ui(&mut self, ui: &mut egui::Ui) -> egui::Response;

    /// The controls for the example.
    fn show_controls(&mut self, ui: &mut egui::Ui) -> egui::Response;
}

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
pub mod internal {
    use std::path::PathBuf;

    use egui_kittest::Harness;
    use egui_kittest::SnapshotOptions;

    pub fn run_screenshot_test<State>(builder: impl Fn(&mut eframe::CreationContext<'_>) -> State, manifest_dir: &str)
    where
        State: eframe::App,
    {
        let output_path = PathBuf::from(manifest_dir);
        let options = SnapshotOptions::new().threshold(2.0).output_path(output_path);

        // Generate main screenshot
        let mut harness = Harness::builder()
            .with_size(egui::Vec2::new(800.0, 800.0))
            .build_eframe(&builder);
        harness.run();
        harness.snapshot_options("screenshot", &options);

        // Generate thumbnail
        let mut thumb_harness = Harness::builder()
            .with_size(egui::Vec2::new(192.0, 192.0))
            .build_eframe(&builder);
        thumb_harness.run();
        let _ = thumb_harness.try_snapshot_options("screenshot_thumb", &options);
    }
}

/// Macro to generate a simple native `main` function for an `eframe` example
/// and a corresponding screenshot test. Intended to be used for [`PlotExample`]
/// implementations.
///
/// # Example
///
/// ```no_run,ignore
/// use examples_utils::make_main;
/// use my_example::MyExample;
///
/// make_main!(MyExample);
/// ```
#[macro_export]
macro_rules! make_main {
    ($inner:ident) => {
        use eframe::egui;

        // Generate wrapper struct
        #[derive(Default)]
        pub struct AppWrapper {
            pub inner: $inner,
            pub plot_only: bool,
        }

        impl eframe::App for AppWrapper {
            fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if self.plot_only {
                        self.inner.show_plot(ui);
                    } else {
                        ui.vertical(|ui| {
                            self.inner.show_controls(ui);
                            ui.separator();
                            self.inner.show_plot(ui);
                        });
                    }
                });
            }
        }

        /// Native entry-point for the example.
        fn main() -> eframe::Result {
            use $crate::PlotExample as _;

            env_logger::init();

            // Derive the application title from the `PlotExample` implementation.
            let app_name: &'static str = <$inner as $crate::PlotExample>::title(&<$inner as Default>::default());

            let options = eframe::NativeOptions::default();
            eframe::run_native(
                app_name,
                options,
                Box::new(|_cc| Ok(Box::new(AppWrapper::default()))),
            )
        }

        /// Screenshot tests for the example.
        ///
        /// This uses `egui_kittest` under the hood and is only compiled for
        /// non-WASM targets.
        #[cfg(all(test, not(target_arch = "wasm32")))]
        mod screenshot_tests {
            use super::AppWrapper;

            #[allow(non_snake_case)]
            #[test]
            fn $inner() {
                ::examples_utils::internal::run_screenshot_test(
                    |_cc| AppWrapper {
                        plot_only: true,
                        ..Default::default()
                    },
                    env!("CARGO_MANIFEST_DIR"),
                );
            }
        }
    };
}
