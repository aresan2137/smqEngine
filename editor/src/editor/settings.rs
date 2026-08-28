
pub struct Settings {
    selected_setting: String
}

impl Default for Settings {
    fn default() -> Self {
        Self { 
            selected_setting: "".to_string() 
        }
    }
}

impl Settings {
    pub fn egui(&mut self, ui: &mut egui::Ui) {
        let height = ui.available_height();
                    
        ui.horizontal_top(|ui| {
            ui.push_id(0, |ui| {
                ui.set_width(100.0);
                ui.set_height(height);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        egui::CollapsingHeader::new("test1")
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.selectable_value(&mut self.selected_setting, "t1sub1".to_string(), "t1sub1");
                                ui.selectable_value(&mut self.selected_setting, "t1sub2".to_string(), "t1sub2");
                            });

                        egui::CollapsingHeader::new("test2")
                            .show(ui, |ui| {
                                ui.selectable_value(&mut self.selected_setting, "t2sub1".to_string(), "t2sub1");
                                ui.selectable_value(&mut self.selected_setting, "t2sub2".to_string(), "t2sub2");
                            });
                    });
                });
            });                    

            ui.separator();

            ui.push_id(1, |ui| {
                ui.set_height(height);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.selected_setting.as_str() {
                        "t1sub1" => {
                            ui.heading("t1sub1");
                        }
                        "t1sub2" => {
                            ui.heading("t1sub2");
                        }
                        "t2sub1" => {
                            ui.heading("t2sub1");
                        }
                        "t2sub2" => {
                            ui.heading("t2sub2");
                        }
                        _ => {
                            ui.heading("select a setting");
                        }
                    }
                });
            });                   
        });
    }
}
