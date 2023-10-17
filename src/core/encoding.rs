use unicode_segmentation::UnicodeSegmentation;

pub trait Guldhjerte {
    fn to_guldhjerte(&self) -> Result<String, String>;
    fn to_price(&self) -> String;
}

impl Guldhjerte for String {
    fn to_guldhjerte(&self) -> Result<String, String> {
        encode_to_string(self)
    }

    fn to_price(&self) -> String {
        decode_to_price(self)
    }
}

fn encode_to_string(price: &str) -> Result<String, String> {
    UnicodeSegmentation::graphemes(price, true)
        .map(price_to_guldhjerte)
        .collect()
}

fn decode_to_price(guldhjerte: &str) -> String {
    UnicodeSegmentation::graphemes(guldhjerte, true)
        .map(|character| match character.to_lowercase().as_str() {
            "e" => "0".to_string(),
            "g" => "1".to_string(),
            "u" => "2".to_string(),
            "l" => "3".to_string(),
            "d" => "4".to_string(),
            "h" => "5".to_string(),
            "j" => "6".to_string(),
            "æ" => "7".to_string(),
            "r" => "8".to_string(),
            "t" => "9".to_string(),
            "," => ".".to_string(),
            _ => "".to_string(),
        })
        .collect()
}

fn price_to_guldhjerte(character: &str) -> Result<String, String> {
    if character == "." || character == "," {
        Ok(",".to_string())
    } else if let Ok(digit) = character.parse::<usize>() {
        if digit < super::CHARSET.chars().count() {
            Ok(super::CHARSET.chars().nth(digit).unwrap().to_string())
        } else {
            Err("Invalid digit".to_string())
        }
    } else {
        Err("Invalid digit".to_string())
    }
}

//TODO: to_barcode, to_QR_code
