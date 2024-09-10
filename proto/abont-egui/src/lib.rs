use std::{
    collections::HashMap,
    process::Termination,
    sync::{Arc, Mutex},
};

use abont_api::{AText, AbontApi, BufferRef, DocumentRef, SelectionRequest, Split};
use eframe::glow::COLOR;
use egui::{Color32, Layout, Pos2, Rect, Vec2};

pub fn run(app: Box<dyn FnOnce(&dyn AbontApi) + Send>) -> eframe::Result {
    let handle = AbontHandle(Arc::new(Mutex::new(AbontEgui::new())));
    let app_thread = std::thread::spawn({
        let handle = handle.clone();
        move || app(&handle)
    });

    std::thread::sleep_ms(100);

    eprintln!("handle.0.lock().unwrap() = {:?}", handle.0.lock().unwrap());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let result = eframe::run_native(
        "abont",
        options,
        Box::new(move |cc| {
            // This gives us image support:
            Ok(Box::new(Gui { handle }))
        }),
    );
    app_thread.join().unwrap();
    result
}

struct Gui {
    handle: AbontHandle,
}
impl Gui {
    fn update_split(&self, ui: &mut egui::Ui, abont: &AbontEgui, split: &Split, level: u32) {
        match split {
            Split::Leaf(buffer) => self.update_buffer(ui, abont, *buffer),
            Split::Branch(splits) => {
                if (splits.is_empty()) {
                    return;
                }
                let vertical = level % 2 == 0;
                let size = ui.available_size();
                let mut i = 0.0;
                for split in splits {
                    let rect = if vertical {
                        let step = size.y / (splits.len() as f32);
                        Rect::from_min_size(Pos2::new(0.0, step * i), Vec2::new(size.x, step))
                    } else {
                        let step = size.x / (splits.len() as f32);
                        Rect::from_min_size(Pos2::new(step * i, 0.0), Vec2::new(step, size.y))
                    };
                    self.update_split(
                        &mut ui.child_ui(rect, Layout::default(), None),
                        abont,
                        split,
                        level + 1,
                    );
                    i += 1.0;
                }
            }
        }
    }

    fn update_buffer(&self, ui: &mut egui::Ui, abont: &AbontEgui, buffer_ref: BufferRef) {
        let Some(buffer) = abont.buffers.get(&buffer_ref) else {
            return;
        };
        let Some(document_ref) = buffer.document else {
            return;
        };
        let Some(document) = abont.documents.get(&document_ref) else {
            return;
        };
        ui.monospace(document.text.as_str());
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default()
            // .frame(egui::Frame::default().fill(Color32::LIGHT_BLUE))
            .show(ctx, |ui| {
                let handle = self.handle.0.lock().unwrap();
                self.update_split(ui, &*handle, &handle.splits, 0);
            });
    }
}

#[derive(Clone)]
struct AbontHandle(Arc<Mutex<AbontEgui>>);

impl AbontApi for AbontHandle {
    fn splits_get(&self) -> abont_api::Split {
        todo!()
    }

    fn splits_set(&self, splits: abont_api::Split) {
        self.0.lock().unwrap().splits_set(splits);
    }

    fn buffer_create(&self) -> abont_api::BufferRef {
        self.0.lock().unwrap().buffer_create()
    }

    fn buffer_show_document(&self, buffer: abont_api::BufferRef, document: abont_api::DocumentRef) {
        self.0
            .lock()
            .unwrap()
            .buffer_show_document(buffer, document)
    }

    fn document_create(&self) -> abont_api::DocumentRef {
        self.0.lock().unwrap().document_create()
    }

    fn document_replace(&self, document: DocumentRef, selection: SelectionRequest, text: AText) {
        self.0
            .lock()
            .unwrap()
            .document_replace(document, selection, text)
    }
}

#[derive(Default, Debug)]
pub struct AbontEgui {
    splits: Split,
    documents: HashMap<DocumentRef, Document>,
    document_id: u32,
    buffers: HashMap<BufferRef, Buffer>,
    buffer_id: u32,
}

impl AbontEgui {
    fn new() -> AbontEgui {
        AbontEgui::default()
    }

    fn splits_set(&mut self, splits: abont_api::Split) {
        self.splits = splits;
    }

    fn buffer_create(&mut self) -> BufferRef {
        let buffer_ref = BufferRef(self.buffer_id);
        self.buffer_id += 1;
        self.buffers.insert(buffer_ref, Buffer::new());
        buffer_ref
    }

    fn buffer_show_document(&mut self, buffer_ref: BufferRef, document_ref: DocumentRef) {
        let Some(buffer) = self.buffers.get_mut(&buffer_ref) else {
            return;
        };
        buffer.document = Some(document_ref)
    }

    fn document_create(&mut self) -> DocumentRef {
        let document_ref = DocumentRef(self.document_id);
        self.document_id += 1;
        self.documents.insert(document_ref, Document::new());
        document_ref
    }
    fn document_replace(
        &mut self,
        document_ref: DocumentRef,
        selection: SelectionRequest,
        text: AText,
    ) {
        let Some(document) = self.documents.get_mut(&document_ref) else {
            return;
        };

        document.text.replace(selection, text);
    }
}

#[derive(Debug)]
struct Buffer {
    document: Option<DocumentRef>,
}

impl Buffer {
    fn new() -> Buffer {
        Buffer { document: None }
    }
}

#[derive(Debug)]
struct Document {
    text: AText,
}

impl Document {
    fn new() -> Document {
        Document { text: AText::new() }
    }
}
