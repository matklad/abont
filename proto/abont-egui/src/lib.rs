use std::{
    collections::HashMap,
    process::Termination,
    sync::{Arc, Mutex},
};

use abont_api::{AText, AbontApi, BufferRef, DocumentRef, SelectionRequest};
use eframe::glow::COLOR;
use egui::Color32;

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

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default()
            // .frame(egui::Frame::default().fill(Color32::LIGHT_BLUE))
            .show(ctx, |ui| {
                let handle = self.handle.0.lock().unwrap();
                for buffer in handle.buffers.values() {
                    if let Some(document_ref) = buffer.document {
                        if let Some(document) = handle.documents.get(&document_ref) {
                            ui.monospace(document.text.as_str());
                        }
                    }
                }
            });
    }
}

#[derive(Clone)]
struct AbontHandle(Arc<Mutex<AbontEgui>>);

impl AbontApi for AbontHandle {
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
    documents: HashMap<DocumentRef, Document>,
    document_id: u32,
    buffers: HashMap<BufferRef, Buffer>,
    buffer_id: u32,
}

impl AbontEgui {
    fn new() -> AbontEgui {
        AbontEgui::default()
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
