use std::num::ParseIntError;

pub fn remove_first_word(s: &String) -> String {
    s.split_whitespace().skip(1).collect::<Vec<_>>().join(" ")
}

pub fn convert_string_to_int_or_display_error_message(
    string: String,
) -> Result<i32, ParseIntError> {
    match string.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(e) => {
            println!("❌ Please enter a correct ID");
            Err(e)
        }
    }
}
