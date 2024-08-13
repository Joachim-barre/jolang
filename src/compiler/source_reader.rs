use super::{source_span::SourceSpan, SourceBuffer};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct SourceCursor<'a> {
    pub data_ref : &'a str,
    pub line : usize,
    pub collumn : usize,
}

pub struct SourceReader<'a> {
    pub source : &'a SourceBuffer,
    current_cursor : SourceCursor<'a>
}

impl<'a> SourceReader<'a> {
    pub fn new<'b>(source : &'b SourceBuffer) -> SourceReader<'b> {
        SourceReader {
            source,
            current_cursor : SourceCursor{
                data_ref : source.buffer.as_str(),
                line : 1,
                collumn : 1
            }
        }
    }

    pub fn peek_char(&self) -> Option<char> {
        // TODO optimize
        self.current_cursor.data_ref.chars().nth(0)
    }

    pub fn next_char(&mut self) -> Option<char> {
        let mut iter = self.current_cursor.data_ref.chars();
        iter.next();
        self.current_cursor = SourceCursor{
            data_ref : iter.as_str(),
            line : self.current_cursor.line,
            collumn : self.current_cursor.line
        };
        if let Some(c) = self.peek_char() {
            match c {
                '\n' => {
                    self.current_cursor.line += 1;
                    self.current_cursor.collumn = 0;
                    Some(c)
                },
                _ => {
                    self.current_cursor.collumn += 1;
                    Some(c)
                }
            }
        }else {
            None
        }
    }

    pub fn get_cursor(&self) -> &SourceCursor {
        &self.current_cursor
    }

    pub fn goto(&mut self, cursor : SourceCursor<'a>) {
        self.current_cursor = cursor
    }

    pub fn goto_begin(&mut self) {
        self.goto(SourceCursor{
            data_ref : self.source.buffer.as_str(),
            line : 1,
            collumn : 1
        })
    }

    pub fn read_span(&self, size : usize) -> Option<SourceSpan>{
        Some(SourceSpan::at(self.source, self.current_cursor, size))
    }
}

impl<'a> From<&'a SourceBuffer> for SourceReader<'a> {
    fn from(value: &'a SourceBuffer) -> Self {
        SourceReader::new(value)
    }
}
