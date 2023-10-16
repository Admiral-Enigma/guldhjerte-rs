use unicode_segmentation::UnicodeSegmentation;

pub fn to_string(price: &str) -> Result<String, String> {
    UnicodeSegmentation::graphemes(price, true)
        .map(parse_input_digit)
        .collect()
}

fn parse_input_digit(character: &str) -> Result<String, String> {
    if character == "." {
        Ok(",".to_string())
    } else {
        if let Ok(digit) = character.parse::<usize>() {
            if digit < super::CHARSET.chars().count() {
                Ok(super::CHARSET.chars().nth(digit).unwrap().to_string())
            } else {
                Err("Invalid digit".to_string())
            }
        } else {
            Err("Invalid digit".to_string())
        }
    }
}

//TODO: to_barcode, to_QR_code
