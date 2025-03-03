
fn base64_decimal_to_char(decimal: u8) -> Result<char, String> {
    match decimal {
        0..=25 => Ok((decimal + 97) as char),
        26..=51 => Ok((decimal + 39) as char),
        52..=61 => Ok((decimal - 4) as char),
        62 => Ok('+'),
        63 => Ok('/'),
        _ => Err(format!("Octet invalide : {}", decimal)),
    }
}

fn complete_octets_vec(mut encoded_binary_text_vec: Vec<u8>) -> Vec<u8> {
    let len_octets = encoded_binary_text_vec.len();
    let modulo_len_octets_3 = len_octets % 3;
    if modulo_len_octets_3 != 0 {
        let add_zero_times = 3 - modulo_len_octets_3;
        for _ in 0..add_zero_times {
            encoded_binary_text_vec.push(0);
        }
    }
    encoded_binary_text_vec
}

fn encode(encoded_binary_text: &[u8]) -> String {
    let mut binary_string: String = String::new(); 

    let mut encoded_binary_text_vec = Vec::from(encoded_binary_text);
    encoded_binary_text_vec = complete_octets_vec(encoded_binary_text_vec);

    for octet in encoded_binary_text_vec {

        let octet_as_binary = format!("{:b}", octet);
        binary_string.push_str(&octet_as_binary);
    }

    let mut encoded_string: String = String::new(); 
    for i in 0..4 {
        let c = &binary_string[i*6..(i+1) * 6];
        let decimal = u32::from_str_radix(c, 2).expect("Binary to decimal conversion failed");
        let char_64 = base64_decimal_to_char(decimal as u8).expect("decimal to char conversion failed");
        encoded_string.push(char_64);
    }

        println!("{} ", encoded_string);
        encoded_string
}


#[cfg(test)]
mod tests {
    use super::*;

      #[test]
    fn test_base64_decimal_to_char() {
        assert_eq!(base64_decimal_to_char(5).unwrap(), 'f');
        assert_eq!(base64_decimal_to_char(33).unwrap(), 'H');
        assert_eq!(base64_decimal_to_char(56).unwrap(), '4');
    }

      #[test]
    fn test_complete_octets_vec() {
        assert_eq!(complete_octets_vec(vec![1]), vec![1, 0, 0]);
        assert_eq!(complete_octets_vec(vec![1, 2]), vec![1, 2, 0]);
        assert_eq!(complete_octets_vec(vec![1, 2, 3]), vec![1, 2, 3]);
    }

      #[test]
    fn test_encode() {
        assert_eq!(encode(&[72, 101, 108, 108, 111]), "KzDM");
    }
}