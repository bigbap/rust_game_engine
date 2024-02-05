#![allow(dead_code)]

use egui::Vec2;

use crate::{
    wrappers::egui::GUI,
    Registry,
    AppState,
    VersionedIndex,
    schema::{
        SchemaRect,
        IPrefab,
        rect::SchemaRectInstance
    },
};

#[cfg(debug_assertions)]
pub struct SceneEditor {
    gui: GUI,

    active_entity: Option<VersionedIndex>
}

impl SceneEditor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            gui: GUI::new(1.0)?,
            active_entity: None,
        })
    }

    pub fn update(
        &mut self,
        registry: &mut Registry,
        app_state: &AppState
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.gui.begin_frame();
        self.entity_list(registry);
        self.debug(app_state, registry);
        self.gui.end_frame(app_state)
    }

    fn menu(&mut self) {
        self.gui.add_panel_top("Menu", |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        println!("saved from editor");
                    }
                    if ui.button("Quit").clicked() {
                        println!("Quiting from editor");
                    }
                });
            });
        });
    }

    fn entity_list(&mut self, registry: &mut Registry) {
        let entities = registry.get_valid_entities();

        self.gui.add_window("Entities", |ui| {
            ui.set_width(200.0);
            ui.separator();

            if ui.button("create entity").clicked() {
                let schema = SchemaRect::default();

                if let Err(e) = schema.build_instance(registry, &SchemaRectInstance::default()) {
                    println!("could not add entity: {}", e);
                }
            }

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for entity in entities.iter() {
                    ui.horizontal(|ui| {
                        ui.set_width(ui.available_width());
                        ui.radio_value(&mut self.active_entity, Some(*entity), entity.to_string());
                    });
                    ui.allocate_space(Vec2::new(0.0, 5.0));
                }
            });
        });
    }

    fn debug(&mut self, app_state: &AppState, registry: &Registry) {
        self.gui.add_window("Debug Info", |ui| {
            ui.set_width(200.0);
            ui.label(format!("fps: {}", app_state.debug_info.fps));
            ui.label(format!("ms: {}", app_state.debug_info.ms));
            ui.separator();
            ui.label(format!("selected entity: {}", self.active_entity.unwrap_or_default()));
            ui.label(format!("entity count: {}", registry.entity_count()));
        })
    }
}

