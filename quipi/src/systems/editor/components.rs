use crate::{components::{CTransform, CVelocity, CRGBA}, wrappers::egui::GUI, Registry, VersionedIndex};

pub fn entity_components(
    gui: &GUI,
    entity: Option<VersionedIndex>,
    registry: &mut Registry
) {
    if let Some(entity) = entity {
        egui::Window::new("Entity").show(&gui.ctx, |ui| {
            if let Some(transform) = registry.entities.get_mut::<CTransform>(&entity) {
                ui.strong("Transforms");
                ui.label("translate");
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(egui::DragValue::new(&mut transform.translate.x).speed(1.0));
                    ui.label("y");
                    ui.add(egui::DragValue::new(&mut transform.translate.y).speed(1.0));
                    ui.label("z");
                    ui.add(egui::DragValue::new(&mut transform.translate.z).speed(1.0));
                });
                ui.label("scale");
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(egui::DragValue::new(&mut transform.scale.x).speed(0.05));
                    ui.label("y");
                    ui.add(egui::DragValue::new(&mut transform.scale.y).speed(0.05));
                    ui.label("z");
                    ui.add(egui::DragValue::new(&mut transform.scale.z).speed(0.05));
                });
                ui.label("rotation");
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(egui::DragValue::new(&mut transform.rotate.x).speed(0.1));
                    ui.label("y");
                    ui.add(egui::DragValue::new(&mut transform.rotate.y).speed(0.1));
                    ui.label("z");
                    ui.add(egui::DragValue::new(&mut transform.rotate.z).speed(0.1));
                    ui.label("angle");
                    ui.add(egui::DragValue::new(&mut transform.angle).speed(0.1));
                });
                ui.separator();
            }
            if let Some(velocity) = registry.entities.get_mut::<CVelocity>(&entity) {
                ui.strong("Velocity");
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(egui::DragValue::new(&mut velocity.x).speed(1.0));
                    ui.label("y");
                    ui.add(egui::DragValue::new(&mut velocity.y).speed(1.0));
                    ui.label("z");
                    ui.add(egui::DragValue::new(&mut velocity.z).speed(1.0));
                });
                ui.separator();
            }
            if let Some(color) = registry.entities.get_mut::<CRGBA>(&entity) {
                ui.horizontal(|ui| {
                    ui.strong("Color");
                    ui.color_edit_button_rgba_premultiplied(&mut color.value);
                });
                ui.separator();
            }
        });
    }
}