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

#[derive(Default)]
pub struct AText {
    text: String,
}
impl AText {
    pub fn new() -> AText {
        AText::default()
    }

    pub fn replace(&mut self, selection_request: SelectionRequest, text: AText) {
        match selection_request {
            SelectionRequest::Everything => *self = text,
            SelectionRequest::Start => todo!(),
            SelectionRequest::End => todo!(),
            SelectionRequest::Selection(_) => todo!(),
        }
    }
}

impl From<&'_ str> for AText {
    fn from(value: &str) -> AText {
        AText {
            text: value.to_string(),
        }
    }
}
