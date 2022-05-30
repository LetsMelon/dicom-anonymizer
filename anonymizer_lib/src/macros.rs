macro_rules! match_field {
    ($key:expr, $fct:tt) => {
        match $key {
            Some(v) => {
                $fct(v)
            },
            None => (),
        }
    }
}
