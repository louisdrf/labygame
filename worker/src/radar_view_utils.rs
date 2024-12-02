#[derive(Clone, Debug)]
enum RadarCell {
        Undefined,                   // 00 
        Open,                        // 01
        Wall,                        // 10
        Unknown(String)              // unknown bits combination
}

impl RadarCell {
    pub fn from_bits(bits: &str) -> Self {
        match bits {
            "00" | "1111" => RadarCell::Undefined,
            "01" | "0000" => RadarCell::Open,
            "10"   => RadarCell::Wall,
            _ => RadarCell::Unknown(bits.to_string())
        }
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
    let horizontal_walls_cells = split_walls_in_cells(&horizontal_walls_part);

    let vertical_walls_part = binary_encoded_radar_view.drain(..24).collect::<String>();
    let vertical_walls_part = reverse_bytes(&vertical_walls_part);
    let vertical_walls_cells = split_walls_in_cells(&vertical_walls_part);

    let cells_part = binary_encoded_radar_view.drain(..).collect::<String>();
    let cells = parse_cells_part(&cells_part);

    let radar_view: Vec<Vec<RadarCell>> = vec![vec![RadarCell::Undefined; 7]; 7];
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

fn fill_radar_view_with_horizontal_walls(radar_view: &Vec<Vec<RadarCell>>, horizontal_walls_cells: &Vec<String>) -> Vec<Vec<RadarCell>> {
    let mut radar_view = radar_view.clone();
    let mut offset = 0;

    for i in (0..radar_view.len()).step_by(2) {
        radar_view[i][1] = RadarCell::from_bits(horizontal_walls_cells[offset].as_str());
        radar_view[i][3] = RadarCell::from_bits(horizontal_walls_cells[offset + 1].as_str());
        radar_view[i][5] = RadarCell::from_bits(horizontal_walls_cells[offset + 2].as_str());
        offset += 3;
    }

    radar_view
}

fn fill_radar_view_with_vertical_walls(radar_view: &Vec<Vec<RadarCell>>, vertical_walls_cells: &Vec<String>) -> Vec<Vec<RadarCell>> {
    let mut radar_view = radar_view.clone();
    let mut offset = 0;

    for i in (0..radar_view.len()).step_by(2) {
        radar_view[1][i] = RadarCell::from_bits(vertical_walls_cells[offset].as_str());
        radar_view[3][i] = RadarCell::from_bits(vertical_walls_cells[offset + 1].as_str());
        radar_view[5][i] = RadarCell::from_bits(vertical_walls_cells[offset + 2].as_str());
        offset += 3;
    }

    radar_view
}

fn fill_radar_view_with_cells(radar_view: &Vec<Vec<RadarCell>>, cells: &Vec<String>) -> Vec<Vec<RadarCell>> {
    let mut radar_view = radar_view.clone();
    let mut offset = 0;

    for i in (1..radar_view.len()).step_by(2) {
        println!("cell line : {:?} {:?} {:?}", 
        RadarCell::from_bits(cells[offset].as_str()), 
        RadarCell::from_bits(cells[offset + 1].as_str()),
         RadarCell::from_bits(cells[offset + 2].as_str()));

         radar_view[i][1] = RadarCell::from_bits(cells[offset].as_str());
         radar_view[i][3] = RadarCell::from_bits(cells[offset + 1].as_str());
         radar_view[i][5] = RadarCell::from_bits(cells[offset + 2].as_str());
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

