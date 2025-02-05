use demo::TemplateApp;
use egui::accesskit::Role;
use egui::ThemePreference;
use egui_kittest::kittest::Queryable;

#[test]
fn test_demos() {
    let mut harness = egui_kittest::Harness::new_eframe(|cc| TemplateApp::new(cc));

    let demo_names: Vec<_> = harness
        .get_by_role_and_label(Role::RadioGroup, "Select Demo")
        .get_all_by_role(Role::Button)
        .filter_map(|w| w.label())
        .collect();

    let mut errors = Vec::new();

    for name in demo_names {
        harness.get_by_label(&name).click();
        harness.run();

        if let Err(error) = harness.try_snapshot(&format!("demos/{name}")) {
            errors.push(error);
        }
    }

    assert!(errors.is_empty(), "Errors: {errors:#?}");
}

#[test]
fn test_scales() {
    let mut errors = Vec::new();
    for scale in [0.5, 1.0, 1.39, 2.0] {
        let mut harness = egui_kittest::HarnessBuilder::default()
            .with_pixels_per_point(scale)
            .build_eframe(|cc| TemplateApp::new(cc));

        harness.run();

        if let Err(error) = harness.try_snapshot(&format!("scale_{scale:.2}")) {
            errors.push(error);
        }
    }

    assert!(errors.is_empty(), "Errors: {errors:#?}");
}

#[test]
fn test_light_mode() {
    let mut harness = egui_kittest::Harness::new_eframe(|cc| TemplateApp::new(cc));

    harness.ctx.set_theme(ThemePreference::Light);
    harness.run();

    harness.snapshot("light_mode");
}
