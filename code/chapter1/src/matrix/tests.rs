use super::*;

#[test]
fn transpose() {
  let matrix: Matrix<usize> =
    Matrix::from(vec![vec![1_usize, 0, 0], vec![1, 0, 0], vec![1, 0, 0]]);
  assert_eq!(matrix.transpose(), vec![vec![1, 1, 1], vec![0, 0, 0], vec![
    0, 0, 0
  ]]);
}

#[cfg(miri)]
#[test]
fn transpose_mut() {
  let matrix: Matrix<usize> =
    Matrix::from(vec![vec![1_usize, 0, 0], vec![1, 0, 0], vec![1, 0, 0]]);
  assert_eq!(matrix.transpose(), vec![vec![1, 1, 1], vec![0, 0, 0], vec![
    0, 0, 0
  ]]);
}
