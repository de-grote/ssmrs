use std::sync::Arc;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct SSMRS {
    file_content: Arc<Mutex<Option<String>>>,
    code: Code,
    cpu: Option<Cpu>,
    halted: bool,
    running: bool,
    max_sp: usize,
    initial_sp: usize,
    message_queue: Arc<RwLock<Vec<String>>>,
}

use std::future::Future;

use egui::mutex::{Mutex, RwLock};
use egui::RichText;
use ssmrs::register::Reg;
use ssmrs::{Code, Cpu, Instr, Parser};
#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    tokio::spawn(f);
}
#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

impl SSMRS {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_content: Arc::new(Mutex::new(None)),
            code: vec![Instr::HALT],
            cpu: None,
            halted: true,
            running: false,
            max_sp: 0,
            initial_sp: 0,
            message_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

fn open_file(z: Arc<Mutex<Option<String>>>) {
    use rfd::AsyncFileDialog;
    let task = AsyncFileDialog::new()
        .add_filter("Simple Stack Machine Assembly", &["ssm", "asm"])
        .pick_file();
    execute(async move {
        let file = task.await;
        if let Some(file) = file {
            let contents = file.read().await;
            if let Ok(contents) = String::from_utf8(contents) {
                let mut z = z.lock();
                *z = Some(contents);
            }
        }
    })
}

impl eframe::App for SSMRS {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        {
            let z = self.file_content.lock();
            if let Some(code) = z.as_ref() {
                let p = ssmrs::parse().parse(code.as_str());
                if let Ok(mut p) = p {
                    p.push(Instr::HALT);
                    if self.cpu.is_none() {
                        self.halted = false;
                        let q = self.message_queue.clone();
                        self.cpu = Some(Cpu::new(
                            0,
                            Box::new(move |s| {
                                let mut q = q.write();
                                q.push(s);
                            }),
                        ));
                        if let Some(cpu) = &mut self.cpu {
                            cpu.load_code(p.clone());
                            self.initial_sp = cpu.read_registers().sp as usize;
                            self.max_sp = self.initial_sp;
                        }
                    }
                    self.code = p;
                }
            }
        }

        {
            if self.running {
                if let Some(cpu) = &mut self.cpu {
                    let res = cpu.step();
                    if !res {
                        self.running = false;
                        self.halted = true;
                    }
                    ctx.request_repaint();
                }
            }
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("load file").clicked() {
                        open_file(self.file_content.clone());
                        self.cpu = None;
                        ui.close_menu();
                    }

                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                egui::warn_if_debug_build(ui);
            });
        });
        egui::TopBottomPanel::top("action_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Single Step").clicked() && !self.halted {
                    if let Some(cpu) = &mut self.cpu {
                        if !cpu.step() {
                            self.halted = true;
                        }
                    }
                }

                if ui.button("Run").clicked() && !self.halted && self.cpu.is_some() {
                    self.running = true;
                }

                if ui.button("Pause execution").clicked() {
                    self.running = false;
                }

                if ui.button("Reset").clicked() {
                    self.cpu = None;
                    self.halted = true;
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            // The bottom panel is often a good place for a status bar:
            ui.horizontal(|ui| {
                ui.label("SSMRS by Julius de Jeu");
                ui.label("Based on ");
                ui.hyperlink_to(
                    "Simple Stack Machine",
                    "https://github.com/atzedijkstra/ssm",
                );
                ui.label(" by Atze Dijkstra");
                ui.hyperlink_to(
                    "Docs",
                    "https://webspace.science.uu.nl/~hage0101/SSM/index.html",
                );
            })
        });
        egui::TopBottomPanel::bottom("trap output")
            .min_height(ctx.available_rect().height() / 2.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .max_width(f32::INFINITY)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading("Trap output");
                            if ui.button("Clear output").clicked() {
                                let mut q = self.message_queue.write();
                                q.clear();
                            }
                        });
                        let q = self.message_queue.read();
                        for s in q.iter() {
                            ui.label(s);
                        }
                    });
            });
        egui::TopBottomPanel::bottom("register_overview").show(ctx, |ui| {
            // The bottom panel is often a good place for a status bar:
            egui::Grid::new("register_table").show(ui, |ui| {
                ui.label("PC/R0");
                ui.label("SP/R1");
                ui.label("MP/R2");
                ui.label("R3");
                ui.label("R4");
                ui.label("R5");
                ui.label("R6");
                ui.label("R7");
                ui.end_row();

                if let Some(cpu) = &self.cpu {
                    for i in 0..8 {
                        ui.monospace(format!("{:08x}", cpu.read_registers()[i]));
                    }
                } else {
                    for _ in 0..8 {
                        ui.monospace("00000000");
                    }
                }
            })
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("code_table").show(ui, |ui| {
                    ui.label(RichText::new("Label").strong());
                    ui.label(RichText::new("Address").strong());
                    ui.label(RichText::new("PC").strong());
                    ui.label(RichText::new("Value").strong());
                    ui.label(RichText::new("Instr").strong());
                    ui.label(RichText::new("Arg1").strong());
                    ui.label(RichText::new("Arg2").strong());
                    ui.end_row();

                    let mut count = 0;
                    let mut next_label = None;
                    for instr in self.code.iter() {
                        if let Instr::LABEL(text) = instr {
                            next_label = Some(text.clone());
                            continue;
                        }
                        // label
                        if let Some(label) = next_label {
                            ui.label(label);
                            next_label = None;
                        } else {
                            ui.label("");
                        }
                        // addr
                        ui.label(format!("{:08x}", count));
                        // pc
                        if self
                            .cpu
                            .as_ref()
                            .map_or(false, |cpu| cpu.read_registers().pc == (count as i32))
                        {
                            ui.radio(true, "").clicked();
                        } else {
                            ui.label("");
                        }
                        // value

                        if let Some(cpu) = self.cpu.as_ref() {
                            ui.label(format!("{:08x}", cpu.read_memory()[count]));
                        } else {
                            ui.label("");
                        }
                        // instr
                        for l in instr.name_and_params() {
                            ui.label(l);
                        }
                        ui.end_row();
                        count += instr.instr_size();
                    }
                })
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("stack_contents").show(ui, |ui| {
                    ui.label(RichText::new("Address").strong());
                    ui.label(RichText::new("Value").strong());
                    ui.label(RichText::new("RegPtrs").strong());
                    ui.end_row();

                    if let Some(cpu) = self.cpu.as_ref() {
                        self.max_sp = self.max_sp.max(cpu.read_registers().sp as usize);
                        let start = self.initial_sp;
                        let end = self.max_sp;
                        for i in start..=end {
                            ui.label(format!("{:08x}", i));
                            ui.label(format!("{:08x}", cpu.read_memory()[i]));
                            for r in 0..8 {
                                let reg = cpu.read_registers()[r];
                                if reg == i as i32 {
                                    let r2 = Reg::try_from(r);
                                    match r2 {
                                        Ok(r2) => ui.label(r2.to_string()),
                                        Err(_) => ui.label(""),
                                    };
                                }
                            }
                            ui.end_row();
                        }
                    }
                })
            });
        });
    }
}
