use super::source_buffer::{SourceBuffer, SourcePos};

#[derive(Clone)]
pub struct SourceSpan<'a> {
    pub start : SourcePos,
    pub end : SourcePos,
    pub data : &'a str,
    pub source : &'a SourceBuffer
}

impl SourceSpan<'_> {
    pub fn at<'a> (source : &'a SourceBuffer, start : SourcePos, end : SourcePos) -> SourceSpan<'a> {
        SourceSpan {
            start,
            end,
            data : &source.buffer[start.index..end.index],
            source
        }
    }
}
