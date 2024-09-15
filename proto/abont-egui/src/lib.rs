use std::{
    collections::HashMap,
    sync::{
        self,
        mpsc::{Receiver, Sender},
    },
};

use abont_api::{AText, AbontApi, BufferRef, DocumentRef, SelectionRequest, Split};
use egui::{Layout, Pos2, Rect, Vec2};

pub trait AbontApp: 'static {
    fn start(self, abont_api: impl AbontApi + Send + 'static);
}

pub fn run(app: impl AbontApp) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let result = eframe::run_native(
        "abont",
        options,
        Box::new(move |cc| {
            let (tx, rx) = sync::mpsc::channel();
            app.start(AbontHandle {
                tx,
                ctx: cc.egui_ctx.clone(),
            });

            Ok(Box::new(Gui {
                rx,
                abont: AbontEgui::new(),
            }))
        }),
    );
    result
}

type F = Box<dyn FnOnce(&mut AbontEgui) + Send + 'static>;

struct Gui {
    rx: Receiver<F>,
    abont: AbontEgui,
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(f) = self.rx.try_recv() {
            f(&mut self.abont)
        }
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default()
            // .frame(egui::Frame::default().fill(Color32::LIGHT_BLUE))
            .show(ctx, |ui| {
                self.update_split(ui, &self.abont, &self.abont.splits, 0);
            });
    }
}

impl Gui {
    fn update_split(&self, ui: &mut egui::Ui, abont: &AbontEgui, split: &Split, level: u32) {
        match split {
            Split::Leaf(buffer) => self.update_buffer(ui, abont, *buffer),
            Split::Branch(splits) => {
                if splits.is_empty() {
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

#[derive(Clone)]
struct AbontHandle {
    ctx: egui::Context,
    tx: Sender<F>,
}

impl AbontHandle {
    async fn call<T: Send + Sync + 'static>(
        &self,
        f: impl FnOnce(&mut AbontEgui) -> T + Send + Sync + 'static,
    ) -> T {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(Box::new(move |abont| {
            let _ = tx.send(f(abont));
        }));
        self.ctx.request_repaint();
        rx.await.unwrap()
    }
}

impl AbontApi for AbontHandle {
    async fn splits_get(&self) -> abont_api::Split {
        todo!()
    }

    async fn splits_set(&self, splits: abont_api::Split) {
        self.call(move |abont| abont.splits_set(splits)).await
    }

    async fn buffer_create(&self) -> abont_api::BufferRef {
        self.call(move |abont| abont.buffer_create()).await
    }

    async fn buffer_show_document(
        &self,
        buffer: abont_api::BufferRef,
        document: abont_api::DocumentRef,
    ) {
        self.call(move |abont| abont.buffer_show_document(buffer, document))
            .await
    }

    async fn document_create(&self) -> abont_api::DocumentRef {
        self.call(move |abont| abont.document_create()).await
    }

    async fn document_replace(
        &self,
        document: DocumentRef,
        selection: SelectionRequest,
        text: AText,
    ) {
        self.call(move |abont| abont.document_replace(document, selection, text))
            .await
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
