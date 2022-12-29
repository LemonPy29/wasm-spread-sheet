#[macro_export]
macro_rules! equal_to_series {
    ($t:tt) => {
        fn equal_to(&self, other: &dyn SeriesTrait) -> $crate::series::errors::FilterResult {
            let set = other.$t()?.iter().collect::<std::collections::HashSet<_>>();
            let ret = self
                .iter()
                .map(move |el| set.contains(el))
                .collect::<bitvec::prelude::BitVec>();

            Ok(ret)
        }
    };
}

#[macro_export]
macro_rules! sum_series {
    ($t:tt) => {
        fn sum(&self) -> Result<Box<dyn $crate::series::SeriesTrait>, &str> {
            let sum = self
                .iter()
                .fold($t::default(), |acc, x| acc + x.unwrap_or_default());
            let series = vec![Some(sum)];

            Ok(Box::new(series))
        }
    };
}

#[macro_export]
macro_rules! join_series {
    () => {
        fn join(&self, offset: usize, size: usize) -> String {
            self.iter()
                .skip(offset)
                .take(size)
                .map(|opt| opt.map_or("".into(), |number| number.to_string()))
                .intersperse($crate::series::DELIMITER_TOKEN.into())
                .collect()
        }
    };
}

#[macro_export]
macro_rules! filter_join {
    () => {
        fn filter_join(&self, mask: &BitSlice, offset: usize, size: usize) -> String {
            self.iter()
                .zip(mask)
                .filter_map(|(opt, mask_el)| {
                    mask_el.then(|| opt.map_or("".into(), |el| el.to_string()))
                })
                .skip(offset)
                .take(size)
                .intersperse(DELIMITER_TOKEN.into())
                .collect::<String>()
        }
    };
}

#[macro_export]
macro_rules! distinct {
    ($t:tt) => {
        fn distinct(&self) -> Result<String, NonHashable> {
            let set = self
                .$t()
                .unwrap()
                .iter()
                .collect::<std::collections::HashSet<_>>();
            let ret = set
                .iter()
                .map(|&el| el.map_or("".into(), |el| el.to_string()))
                .intersperse(DELIMITER_TOKEN.into())
                .collect();

            Ok(ret)
        }
    };
}
