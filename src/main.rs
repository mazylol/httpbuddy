use eframe::egui::RichText;
use eframe::egui::Context;
use eframe::{egui, Frame};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

#[derive(Debug, PartialEq)]
enum ScratchPadMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
}

struct ScratchPad {
    title: String,

    method: ScratchPadMethod,
    url: String,
    body: String,
    request_headers: Vec<(String, String)>,

    response_body: String,
    response_headers: Vec<(String, String)>,
    response_status: reqwest::StatusCode,
    response_time: f32,
    response_size: usize,
}

impl ScratchPad {
    fn new() -> Self {
        Self {
            title: String::from("New Scratch Pad"),
            method: ScratchPadMethod::GET,
            url: String::new(),
            body: String::new(),
            request_headers: Vec::new(),
            response_body: String::new(),
            response_headers: Vec::new(),
            response_status: reqwest::StatusCode::OK,
            response_time: 0.0,
            response_size: 0,
        }
    }
}

struct App {
    scratchpads: Vec<ScratchPad>,
    current_scratchpad: Option<usize>,
    confirm_delete: bool,
    close_button: String,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        cc.egui_ctx.set_fonts(egui::FontDefinitions::default());

        Self {
            scratchpads: Vec::new(),
            current_scratchpad: None,
            confirm_delete: false,
            close_button: String::from('❌'),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Scratch Pads");

                if ui.button("New Scratch Pad").clicked() {
                    let new_scratchpad = ScratchPad::new();
                    self.scratchpads.push(new_scratchpad);
                }

                let mut to_remove = None;

                for (index, scratchpad) in self.scratchpads.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if index == self.current_scratchpad.unwrap_or(usize::MAX) {
                            ui.label(RichText::new(&scratchpad.title).strong());
                            if ui.button(self.close_button.clone()).clicked() {
                                if self.confirm_delete {
                                    to_remove = Some(index);
                                    self.confirm_delete = false;
                                    self.close_button = String::from("❌");
                                } else {
                                    self.confirm_delete = true;
                                    self.close_button = String::from("✅");
                                }
                            }
                        } else {
                            if ui.label(RichText::new(&scratchpad.title)).clicked() {
                                self.current_scratchpad = Some(index);
                            }
                        }
                    });
                }

                if let Some(index) = to_remove {
                    self.scratchpads.remove(index);
                    self.current_scratchpad = None;
                }

                ui.allocate_space(ui.available_size());
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(current_index) = self.current_scratchpad {
                if let Some(current_scratchpad) = self.scratchpads.get_mut(current_index) {
                    ui.columns(2, |columns| {
                        egui::Frame::new()
                            .fill(egui::Color32::from_black_alpha(0))
                            .show(&mut columns[0], |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Method:");
                                    egui::ComboBox::from_id_salt("method")
                                        .selected_text(format!("{:?}", current_scratchpad.method))
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut current_scratchpad.method,
                                                ScratchPadMethod::GET,
                                                "GET",
                                            );
                                            ui.selectable_value(
                                                &mut current_scratchpad.method,
                                                ScratchPadMethod::POST,
                                                "POST",
                                            );
                                            ui.selectable_value(
                                                &mut current_scratchpad.method,
                                                ScratchPadMethod::PUT,
                                                "PUT",
                                            );
                                            ui.selectable_value(
                                                &mut current_scratchpad.method,
                                                ScratchPadMethod::PATCH,
                                                "PATCH",
                                            );
                                            ui.selectable_value(
                                                &mut current_scratchpad.method,
                                                ScratchPadMethod::DELETE,
                                                "DELETE",
                                            );
                                            ui.selectable_value(
                                                &mut current_scratchpad.method,
                                                ScratchPadMethod::OPTIONS,
                                                "OPTIONS",
                                            );
                                            ui.selectable_value(
                                                &mut current_scratchpad.method,
                                                ScratchPadMethod::HEAD,
                                                "HEAD",
                                            );
                                        });

                                    ui.label("URL:");
                                    ui.text_edit_singleline(&mut current_scratchpad.url);
                                });
                            });

                        egui::Frame::new()
                            .fill(egui::Color32::from_black_alpha(0))
                            .show(&mut columns[1], |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(current_scratchpad.url.clone());
                                });
                            });
                    });
                }
            }
        });
    }
}
