#[macro_export]
macro_rules! params {
    ($($key: ident), *) => {
        {
            [$(
                $key.to_param().map(|value| (stringify!($key), value))
            ), *]
            .into_iter()
            .filter_map(|opt| opt)
            .collect()
        }
    };
}

pub trait Param {
    fn to_param(&self) -> Option<&str>;
}

impl Param for &str {
    fn to_param(&self) -> Option<&str> {
        Some(self)
    }
}

impl Param for Option<&str> {
    fn to_param(&self) -> Option<&str> {
        match self {
            Some(_) => *self,
            None => None,
        }
    }
}
