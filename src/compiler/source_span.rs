use super::{source_buffer::SourceBuffer, source_reader::SourceCursor};

#[derive(Clone)]
pub struct SourceSpan<'a> {
    pub start : SourceCursor<'a>,
    pub size : usize,
    pub data : &'a str,
    pub source : &'a SourceBuffer
}

impl SourceSpan<'_> {
    pub fn at<'a> (source : &'a SourceBuffer, start : SourceCursor<'a>, size : usize) -> SourceSpan<'a> {
        SourceSpan {
            start,
            size,
            data : start.data_ref.split_at(size).0,
            source
        }
    }
}
