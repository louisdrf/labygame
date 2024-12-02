fn base64_char_to_base10_char(c: char) -> Result<u8, String> {
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

    for base64_character in encoded_radar_view.chars() {
        let char_base10_value = base64_char_to_base10_char(base64_character).unwrap();
        let six_bits_binary_letter = format!("{:06b}", char_base10_value);
        binary_encoded_radar_view.push_str(&six_bits_binary_letter);
    }

    let horizontal_walls_part = binary_encoded_radar_view.drain(..24).collect::<String>();
    let horizontal_walls_part = reverse_bytes(&horizontal_walls_part);

    let vertical_walls_part = binary_encoded_radar_view.drain(..24).collect::<String>();
    let vertical_walls_part = reverse_bytes(&vertical_walls_part);

    let cells_part = binary_encoded_radar_view.drain(..).collect::<String>();
    let cells = parse_cells_part(&cells_part);

    println!("horizontal walls : {} - 4 * 6 bits", horizontal_walls_part);
    println!("vertical walls : {} - 3 * 8 bits", vertical_walls_part);
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
fn parse_cells_part(cells_part: &str) -> Vec<String> {
    let mut cells: Vec<String> = Vec::new();

    for i in 0..9 {
        let cell = &cells_part[i * 4..(i + 1) * 4];
        cells.push(cell.to_string());
    }

    cells
}

/**
 * @param : 24 bits binary string representing each of the horizontal/vertical walls (2 bytes paquets) around the player
 * @returns a Vec<String> containg each of the wall cells
 */
fn split_walls_in_cells(walls_part: &str) -> Vec<String> {
    let mut wall_cells: Vec<String> = Vec::new();

    for i in 0..12 {
        let wall_cell = &walls_part[i * 2..(i + 1) * 2];
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
 * @returns the 24 bits representing each of the horizontal/vertical walls (2 bytes paquets) around the player
 * 
 * example : 11111111-00001111-00000000 : @param (without the '-')
 * ->  00000000-00001111-11111111: : @returns (without the '-')
 */
fn reverse_bytes(binary_string: &str) -> String {
    let mut reversed_binary_string: String = String::new(); 

    for i in 0..3 {
        let byte = &binary_string[i * 8..(i + 1) * 8];
        reversed_binary_string.insert_str(0, &byte); 
    }

    reversed_binary_string
}



#[cfg(test)]
mod tests {
    use super::*;

      #[test]
    fn test_decode() {
        assert_eq!(decode("ieysGjGO8papd/a"), 172);
    }
}

