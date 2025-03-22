#[macro_export]
macro_rules! get_from_enum {
    ($enum:ident :: $arm:ident($var:ident) = $old:expr) => {
        let $enum::$arm($var) = $old else {
            unreachable!()
        };
    };
    ($enum:ident :: $arm:ident { $($data:tt)* } = $old:expr) => {
        let $enum::$arm { $($data)* } = $old else {
            unreachable!()
        };
    };
}

#[macro_export]
macro_rules! assert_matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        assert!(matches!($expression, $pattern $(if $guard)?))
    };
}
