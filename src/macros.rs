macro_rules! colors {
    ($($int:literal)*) => {
        [$(crate::color::color($int),)*]
    };
}
