use crate::{ChildCursor, Error, Extract, Result, Span, WithSpan};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Comment<'a> {
    pub lexeme: &'a str,
    pub span: Span,
}

impl<'a> WithSpan for Comment<'a> {
    fn span(&self) -> Span {
        self.span
    }
}

pub struct MultilineComment;

fn multi_parse<'a>(pos: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, Comment<'a>> {
    while let Some((next_span, next_token)) = cursor.next() {
        if next_token == "*" {
            let (next_pos, escape) = match cursor.peek() {
                Some(ret) => ret,
                None => return Err(Error::new(next_span, "unterminated multiline comment")),
            };

            if escape == "/" {
                cursor.next();
                let span = Span::new(pos, next_pos + 1);
                let lexeme = span
                    .slice(cursor.input())
                    .ok_or_else(|| Error::new(next_span, "invalid span"))?;
                return Ok(Comment { lexeme, span });
            } else {
                continue;
            }
        }
    }

    Err(Error::new(pos, "unterminated multiline comment"))
}

impl<'a, T: From<Comment<'a>>> Extract<'a, T> for MultilineComment {
    #[inline]
    fn extract(token: &'a str, span: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, T> {
        if token == "/" {
            if let Some((_, "*")) = cursor.peek() {
                return multi_parse(span, cursor).map(|i| i.into());
            }
        }

        Err(Error::new(span, "comment literal"))
    }
}

pub struct SinglelineComment;

fn single_parse<'a>(pos: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, Comment<'a>> {
    let mut end = pos;
    for (next_pos, next_token) in cursor.by_ref() {
        end = next_pos;
        if next_token == "\n" {
            break;
        }
    }

    let span = Span::new(pos, end + 1);
    let lexeme = span
        .slice(cursor.input())
        .ok_or_else(|| Error::new(end, "invalid span"))?;

    Ok(Comment { lexeme, span })
}

impl<'a, T: From<Comment<'a>>> Extract<'a, T> for SinglelineComment {
    #[inline]
    fn extract(token: &'a str, span: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, T> {
        if token == "/" {
            if let Some((_, "/")) = cursor.peek() {
                return single_parse(span, cursor).map(|i| i.into());
            }
        }

        Err(Error::new(span, "comment literal"))
    }
}
