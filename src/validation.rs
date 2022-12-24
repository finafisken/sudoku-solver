// rows are valid if unique numbers from 1-9 or empty spaces (0)
pub fn is_row_valid(row: &[u8; 9]) -> bool {
    for (i, x) in row.iter().enumerate() {
        if x != &0_u8 && row[(i + 1)..row.len()].contains(x) {
            return false;
        }
    }

    true
}

// cols are valid if unique numbers from 1-9 or empty spaces (0)
pub fn is_col_valid(col: &[[u8; 1]; 9]) -> bool {
    for (i, y) in col.iter().enumerate() {
        if y != &[0_u8] && col[(i + 1)..col.len()].contains(y) {
            return false;
        }
    }

    true
}

// 3x3s are valid if unique numbers from 1-9 or empty spaces (0)
pub fn is_3x3_valid(area: &[[u8; 3]; 3]) -> bool {
    let mut seen = Vec::new();
    for y in area {
        for x in y {
            if x != &0_u8 && seen.contains(x) {
                return false;
            }
            seen.push(x.to_owned());
        }
    }

    true
}

#[test]
fn test_is_row_valid() {
    let valid_row1 = [6, 1, 5, 7, 2, 4, 8, 3, 9];
    let valid_row2 = [2, 7, 4, 6, 5, 1, 9, 8, 3];
    let invalid_row1 = [2, 7, 1, 6, 5, 1, 9, 8, 3];
    let invalid_row2 = [6, 1, 5, 7, 2, 4, 8, 9, 9];
    let invalid_row3 = [1, 1, 1, 6, 5, 4, 9, 8, 3];

    assert!(is_row_valid(&valid_row1));
    assert!(is_row_valid(&valid_row2));
    assert!(!is_row_valid(&invalid_row1));
    assert!(!is_row_valid(&invalid_row2));
    assert!(!is_row_valid(&invalid_row3));
}

#[test]
fn test_is_col_valid() {
    let valid_col1 = [[6], [4], [9], [5], [1], [2], [8], [7], [3]];
    let valid_col2 = [[9], [3], [4], [8], [5], [2], [1], [7], [6]];
    let invalid_col1 = [[6], [4], [9], [5], [1], [3], [8], [7], [3]];
    let invalid_col2 = [[1], [1], [4], [8], [5], [2], [1], [7], [6]];
    let invalid_col3 = [[9], [3], [4], [8], [5], [2], [1], [7], [9]];

    assert!(is_col_valid(&valid_col1));
    assert!(is_col_valid(&valid_col2));
    assert!(!is_col_valid(&invalid_col1));
    assert!(!is_col_valid(&invalid_col2));
    assert!(!is_col_valid(&invalid_col3));
}

#[test]
fn test_is_3x3_valid() {
    let valid_3x3_1 = [[6, 1, 5], [4, 8, 7], [9, 2, 3]];
    let valid_3x3_2 = [[8, 3, 9], [1, 6, 2], [5, 7, 4]];
    let invalid_3x3_1 = [[8, 3, 9], [1, 6, 2], [5, 7, 1]];
    let invalid_3x3_2 = [[8, 3, 9], [1, 6, 2], [9, 7, 4]];
    let invalid_3x3_3 = [[6, 1, 5], [4, 8, 4], [9, 2, 3]];

    assert!(is_3x3_valid(&valid_3x3_1));
    assert!(is_3x3_valid(&valid_3x3_2));
    assert!(!is_3x3_valid(&invalid_3x3_1));
    assert!(!is_3x3_valid(&invalid_3x3_2));
    assert!(!is_3x3_valid(&invalid_3x3_3));
}
