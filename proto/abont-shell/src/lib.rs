//! Uses abont-api to implement shell/editor/file manager.

use abont_api::{AText, AbontApi, BufferRef, DocumentRef, SelectionRequest, Split};

pub fn main(abont: &dyn AbontApi) {
    let (input_buffer, _input_document) = show_buffer_with_document(abont, "$".into());
    let (output_buffer, _output_document) =
        show_buffer_with_document(abont, "drwxr-xr-x   - matklad  7 Sep 14:55 proto".into());

    abont.splits_set(Split::Branch(vec![
        Split::Leaf(input_buffer),
        Split::Leaf(output_buffer),
    ]))
}

fn show_buffer_with_document(
    abont: &dyn AbontApi,
    initial_contents: AText,
) -> (BufferRef, DocumentRef) {
    let document = abont.document_create();
    abont.document_replace(document, SelectionRequest::Everything, initial_contents);
    let buffer = abont.buffer_create();
    abont.buffer_show_document(buffer, document);
    return (buffer, document);
}
