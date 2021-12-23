use std::str;

pub fn to_str(bytes: Option<&[u8]>) -> Option<&str> {
    bytes.map(|b| str::from_utf8(b).unwrap())
}

#[inline]
fn slice_bytes(bytes: &[u8], border: i32, offset: i32) -> Option<&[u8]> {
    Some(&bytes[border as usize..(offset - border - 1) as usize])
}

pub struct LineSplitter<'a> {
    bytes: &'a [u8],
    finish: bool,
}

impl<'a> LineSplitter<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let finish = false;
        Self { bytes, finish }
    }
}

impl<'a> Iterator for LineSplitter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.finish {
            return None;
        }

        let mut cursor = 0i32;
        let mut iter = self.bytes.iter();
        let mut quoted = false;

        loop {
            cursor += 1;
            match iter.next() {
                Some(&byte) if byte == b'"' => {
                    quoted = !quoted;
                }
                Some(&byte) if byte == b'\n' && !quoted => {
                    break;
                }
                None if !self.finish && cursor > 1 => {
                    self.finish = !self.finish;
                    return Some(&self.bytes);
                }
                None => {
                    return None;
                }
                _ => {
                    continue;
                }
            }
        }

        let res = Some(&self.bytes[..(cursor - 1) as usize]);
        self.bytes = &self.bytes[(cursor as usize)..];
        res
    }
}

pub struct FieldSplitter<'a> {
    bytes: &'a [u8],
    del: u8,
    finish: bool,
}

impl<'a> FieldSplitter<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            del: b',',
            finish: false,
        }
    }
}

impl<'a> Iterator for FieldSplitter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.finish {
            return None;
        }

        let mut cursor = 0i32;
        let mut iter = self.bytes.iter();
        let mut quoted = false;
        let mut n_quotes = 0;

        loop {
            cursor += 1;
            match iter.next() {
                Some(&byte) if byte == b'"' => {
                    if quoted {
                        n_quotes += 1;
                    }
                    quoted = !quoted;
                }
                Some(&byte) if byte == self.del && !quoted => {
                    break;
                }
                None if !self.finish => {
                    self.finish = !self.finish;
                    return slice_bytes(&self.bytes, n_quotes, cursor);
                }
                None => {
                    return None;
                }
                _ => {
                    continue;
                }
            }
        }

        let ret = slice_bytes(&self.bytes, n_quotes, cursor);
        self.bytes = &self.bytes[(cursor as usize)..];
        ret
    }
}

pub struct FieldIter<'a>(pub FieldSplitter<'a>);

impl<'a> FieldIter<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self(FieldSplitter::from_bytes(bytes))
    }
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|b| str::from_utf8(b).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::{to_str, FieldSplitter, LineSplitter};

    #[test]
    fn get_lines() {
        let data = "Espeon, Eevee, Psychic, Gen II
Umbreon, Eevee, Dark, Gen II
";

        let mut line_splitter = LineSplitter::from_bytes(data.as_bytes());

        assert_eq!(
            to_str(line_splitter.next()),
            Some("Espeon, Eevee, Psychic, Gen II")
        );
        assert_eq!(
            to_str(line_splitter.next()),
            Some("Umbreon, Eevee, Dark, Gen II")
        )
    }

    #[test]
    fn get_fields() {
        // Everything as expected
        let data = "Espeon,Eevee,Psychic,Gen II";
        let mut field_splitter = FieldSplitter::from_bytes(data.as_bytes());

        assert_eq!(to_str(field_splitter.next()), Some("Espeon"));
        assert_eq!(to_str(field_splitter.next()), Some("Eevee"));
        assert_eq!(to_str(field_splitter.next()), Some("Psychic"));
        assert_eq!(to_str(field_splitter.next()), Some("Gen II"));
        assert_eq!(to_str(field_splitter.next()), None);

        // Missing field
        let data = "Espeon,,Psychic,Gen II";
        let mut field_splitter = FieldSplitter::from_bytes(data.as_bytes());

        assert_eq!(to_str(field_splitter.next()), Some("Espeon"));
        assert_eq!(to_str(field_splitter.next()), Some(""));
        assert_eq!(to_str(field_splitter.next()), Some("Psychic"));
        assert_eq!(to_str(field_splitter.next()), Some("Gen II"));
        assert_eq!(to_str(field_splitter.next()), None);

        // Delimiter inside a field
        let data = r#"Espeon,"Eevee, Friendship",Psychic,"Gen II, Number""#;
        let mut field_splitter = FieldSplitter::from_bytes(data.as_bytes());

        assert_eq!(to_str(field_splitter.next()), Some(r#"Espeon"#));
        assert_eq!(to_str(field_splitter.next()), Some("Eevee, Friendship"));
        assert_eq!(to_str(field_splitter.next()), Some("Psychic"));
        assert_eq!(to_str(field_splitter.next()), Some("Gen II, Number"));
        assert_eq!(to_str(field_splitter.next()), None);
    }
}
