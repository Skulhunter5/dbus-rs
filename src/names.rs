mod bus;
mod error;
mod interface;
mod member;
pub use bus::BusName;
pub use error::ErrorName;
pub use interface::InterfaceName;
pub use member::MemberName;

pub const MAX_LENGTH: usize = 255;

fn validate(
    name: &str,
    separator: char,
    min_element_count: usize,
    elements_can_start_with_digit: bool,
    validate_element_char: fn(&char) -> bool,
) -> Result<(), String> {
    if name.len() > MAX_LENGTH {
        return Err(format!(
            "too long: has {} but max allowed is {}",
            name.len(),
            MAX_LENGTH
        ));
    }

    let mut element_count = 0;
    for (element, error) in name.split(separator).map(|element| {
        (
            element,
            validate_element(
                element,
                elements_can_start_with_digit,
                validate_element_char,
            )
            .err(),
        )
    }) {
        element_count += 1;
        if let Some(error) = error {
            return Err(format!("invalid element ({:?}): {}", element, error));
        }
    }

    if element_count < min_element_count {
        return Err(format!(
            "not enough elements: has {} but expected at least {}",
            element_count, min_element_count
        ));
    }

    Ok(())
}

fn validate_element(
    element: &str,
    can_start_with_digit: bool,
    validate_element_char: fn(&char) -> bool,
) -> Result<(), String> {
    let Some(c) = element.chars().next() else {
        return Err("empty".to_owned());
    };
    if !can_start_with_digit && c.is_ascii_digit() {
        return Err("starts with digit".to_owned());
    }

    if let Some(invalid_char) = element.chars().find(|c| !validate_element_char(c)) {
        return Err(format!("invalid character: {:?}", invalid_char));
    }

    Ok(())
}
