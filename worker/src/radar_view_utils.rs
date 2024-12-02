pub fn decode(encoded_radar_view: &str) -> i32 {
    let mut binary_encoded_radar_view: String = String::new(); 

    for base64_character in encoded_radar_view.chars() {
        let char_base10_value = base64_char_to_base10_char(base64_character).unwrap();
        let six_bits_binary_letter = format!("{:06b}", char_base10_value);
        binary_encoded_radar_view.push_str(&six_bits_binary_letter);
    }

    let horizontal_walls_part = binary_encoded_radar_view.drain(..24).collect::<String>();
    let horizontal_walls_part = reverse_bytes(&horizontal_walls_part);
    let horizontal_walls_cells = split_walls_in_cells(&horizontal_walls_part);

    let vertical_walls_part = binary_encoded_radar_view.drain(..24).collect::<String>();
    let vertical_walls_part = reverse_bytes(&vertical_walls_part);
    let vertical_walls_cells = split_walls_in_cells(&vertical_walls_part);

    let cells_part = binary_encoded_radar_view.drain(..).collect::<String>();
    let cells = parse_cells_part(&cells_part);

    let radar_view: Vec<Vec<String>> = vec![vec![String::new(); 7]; 7];
    let radar_view = fill_radar_view_with_horizontal_walls(&radar_view, &horizontal_walls_cells);
    let radar_view = fill_radar_view_with_vertical_walls(&radar_view, &vertical_walls_cells);
    let radar_view = fill_radar_view_with_cells(&radar_view, &cells);

    for row in &radar_view {
        println!("{:?}", row);
    }

    50
}

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

fn fill_radar_view_with_horizontal_walls(radar_view: &Vec<Vec<String>>, horizontal_walls_cells: &Vec<String>) -> Vec<Vec<String>> {
    let mut radar_view = radar_view.clone();
    let mut offset = 0;

    for i in (0..radar_view.len()).step_by(2) {
        radar_view[i][1] = horizontal_walls_cells[offset].clone();
        radar_view[i][3] = horizontal_walls_cells[offset + 1].clone();
        radar_view[i][5] = horizontal_walls_cells[offset + 2].clone();
        offset += 3;
    }

    radar_view
}

fn fill_radar_view_with_vertical_walls(radar_view: &Vec<Vec<String>>, vertical_walls_cells: &Vec<String>) -> Vec<Vec<String>> {
    let mut radar_view = radar_view.clone();
    let mut offset = 0;

    for i in (0..radar_view.len()).step_by(2) {
        radar_view[1][i] = vertical_walls_cells[offset].clone();
        radar_view[3][i] = vertical_walls_cells[offset + 1].clone();
        radar_view[5][i] = vertical_walls_cells[offset + 2].clone();
        offset += 3;
    }

    radar_view
}

fn fill_radar_view_with_cells(radar_view: &Vec<Vec<String>>, cells: &Vec<String>) -> Vec<Vec<String>> {
    let mut radar_view = radar_view.clone();
    let mut offset = 0;

    for i in (1..radar_view.len()).step_by(2) {
        radar_view[1][i] = cells[offset].clone();
        radar_view[3][i] = cells[offset + 1].clone();
        radar_view[5][i] = cells[offset + 2].clone();
        offset += 3;
    }

    radar_view
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
 * reverse bytes of a 24 bits binary string
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

