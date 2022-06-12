#[macro_export]
macro_rules! hashmap {
    ($($( $key: expr => $val: expr )+$(,)?)*) => {
        {
            let mut ret = ::std::collections::HashMap::new();
            $($(
                ret.insert($key, $val);
            )*)*
            ret
        }
    }
}
