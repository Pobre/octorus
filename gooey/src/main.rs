use egui_extras::{Column, Table, TableBuilder};
use octorus::{ordatabase::ORDatabase, orresult::ORResult};

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

struct TableResult {
    orresult: ORResult,
}

struct MyApp {
    dbms_names: Vec<String>,
    host: String,
    user: String,
    password: String,
    database: String,
    query: String,
    ordb: Option<Ordb>,
    table: Option<TableResult>,
}

impl MyApp {
    fn new(
        dbms_names: Vec<String>,
        host: String,
        user: String,
        password: String,
        database: String,
        query: String,
        ordb: Option<Ordb>,
        table: Option<TableResult>,
    ) -> Self {
        Self {
            dbms_names,
            host,
            user,
            password,
            database,
            query,
            ordb,
            table,
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
            String::new(),
            None,
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
            egui::TextEdit::multiline(&mut self.query).show(ui);

            match &self.table {
                Some(r) => {
                    let orresult = &r.orresult;
                    let mut table = TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center));
                    table = table.columns(Column::auto(), orresult.header.len());
                    table = table.scroll_to_row(orresult.result_set.len(), None);
                    table
                        .header(20.0, |mut header| {
                            for h in &orresult.header {
                                header.col(|ui| {
                                    ui.strong(h);
                                });
                            }
                        })
                        .body(|body| {
                            body.rows(20.0, orresult.result_set.len(), |row_index, mut row| {
                                for text in &orresult.result_set[row_index] {
                                    row.col(|ui| {
                                        ui.label(text);
                                    });
                                }
                            });
                        });
                }
                None => (),
            }
            if ui.button("Execute Query").clicked() {
                match self.ordb.as_mut() {
                    Some(Ordb::ORMySql(db)) => {
                        self.table = Some(TableResult {
                            orresult: db.send_query(&mut self.query).unwrap(),
                        });
                    }
                    None => println!("Erro generico"),
                }
            }
        });
    }
}
