mod bus;
mod error;
mod interface;
mod member;
pub use bus::BusName;
pub use error::ErrorName;
pub use interface::InterfaceName;
pub use member::MemberName;

const MAX_NAME_LENGTH: usize = 255;

fn validate(
    name: &str,
    separator: char,
    min_element_count: usize,
    elements_can_start_with_digit: bool,
    validate_element_char: fn(&char) -> bool,
) -> bool {
    if name.len() > MAX_NAME_LENGTH {
        return false;
    }

    let mut element_count = 0;
    let contains_invalid_element = name
        .split(separator)
        .find(|element| {
            element_count += 1;
            !validate_element(
                element,
                elements_can_start_with_digit,
                validate_element_char,
            )
        })
        .is_some();
    if contains_invalid_element {
        return false;
    }

    if element_count < min_element_count {
        return false;
    }

    true
}

fn validate_element(
    element: &str,
    can_start_with_digit: bool,
    validate_element_char: fn(&char) -> bool,
) -> bool {
    let Some(c) = element.chars().next() else {
        return false;
    };
    if !can_start_with_digit && c.is_ascii_digit() {
        return false;
    }

    let contains_invalid_char = element
        .chars()
        .find(|c| !validate_element_char(c))
        .is_some();
    if contains_invalid_char {
        return false;
    }

    true
}
