//! Defines the interface used by the shell and implemented by the engine.
mod atext;

pub use atext::{AText, SelectionRequest, Selection, Point, PointRange};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DocumentRef(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferRef(pub u32);

pub trait AbontApi {
    fn buffer_create(&self) -> BufferRef;
    fn buffer_show_document(&self, buffer: BufferRef, document: DocumentRef);

    fn document_create(&self) -> DocumentRef;
    fn document_replace(&self, document: DocumentRef, selection: SelectionRequest, text: AText) {
    }
}
