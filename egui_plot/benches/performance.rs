use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use egui::RawInput;
use performance::PerformanceDemo;

fn performance_demo(c: &mut Criterion) {
    let ctx = egui::Context::default();
    let demo = PerformanceDemo::default();

    c.bench_function("random_walks_with_tessellate__realistic", |b| {
        b.iter(|| {
            let mut full_output = ctx.run_ui(RawInput::default(), |ui| {
                demo.show_plot(ui);
            });
            ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

            full_output.textures_delta.clear();
        });
    });

    c.bench_function("random_walks_no_tessellate", |b| {
        b.iter(|| {
            let output = ctx.run_ui(RawInput::default(), |ui| {
                demo.show_plot(ui);
            });
            output.drop_without_applying_deltas();
        });
    });

    let full_output = ctx.run_ui(RawInput::default(), |ui| {
        demo.show_plot(ui);
    });
    c.bench_function("random_walks_only_tessellate", |b| {
        b.iter(|| ctx.tessellate(full_output.shapes.clone(), full_output.pixels_per_point));
    });
    full_output.drop_without_applying_deltas();
}

criterion_group!(benches, performance_demo);
criterion_main!(benches);
