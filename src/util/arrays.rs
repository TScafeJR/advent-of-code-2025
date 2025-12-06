#![allow(dead_code)]
pub fn remove_element(arr: Vec<i64>, index: usize) -> (Option<i64>, Vec<i64>) {
    let mut arr = arr.clone();
    if index < arr.len() {
        let removed = arr.remove(index);
        (Some(removed), arr)
    } else {
        (None, arr)
    }
}

pub fn rotate_90_clockwise<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut rotated = vec![vec![matrix[0][0].clone(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[j][rows - i - 1] = matrix[i][j].clone();
        }
    }

    rotated
}

pub fn convert_str_to_vec(input: &str) -> Vec<char> {
    input.chars().collect()
}
