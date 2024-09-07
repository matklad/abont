//! Defines the interface used by the shell and implemented by the engine.
mod atext;

pub use atext::AText;

#[derive(Clone, Copy)]
pub struct DocumentRef(pub u32);

#[derive(Clone, Copy)]
pub struct BufferRef(pub u32);

pub trait AbontApi {
    fn document_create(&self) -> DocumentRef;
    fn document_replace(&self, document: DocumentRef, selection: SelectionRequest, text: AText) {

    }

    fn buffer_create(&self) -> BufferRef;
    fn buffer_show_document(&self, buffer: BufferRef, document: DocumentRef);
}

pub enum SelectionRequest {
  Everything,
  Start,
  End,
  Selection(Selection),
}

pub struct Selection {
  pub ranges: PointRange,
}

pub struct PointRange {
  pub start: Point,
  pub end: Point,
}

pub struct Point {
    pub utf8_index: u32,
}
