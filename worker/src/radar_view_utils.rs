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
    let mut binary_encoded_radar_view: String = String::new(); 

    for encoded_char in encoded_radar_view.chars() {
        let encoded_char_decimal_value = base64_char_to_decimal(encoded_char).unwrap();
        let six_bits_binary_letter = format!("{:06b}", encoded_char_decimal_value);
        binary_encoded_radar_view.push_str(&six_bits_binary_letter);
    }

    // parsing horizontal walls bits
    let binary_encoded_radar_view_horizontal_walls_part = &binary_encoded_radar_view[0..24]; 

    let horizontal_walls_binary_string = get_decoded_walls_binary_string(&binary_encoded_radar_view_horizontal_walls_part);
    println!("horizontal walls : {} - 4 * 6 bits", horizontal_walls_binary_string);

    // parsing vertical walls bits
    let binary_encoded_radar_view_vertical_walls_part = &binary_encoded_radar_view[24..48]; 

    let vertical_walls_binary_string = get_decoded_walls_binary_string(&binary_encoded_radar_view_vertical_walls_part);
    println!("vertical walls : {} - 3 * 8 bits", vertical_walls_binary_string);


    // CELLS -> last 40 bits by 8 bits paquets
    let mut cells: String = String::new(); 
    for i in 6..11 {
        let part = &binary_encoded_radar_view[i*8..(i+1) * 8];
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

    50
}


/**
 * @param: the 24 bits representing the horizontal/vertical walls in the radar view
 * 
 * split each of the rows/columns, represented by each 8 bits paquets of the @param
 * build a string with each of these paquets in the reversed order because of the little endian encoding
 * 
 * @returns the 24 bits representing each of the walls representing the rows/columns in the radar view
 */
fn get_decoded_walls_binary_string(encoded_walls_bs: &str) -> String {
     // bs = BINARY_STRING
     let mut decoded_walls_bs: String = String::new(); 

     let number_of_bytes = 3;
     let byte_size = 8;
 
     for byte_index in 0..number_of_bytes {
         let byte_start_index = byte_index * byte_size;
         let byte_end_index = (byte_index + 1) * byte_size;
 
         let walls_bs = &encoded_walls_bs[byte_start_index..byte_end_index];
 
         decoded_walls_bs.insert_str(0, &walls_bs); // reverse while adding it because of the little endian encoding
     }
 
     decoded_walls_bs
}



#[cfg(test)]
mod tests {
    use super::*;

      #[test]
    fn test_decode() {
        assert_eq!(decode("ieysGjGO8papd/a"), 172);
    }
}

