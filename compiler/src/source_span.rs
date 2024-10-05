use super::{source_buffer::SourceBuffer, source_reader::SourceCursor};

#[derive(Clone, Debug)]
pub struct SourceSpan<'a> {
    pub start : SourceCursor<'a>,
    pub size : usize,
    pub data : &'a str,
    pub source : &'a SourceBuffer
}

impl<'a> PartialEq for SourceSpan<'a> {
    fn eq(&self, other: &Self) -> bool {
        return self.start == other.start
            && self.size == other.size
            && self.source.path == other.source.path;
    }
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
