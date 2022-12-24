mod validation;

use validation::{is_3x3_valid, is_col_valid, is_row_valid};

type Coord = (u8, u8);
type Grid = [[u8; 9]; 9];
type Row = [u8; 9];
type Column = [[u8; 1]; 9];
type Area_3x3 = [[u8; 3]; 3];

fn main() {
    // assert!(is_row_valid(&[6, 1, 5, 7, 2, 4, 8, 2, 9]));
    let mut puzzle = [
        [6, 0, 5, 7, 2, 0, 0, 3, 9],
        [4, 0, 0, 0, 0, 5, 1, 0, 0],
        [0, 2, 0, 1, 0, 0, 0, 0, 4],
        [0, 9, 0, 0, 3, 0, 7, 0, 6],
        [1, 0, 0, 8, 0, 9, 0, 0, 5],
        [2, 0, 4, 0, 5, 0, 0, 8, 0],
        [8, 0, 0, 0, 0, 3, 0, 2, 0],
        [0, 0, 2, 9, 0, 0, 0, 0, 1],
        [3, 5, 0, 0, 6, 7, 4, 0, 8],
    ];
    sudoku(&mut puzzle);
}

fn sudoku(puzzle: &mut Grid) {
    // implement backtracking algo

    // 1. find empty space from top -> bot, left -> right
    // 2. insert candidate number
    // 3. validate puzzle, if ok go step one, if bad backtrack step two and increment candidate
    // once no empty spaces puzzle should be solved

    let mut to_fill: Vec<Coord> = Vec::new();

    for (y, row) in puzzle.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            // println!("({}, {}): {}", x, y, val);
            if val == &0_u8 {
                to_fill.push((x as u8, y as u8));
            }
        }
    }

    // reverse order for easy pop/push working from top left
    to_fill.reverse();

    while !to_fill.is_empty() {
        let location = to_fill.pop();

        let mut candidate = 1;

        // println!("{:?}", to_fill);
    }
}

fn get_row_from_coord(loc: Coord, grid: Grid) -> Row {
    todo!()
}

fn get_col_from_coord(loc: Coord, grid: Grid) -> Column {
    todo!()
}

fn get_3x3_from_coord(loc: Coord, grid: Grid) -> Area_3x3 {
    todo!()
}

// #[test]
// fn puzzle1() {
//     let mut puzzle = [
//         [6, 0, 5, 7, 2, 0, 0, 3, 9],
//         [4, 0, 0, 0, 0, 5, 1, 0, 0],
//         [0, 2, 0, 1, 0, 0, 0, 0, 4],
//         [0, 9, 0, 0, 3, 0, 7, 0, 6],
//         [1, 0, 0, 8, 0, 9, 0, 0, 5],
//         [2, 0, 4, 0, 5, 0, 0, 8, 0],
//         [8, 0, 0, 0, 0, 3, 0, 2, 0],
//         [0, 0, 2, 9, 0, 0, 0, 0, 1],
//         [3, 5, 0, 0, 6, 7, 4, 0, 8],
//     ];
//     let solution = [
//         [6, 1, 5, 7, 2, 4, 8, 3, 9],
//         [4, 8, 7, 3, 9, 5, 1, 6, 2],
//         [9, 2, 3, 1, 8, 6, 5, 7, 4],
//         [5, 9, 8, 4, 3, 2, 7, 1, 6],
//         [1, 3, 6, 8, 7, 9, 2, 4, 5],
//         [2, 7, 4, 6, 5, 1, 9, 8, 3],
//         [8, 4, 9, 5, 1, 3, 6, 2, 7],
//         [7, 6, 2, 9, 4, 8, 3, 5, 1],
//         [3, 5, 1, 2, 6, 7, 4, 9, 8],
//     ];

//     sudoku(&mut puzzle);
//     assert_eq!(
//         puzzle, solution,
//         "\nYour solution (left) did not match the correct solution (right)"
//     );
// }

// #[test]
// fn puzzle2() {
//     let mut puzzle = [
//         [0, 0, 8, 0, 3, 0, 5, 4, 0],
//         [3, 0, 0, 4, 0, 7, 9, 0, 0],
//         [4, 1, 0, 0, 0, 8, 0, 0, 2],
//         [0, 4, 3, 5, 0, 2, 0, 6, 0],
//         [5, 0, 0, 0, 0, 0, 0, 0, 8],
//         [0, 6, 0, 3, 0, 9, 4, 1, 0],
//         [1, 0, 0, 8, 0, 0, 0, 2, 7],
//         [0, 0, 5, 6, 0, 3, 0, 0, 4],
//         [0, 2, 9, 0, 7, 0, 8, 0, 0],
//     ];
//     let solution = [
//         [9, 7, 8, 2, 3, 1, 5, 4, 6],
//         [3, 5, 2, 4, 6, 7, 9, 8, 1],
//         [4, 1, 6, 9, 5, 8, 3, 7, 2],
//         [8, 4, 3, 5, 1, 2, 7, 6, 9],
//         [5, 9, 1, 7, 4, 6, 2, 3, 8],
//         [2, 6, 7, 3, 8, 9, 4, 1, 5],
//         [1, 3, 4, 8, 9, 5, 6, 2, 7],
//         [7, 8, 5, 6, 2, 3, 1, 9, 4],
//         [6, 2, 9, 1, 7, 4, 8, 5, 3],
//     ];

//     sudoku(&mut puzzle);
//     assert_eq!(
//         puzzle, solution,
//         "\nYour solution (left) did not match the correct solution (right)"
//     );
// }
// #[test]
// fn puzzle3() {
//     let mut puzzle = [
//         [5, 3, 0, 0, 7, 0, 0, 0, 0],
//         [6, 0, 0, 1, 9, 5, 0, 0, 0],
//         [0, 9, 8, 0, 0, 0, 0, 6, 0],
//         [8, 0, 0, 0, 6, 0, 0, 0, 3],
//         [4, 0, 0, 8, 0, 3, 0, 0, 1],
//         [7, 0, 0, 0, 2, 0, 0, 0, 6],
//         [0, 6, 0, 0, 0, 0, 2, 8, 0],
//         [0, 0, 0, 4, 1, 9, 0, 0, 5],
//         [0, 0, 0, 0, 8, 0, 0, 7, 9],
//     ];

//     let solution = [
//         [5, 3, 4, 6, 7, 8, 9, 1, 2],
//         [6, 7, 2, 1, 9, 5, 3, 4, 8],
//         [1, 9, 8, 3, 4, 2, 5, 6, 7],
//         [8, 5, 9, 7, 6, 1, 4, 2, 3],
//         [4, 2, 6, 8, 5, 3, 7, 9, 1],
//         [7, 1, 3, 9, 2, 4, 8, 5, 6],
//         [9, 6, 1, 5, 3, 7, 2, 8, 4],
//         [2, 8, 7, 4, 1, 9, 6, 3, 5],
//         [3, 4, 5, 2, 8, 6, 1, 7, 9],
//     ];

//     sudoku(&mut puzzle);

//     assert_eq!(
//         puzzle, solution,
//         "\nYour solution (left) did not match the correct solution (right)"
//     );
// }
