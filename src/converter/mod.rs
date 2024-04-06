use std::num::ParseIntError;

#[cfg(test)]
mod tests;

pub fn convert_to_binary(value: u8) -> String {
    format!("{:08b}", value)
}

pub fn convert_to_hex(value: u8) -> String {
    format!("{:02X}", value)
}

pub fn val_from_decimal(value: &str) -> Result<u8,ParseIntError> {
    value.trim().parse()
}

pub fn val_from_binary(value: &str) -> Result<u8,ParseIntError> {
    let aux = value.trim();
    u8::from_str_radix(aux, 2)
}

pub fn val_from_hex(value: &str) -> Result<u8,ParseIntError> {
    let upper = value.trim().to_uppercase();
    u8::from_str_radix(&upper, 16)
}