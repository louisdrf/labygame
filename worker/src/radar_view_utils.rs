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
    let mut binary: String = String::new(); 

    for c in encoded_radar_view.chars() {
        let c_as_base64 = base64_char_to_decimal(c).unwrap();
        let binary_letter = format!("{:06b}", c_as_base64);
        binary.push_str(&binary_letter);
    }

    // HORIZONTAL WALLS -> 24 first bits by 6 bits paquets
    let mut horizontal_walls_reversed: String = String::new(); 
    for i in 0..4 {
        let part = &binary[i*6..(i+1) * 6];
        horizontal_walls_reversed.insert_str(0, &part);
    }

    println!("horizontal walls : {} - 4 * 6 bits", horizontal_walls_reversed);


    // VERTICAL WALLS -> 24 next bits by 8 bits paquets
    let mut vertical_walls_reversed: String = String::new(); 
    for i in 3..6 {
        let part = &binary[i*8..(i+1) * 8];
        vertical_walls_reversed.insert_str(0, &part);
    }

    println!("horizontal walls : {} - 3 * 8 bits", vertical_walls_reversed);


    // CELLS -> last 40 bits by 8 bits paquets
    let mut cells: String = String::new(); 
    for i in 6..11 {
        let part = &binary[i*8..(i+1) * 8];
        cells.push_str(part);
    }

    println!("cells : {} - 5 * 8 bits", cells);


    // couper en 4 * 9 bits
    let mut counter = 0;

    for i in 0..9 {
        let cell = &cells[i*4..(i+1) * 4];
        if cell == "1111" { print!(" Undefined ") }
        if cell == "0000" { print!(" Rien ") }

        counter += 1;

        if counter == 3 {
            println!();
            counter = 0;
        }
    }

    horizontal_walls_reversed.len() as i32
}


#[cfg(test)]
mod tests {
    use super::*;

      #[test]
    fn test_decode() {
        assert_eq!(decode("ieysGjGO8papd/a"), 172);
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