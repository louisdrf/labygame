fn base64_char_to_decimal(c: char) -> Result<u8, String> {
    match c {
        'a'..='z' => Ok((c as u8) - 97),
        'A'..='Z' => Ok((c as u8) - 39),
        '0'..='9' => Ok((c as u8) + 4),
        '+' => Ok(62),
        '/' => Ok(63),
        _ => Err(format!("Caractère invalide : {}", c)),
    }
}



pub fn decode(encoded_radar_view: &str) -> i32 {
    let mut result: Vec<u8> = Vec::new();

    let mut binary: String = String::new(); 

    for c in encoded_radar_view.chars() {
        let c_as_base64 = base64_char_to_decimal(c).unwrap();
        
        let binary_letter = format!("{:08b}", c_as_base64);

        let b6_letter = &binary_letter[2..]; // keep 6 last bits only

        binary.push_str(b6_letter);
    }

    let mut binary_reversed: String = String::new(); 

    for i in 0..3 {
        let part = &binary[i*8..(i+1) * 8];
        binary_reversed.insert_str(0, &part);
    }

    println!("{}",binary_reversed);

    result.len() as i32
}


#[cfg(test)]
mod tests {
    use super::*;

      #[test]
    fn test_decode() {
        assert_eq!(decode("ieys"), 30);
    }


  /*    #[test]
    fn test_A_base_64_format() {
        assert_eq!(base64_char_to_decimal('A').unwrap(), 26);
    } 

    #[test]
    fn test_e_base_64_format() {
        assert_eq!(base64_char_to_decimal('e').unwrap(), 4);
    } 

    #[test]
    fn test_0_base_64_format() {
        assert_eq!(base64_char_to_decimal('0').unwrap(), 52);
    } 

    */



}