pub fn annotate(minefield: &[&str]) -> Vec<String> {
    return minefield
        .iter()
        .enumerate()
        .map(|(x, &row)| annotate_row(x, row, minefield))
        .collect::<Vec<String>>();
}

fn annotate_row(x: usize, row: &str, minefield: &[&str]) -> String {
    return row
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(y, tile)| annotate_tile(x, y, *tile, minefield))
        .collect::<String>();
}

fn annotate_tile(x: usize, y: usize, tile: u8, minefield: &[&str]) -> String {
    if is_mine(tile) {
        return "*".to_string();
    } else {
        let mines_count = count_surrounding_mines(x, y, minefield);
        if mines_count > 0 {
            return mines_count.to_string();
        } else {
            return " ".to_string();
        }
    }
}

fn count_surrounding_mines(x: usize, y: usize, minefield: &[&str]) -> u8 {
    return count_top_row_mines(x, y, minefield)
        + count_mines_for_row(y, minefield[x].as_bytes(), false)
        + count_bottom_row_mines(x, y, minefield);
}

fn count_top_row_mines(x: usize, y: usize, minefield: &[&str]) -> u8 {
    if x == 0 {
        return 0;
    }
    return count_mines_for_row(y, minefield[x - 1].as_bytes(), true);
}

fn count_bottom_row_mines(x: usize, y: usize, minefield: &[&str]) -> u8 {
    if x == minefield.len() - 1 {
        return 0;
    }
    return count_mines_for_row(y, minefield[x + 1].as_bytes(), true);
}

fn count_mines_for_row(y: usize, row: &[u8], include_center: bool) -> u8 {
    let left = (y > 0 && is_mine(row[y - 1])) as u8;
    let right = (y < row.len() - 1 && is_mine(row[y + 1])) as u8;
    let center = (include_center && is_mine(row[y])) as u8;

    return left + center + right;
}

fn is_mine(tile: u8) -> bool {
    return tile == b'*';
}
