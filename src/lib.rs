use wasm_macro::*;

initiate_protocol!();

macro_rules! str {
    ($val: expr, $type: expr) => {
        match String::from_utf8($val.to_vec()) {
            Ok(val) => val,
            Err(_) => return Err(format!("{}: Failed to parse '' as UTF-8 string.", $type)),
        }
    };
}

macro_rules! int {
    ($val: expr, $type: expr) => {
        match str!($val, $type).parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                return Err(format!(
                    "{}: '{}' is not a valid integer.",
                    $type,
                    str!($val, $type)
                ))
            }
        }
    };
}

macro_rules! bytes {
    ($val: expr) => {
        $val.as_bytes().to_vec()
    };
}

macro_rules! ok {
    ($val: expr) => {
        Ok(bytes!($val))
    };
}

#[wasm_func]
pub fn greet(name: &[u8], age: &[u8]) -> Result<Vec<u8>, String> {
    let name = str!(name, "Name");
    let age = int!(age, "Age");
    ok!(format!("Hello {name} you are {age} years old."))
}
