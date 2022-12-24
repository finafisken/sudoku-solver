fn main() {
    println!("Hello, world!");
}

fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    todo!()
}

fn check_row(row: &[u8; 9]) -> bool {
    // let mut seen = Vec::new();
    // for x in row {
    //     if seen.contains(x) {
    //         return false;
    //     }
    //     seen.push(x.to_owned());
    // }

    for (i, x) in row.iter().enumerate() {
        if row[(i + 1)..row.len()].contains(x) {
            return false;
        }
    }

    true
}

fn check_col(col: &[[u8; 1]; 9]) -> bool {
    // let mut seen = Vec::new();
    // for [y] in col {
    //     if seen.contains(y) {
    //         return false;
    //     }
    //     seen.push(y.to_owned());
    // }

    for (i, y) in col.iter().enumerate() {
        if col[(i + 1)..col.len()].contains(y) {
            return false;
        }
    }

    true
}

#[test]
fn test_check_row() {
    let valid_row1 = [6, 1, 5, 7, 2, 4, 8, 3, 9];
    let valid_row2 = [2, 7, 4, 6, 5, 1, 9, 8, 3];
    let invalid_row1 = [2, 7, 1, 6, 5, 1, 9, 8, 3];
    let invalid_row2 = [6, 1, 5, 7, 2, 4, 8, 9, 9];
    let invalid_row3 = [1, 1, 1, 6, 5, 4, 9, 8, 3];

    assert!(check_row(&valid_row1));
    assert!(check_row(&valid_row2));
    assert!(!check_row(&invalid_row1));
    assert!(!check_row(&invalid_row2));
    assert!(!check_row(&invalid_row3));
}

#[test]
fn test_check_col() {
    let valid_col1 = [[6], [4], [9], [5], [1], [2], [8], [7], [3]];
    let valid_col2 = [[9], [3], [4], [8], [5], [2], [1], [7], [6]];
    let invalid_col1 = [[6], [4], [9], [5], [1], [3], [8], [7], [3]];
    let invalid_col2 = [[1], [1], [4], [8], [5], [2], [1], [7], [6]];
    let invalid_col3 = [[9], [3], [4], [8], [5], [2], [1], [7], [9]];

    assert!(check_col(&valid_col1));
    assert!(check_col(&valid_col2));
    assert!(!check_col(&invalid_col1));
    assert!(!check_col(&invalid_col2));
    assert!(!check_col(&invalid_col3));
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
