use abont_api::{AText, AbontApi, BufferRef, DocumentRef, SelectionRequest};
use std::{collections::HashMap, process::Termination, sync::Mutex};

pub fn new() -> impl AbontApi {
    struct Wrapper(Mutex<AbontEgui>);

    impl AbontApi for Wrapper {
        fn buffer_create(&self) -> abont_api::BufferRef {
            self.0.lock().unwrap().buffer_create()
        }

        fn buffer_show_document(
            &self,
            buffer: abont_api::BufferRef,
            document: abont_api::DocumentRef,
        ) {
            self.0
                .lock()
                .unwrap()
                .buffer_show_document(buffer, document)
        }

        fn document_create(&self) -> abont_api::DocumentRef {
            self.0.lock().unwrap().document_create()
        }
    }

    Wrapper(Mutex::new(AbontEgui::new()))
}

#[derive(Default)]
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

struct Buffer {
    document: Option<DocumentRef>,
}

impl Buffer {
    fn new() -> Buffer {
        Buffer { document: None }
    }
}

struct Document {
    text: AText,
}

impl Document {
    fn new() -> Document {
        Document { text: AText::new() }
    }
}
