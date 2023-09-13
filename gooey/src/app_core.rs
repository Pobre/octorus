pub struct GooCore {}

impl GooCore {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let goo = Self {};
        goo
    }

    fn core_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("core_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.label("Databases").clicked() {
                    println!("Cliquei");
                }
                ui.separator();
                if ui.button("Query Editor").clicked() {}
                ui.separator();
            });
        });
    }

    fn database_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("database_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Databases");
                });

                ui.separator();
                //self.database_panel_contents(ui, frame);
            });
    }
}

impl eframe::App for GooCore {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.core_panel(ctx, frame);
    }
}
