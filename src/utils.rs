#![macro_use]

macro_rules! hash_map {
    [$(($key: expr, $value: expr)), *] => ({
        let mut result = HashMap::new();
        $(
            result.insert($key, $value)
         ); *;
        return result;
     })
}
