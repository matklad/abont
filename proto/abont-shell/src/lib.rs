//! Uses abont-api to implement shell/editor/file manager.

use abont_api::{AText, AbontApi, BufferRef, DocumentRef, SelectionRequest, Split};

pub async fn main(abont: &impl AbontApi) {
    let (input_buffer, _input_document) = show_buffer_with_document(abont, "$".into()).await;
    let (output_buffer, _output_document) =
        show_buffer_with_document(abont, "drwxr-xr-x   - matklad  7 Sep 14:55 proto".into()).await;

    abont.splits_set(Split::Branch(vec![
        Split::Leaf(input_buffer),
        Split::Leaf(output_buffer),
    ])).await;
}

async fn show_buffer_with_document(
    abont: &impl AbontApi,
    initial_contents: AText,
) -> (BufferRef, DocumentRef) {
    let document = abont.document_create().await;
    abont.document_replace(document, SelectionRequest::Everything, initial_contents).await;
    let buffer = abont.buffer_create().await;
    abont.buffer_show_document(buffer, document).await;
    return (buffer, document);
}
