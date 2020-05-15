
#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            let mut dict = ::std::collections::HashMap::new();
            $(dict.insert($key, $val);)*
            dict
        }
    };
}
