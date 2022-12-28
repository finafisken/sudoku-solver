use crate::{Area3x3, Column, Coord, Grid, Row};

pub fn get_row_from_coord(loc: Coord, grid: Grid) -> Row {
    grid[loc.1]
}

pub fn get_col_from_coord(loc: Coord, grid: Grid) -> Column {
    grid.map(|row| [row[loc.0]])
}

pub fn get_3x3_from_coord(loc: Coord, grid: Grid) -> Area3x3 {
    let mut area = [[0_u8; 3]; 3];

    let (x_range, y_range) = match loc {
        (0..=2, 0..=2) => (0..3, 0..3),
        (0..=2, 3..=5) => (0..3, 3..6),
        (0..=2, 6..=8) => (0..3, 6..9),
        (3..=5, 0..=2) => (3..6, 0..3),
        (3..=5, 3..=5) => (3..6, 3..6),
        (3..=5, 6..=8) => (3..6, 6..9),
        (6..=8, 0..=2) => (6..9, 0..3),
        (6..=8, 3..=5) => (6..9, 3..6),
        (6..=8, 6..=8) => (6..9, 6..9),
        _ => panic!("bad range"),
    };

    for (y, row) in grid[y_range].iter().enumerate() {
        for (x, val) in row[x_range.to_owned()].iter().enumerate() {
            area[y][x] = *val;
        }
    }

    area
}

#[test]
fn test_get_row() {
    let grid = [
        [6, 1, 5, 7, 2, 4, 8, 3, 9],
        [4, 8, 7, 3, 9, 5, 1, 6, 2],
        [9, 2, 3, 1, 8, 6, 5, 7, 4],
        [5, 9, 8, 4, 3, 2, 7, 1, 6],
        [1, 3, 6, 8, 7, 9, 2, 4, 5],
        [2, 7, 4, 6, 5, 1, 9, 8, 3],
        [8, 4, 9, 5, 1, 3, 6, 2, 7],
        [7, 6, 2, 9, 4, 8, 3, 5, 1],
        [3, 5, 1, 2, 6, 7, 4, 9, 8],
    ];

    assert_eq!(
        get_row_from_coord((7, 2), grid),
        [9, 2, 3, 1, 8, 6, 5, 7, 4]
    );
    assert_eq!(
        get_row_from_coord((0, 8), grid),
        [3, 5, 1, 2, 6, 7, 4, 9, 8]
    );
    assert_eq!(
        get_row_from_coord((1, 1), grid),
        [4, 8, 7, 3, 9, 5, 1, 6, 2]
    );
}

#[test]
fn test_get_col() {
    let grid = [
        [6, 1, 5, 7, 2, 4, 8, 3, 9],
        [4, 8, 7, 3, 9, 5, 1, 6, 2],
        [9, 2, 3, 1, 8, 6, 5, 7, 4],
        [5, 9, 8, 4, 3, 2, 7, 1, 6],
        [1, 3, 6, 8, 7, 9, 2, 4, 5],
        [2, 7, 4, 6, 5, 1, 9, 8, 3],
        [8, 4, 9, 5, 1, 3, 6, 2, 7],
        [7, 6, 2, 9, 4, 8, 3, 5, 1],
        [3, 5, 1, 2, 6, 7, 4, 9, 8],
    ];

    assert_eq!(
        get_col_from_coord((7, 2), grid),
        [[3], [6], [7], [1], [4], [8], [2], [5], [9]]
    );
    assert_eq!(
        get_col_from_coord((0, 8), grid),
        [[6], [4], [9], [5], [1], [2], [8], [7], [3]]
    );
    assert_eq!(
        get_col_from_coord((1, 1), grid),
        [[1], [8], [2], [9], [3], [7], [4], [6], [5]]
    );
}

#[test]
fn test_get_3x3() {
    let grid = [
        [6, 1, 5, 7, 2, 4, 8, 3, 9],
        [4, 8, 7, 3, 9, 5, 1, 6, 2],
        [9, 2, 3, 1, 8, 6, 5, 7, 4],
        [5, 9, 8, 4, 3, 2, 7, 1, 6],
        [1, 3, 6, 8, 7, 9, 2, 4, 5],
        [2, 7, 4, 6, 5, 1, 9, 8, 3],
        [8, 4, 9, 5, 1, 3, 6, 2, 7],
        [7, 6, 2, 9, 4, 8, 3, 5, 1],
        [3, 5, 1, 2, 6, 7, 4, 9, 8],
    ];

    assert_eq!(
        get_3x3_from_coord((7, 2), grid),
        [[8, 3, 9], [1, 6, 2], [5, 7, 4]]
    );
    assert_eq!(
        get_3x3_from_coord((0, 8), grid),
        [[8, 4, 9], [7, 6, 2], [3, 5, 1]]
    );
    assert_eq!(
        get_3x3_from_coord((1, 1), grid),
        [[6, 1, 5], [4, 8, 7], [9, 2, 3]]
    );
}
