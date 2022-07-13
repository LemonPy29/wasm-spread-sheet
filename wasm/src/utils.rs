use std::marker::PhantomData;

pub struct HeaderFillerGenerator<'a, T: 'a> {
    symbols: Vec<[u8; 2]>,
    current: [u8; 2],
    cycles: usize,
    cursor: usize,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> HeaderFillerGenerator<'a, T> {
    pub fn new(slice: &[[u8; 2]], init: [u8; 2]) -> Self {
        let mut symbols = Vec::with_capacity(slice.len());
        symbols.extend_from_slice(slice);
        Self {
            symbols,
            current: init,
            cycles: 0,
            cursor: 0,
            phantom: PhantomData,
        }
    }
}

#[allow(clippy::needless_lifetimes)]
pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

#[allow(clippy::needless_lifetimes)]
impl<'a, T> LendingIterator for HeaderFillerGenerator<'a, T> {
    type Item<'t> = &'t [u8] where Self: 't;

    fn next<'t>(&'t mut self) -> Option<Self::Item<'t>> {
        if self.cycles > self.symbols.len() {
            return None;
        }

        let n = self.cycles.min(1) + 1;
        self.current = self.symbols[self.cursor];

        self.cursor += 1;
        self.cursor %= self.symbols.len();
        if self.cursor % self.symbols.len() == 0 {
            let new_symbol = self.symbols[self.cycles][1];
            self.symbols
                .iter_mut()
                .for_each(|symbol| symbol[0] = new_symbol);
            self.cycles += 1;
        }
        Some(&self.current[0..n])
    }
}

impl<'a, T> Default for HeaderFillerGenerator<'a, T> {
    fn default() -> Self {
        let init = [b'A', b'A'];
        let symbols = [
            [b'A', b'A'],
            [b'B', b'B'],
            [b'C', b'C'],
            [b'D', b'D'],
            [b'E', b'E'],
            [b'F', b'F'],
            [b'G', b'G'],
            [b'H', b'H'],
            [b'I', b'I'],
            [b'J', b'J'],
            [b'K', b'K'],
            [b'L', b'L'],
            [b'M', b'M'],
            [b'N', b'N'],
            [b'O', b'O'],
            [b'P', b'P'],
            [b'Q', b'Q'],
            [b'R', b'R'],
            [b'S', b'S'],
            [b'T', b'T'],
            [b'U', b'U'],
            [b'V', b'V'],
            [b'W', b'W'],
            [b'X', b'X'],
            [b'Y', b'Y'],
            [b'Z', b'Z'],
        ];
        Self::new(&symbols, init)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn filler() {
        let mut filler = HeaderFillerGenerator::<u8>::default();
        let mut bytes = Vec::new();

        for _ in 0..27 {
            let name = filler.next().unwrap();
            bytes.extend_from_slice(name);
        }

        assert_eq!(Some(&b'A'), bytes.get(0));
        assert_eq!(Some(&b'A'), bytes.get(26));
        assert_eq!(Some(&b'A'), bytes.get(27));
    }
}
