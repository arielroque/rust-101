// Public function
pub fn print_value() -> String{
    let res = self::private_function();
    return res;
}

// Private function
pub fn private_function() -> String{
    return "OK".to_string();
}
