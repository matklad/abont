//! Uses abont-api to implement shell/editor/file manager.

use abont_api::{AbontApi, SelectionRequest};

pub fn main(abont: &dyn AbontApi) {
    let document = abont.document_create();
    abont.document_replace(document, SelectionRequest::Everything, "hello world".into());

    let buffer = abont.buffer_create();
    abont.buffer_show_document(buffer, document);
}
