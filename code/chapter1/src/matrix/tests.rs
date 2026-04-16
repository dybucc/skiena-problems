use super::*;

#[test]
fn transpose() {
    let matrix = Matrix::from_iter([[1, 2, 3], [3, 2, 1]]);

    assert_eq!(matrix.transpose(), [[1, 3], [2, 2], [3, 1]]);
}

#[cfg(miri)]
#[test]
fn transpose_mut() {
    #![expect(unused, reason = "It's meant for Miri to check if things are right.")]

    let mut matrix = Matrix::from_iter([[1, 2, 3], [3, 2, 1]]);
    let mut transpose = matrix.transpose_mut();
}
