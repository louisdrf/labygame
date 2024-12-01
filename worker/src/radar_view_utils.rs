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
    let horizontal_walls_part_length = 24;
    let horizontal_walls_part = binary_encoded_radar_view
                                        .drain(..horizontal_walls_part_length)
                                        .collect::<String>();
    let horizontal_walls_binary_string = get_decoded_walls_binary_string(&horizontal_walls_part);
    println!("horizontal walls : {} - 4 * 6 bits", horizontal_walls_binary_string);

    // parsing vertical walls bits
    let vertical_walls_part_length = 24;
    let vertical_walls_part = binary_encoded_radar_view
                                      .drain(..vertical_walls_part_length)
                                      .collect::<String>();
    let vertical_walls_binary_string = get_decoded_walls_binary_string(&vertical_walls_part);
    println!("vertical walls : {} - 3 * 8 bits", vertical_walls_binary_string);

    // parsing cells
    let cells_part = binary_encoded_radar_view
                            .drain(..)
                            .collect::<String>();
    let cells = get_cells(&cells_part);
    display_cells(cells);

    50
}

fn display_cells(cells: Vec<String>) {
    let mut counter = 0;

    for cell in cells {
        if cell == "1111" { print!(" Undefined ") }
        if cell == "0000" { print!(" Rien ") }

        counter += 1;

        if counter == 3 {
            println!();
            counter = 0;
        }
    }
}

/**
 * @param : 40 bits string representation of the cells
 * @returns a Vec<String> containg each cells
 */
fn get_cells(binary_encoded_radar_view_cells_bytes: &str) -> Vec<String> {
    let mut cells: Vec<String> = Vec::new();
    let number_of_cells_bytes = 9;
    let cell_byte_size = 4;

    for byte_index in 0..number_of_cells_bytes {
        let byte_start_index = byte_index * cell_byte_size;
        let byte_end_index = (byte_index + 1) * cell_byte_size;

        let cell = &binary_encoded_radar_view_cells_bytes[byte_start_index..byte_end_index];
        cells.push(cell.to_string());
    }

    cells
}

/**
 * @param : 24 bits string representation of the horizontal/vertical walls
 * @returns a Vec<String> containg each of the wall cells
 */
fn get_walls_cells(decoded_walls_bs: &str) -> Vec<String> {
    let mut wall_cells: Vec<String> = Vec::new();

    let wall_cell_byte_size = 2;
    let number_of_cells = 12;

    for byte_index in 0..number_of_cells {
        let byte_start_index = byte_index * wall_cell_byte_size;
        let byte_end_index = (byte_index + 1) * wall_cell_byte_size;

        let wall_cell = &decoded_walls_bs[byte_start_index..byte_end_index];

        wall_cells.push(wall_cell.to_string());
    }

    wall_cells
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

