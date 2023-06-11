use egui_extras::TableBuilder;
use octorus::{ordatabase::ORDatabase, ormysql::ORMySql};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1320.0, 900.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Primeiro egui APP",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Debug)]
enum Ordb {
    ORMySql(octorus::ormysql::ORMySql),
}

struct MyApp {
    dbms_names: Vec<String>,
    host: String,
    user: String,
    password: String,
    database: String,
    ordb: Option<Ordb>,
}

impl MyApp {
    fn new(
        dbms_names: Vec<String>,
        host: String,
        user: String,
        password: String,
        database: String,
        ordb: Option<Ordb>,
    ) -> Self {
        Self {
            dbms_names,
            host,
            user,
            password,
            database,
            ordb,
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self::new(
            octorus::get_supported_dbms(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            None,
        )
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::left("ordbm_conector")
                .resizable(true)
                .default_width(320.0)
                .width_range(150.0..=320.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Connect to DBMS");
                    });
                    let mut selected_dbms = 0;
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.dbms_names[selected_dbms]))
                        .show_index(ui, &mut selected_dbms, self.dbms_names.len(), |i| {
                            &self.dbms_names[i]
                        });
                    ui.horizontal(|ui| {
                        let name_label = ui.label("Host: ");
                        ui.text_edit_singleline(&mut self.host)
                            .labelled_by(name_label.id);
                    });
                    ui.horizontal(|ui| {
                        let name_label = ui.label("User: ");
                        ui.text_edit_singleline(&mut self.user)
                            .labelled_by(name_label.id);
                    });
                    ui.horizontal(|ui| {
                        let name_label = ui.label("Password: ");
                        ui.text_edit_singleline(&mut self.password)
                            .labelled_by(name_label.id);
                    });
                    ui.horizontal(|ui| {
                        let name_label = ui.label("Database: ");
                        ui.text_edit_singleline(&mut self.database)
                            .labelled_by(name_label.id);
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Conectar").clicked() {
                            let ordb = octorus::ormysql::ORMySql::new(
                                &self.host,
                                &self.user,
                                &self.password,
                                &self.database,
                            );

                            self.ordb = Some(Ordb::ORMySql(ordb.unwrap()));
                        }
                        let status = if self.ordb.is_some() {
                            "Conectado!"
                        } else {
                            "Nao foi"
                        };
                        ui.label(format!("{}", status));
                    });
                });
            let mut query = String::new();
            ui.text_edit_multiline(&mut query);
            let mut table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center));
            if ui.button("Execute Query").clicked() {
                match &self.ordb {
                    Some(Ordb::ORMySql(db)) => {
                        let current_db: &ORMySql = db.clone();
                        let orresult = *current_db.send_query(&query);
                    }
                    None => println!("Erro generico"),
                }
            }
        });
    }
}
