//! Defines the interface used by the shell and implemented by the engine.
#![allow(async_fn_in_trait)]
mod atext;

pub use atext::{AText, Point, PointRange, Selection, SelectionRequest};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DocumentRef(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BufferRef(pub u32);

pub trait AbontApi {
    async fn splits_get(&self) -> Split;
    async fn splits_set(&self, splits: Split);

    async fn buffer_create(&self) -> BufferRef;
    async fn buffer_show_document(&self, buffer: BufferRef, document: DocumentRef);

    async fn document_create(&self) -> DocumentRef;
    async fn document_replace(&self, document: DocumentRef, selection: SelectionRequest, text: AText);
}

#[derive(Debug)]
pub enum Split {
    Leaf(BufferRef),
    Branch(Vec<Split>),
}

impl Default for Split {
    fn default() -> Split {
        Split::Branch(Vec::new())
    }
}
