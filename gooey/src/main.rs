fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some([1320.0, 1024.0].into()),
        ..Default::default()
    };

    eframe::run_native(
        "Octorus - Gooey",
        options,
        Box::new(|ctx| Box::new(gooey::GooCore::new(ctx))),
    )
}
