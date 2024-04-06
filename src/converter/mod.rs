#[cfg(test)]
mod tests;

pub fn convert_to_binary(value: u8) -> String {
    format!("{:b}", value)
}

pub fn convert_to_hex(value: u8) -> String {
    format!("{:X}", value)
}

pub fn val_from_decimal(value: &str) -> u8 {
    value.parse().unwrap()
}

pub fn val_from_binary(value: &str) -> u8 {
    u8::from_str_radix(value, 2).unwrap()
}

pub fn val_from_hex(value: &str) -> u8 {
    u8::from_str_radix(value, 16).unwrap()
}