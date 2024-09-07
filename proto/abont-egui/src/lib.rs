use abont_api::AbontApi;

pub struct AbontEgui {}

impl AbontEgui {
    pub fn new() -> AbontEgui {
        todo!()
    }
}

impl AbontApi for AbontEgui {
    fn document_create(&self) -> abont_api::DocumentRef {
        todo!()
    }

    fn buffer_create(&self) -> abont_api::BufferRef {
        todo!()
    }

    fn buffer_show_document(&self, buffer: abont_api::BufferRef, document: abont_api::DocumentRef) {
        todo!()
    }
}
