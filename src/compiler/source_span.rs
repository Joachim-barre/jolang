use super::source_buffer::SourceBuffer;

pub struct SourceSpan<'a> {
    pub start : usize,
    pub end : usize,
    pub data : &'a str,
    pub source : &'a SourceBuffer
}

impl SourceSpan<'_> {
    pub fn at<'a> (source : &'a SourceBuffer, start : usize, end : usize) -> SourceSpan<'a> {
        SourceSpan {
            start,
            end,
            data : &source.buffer[start..end],
            source
        }
    }
}
