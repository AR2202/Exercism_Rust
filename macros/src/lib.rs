pub use ::std::collections::HashMap;
#[macro_export]

macro_rules! hashmap {
    
   ($ ($x:expr => $y:expr),*)=> {
        {
            let mut temp_hashmap = ::std::collections::HashMap::new();
            $(
                temp_hashmap.insert($x, $y);
            )*
            temp_hashmap
        }
    };
    ($ ($x:expr => $y:expr,)*)=> {
        {
            let mut temp_hashmap = ::std::collections::HashMap::new();
            $(
                temp_hashmap.insert($x, $y);
            )*
            temp_hashmap
        }
    };
    
}
