use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::matrix::Matrix;

#[derive(Debug)]
pub struct BorrowedMatrix<'a, T> {
    pub inn: Matrix<&'a T>,
    pub _marker: PhantomData<&'a Matrix<T>>,
}

#[derive(Debug)]
pub struct ExclusiveMatrix<'a, T> {
    pub inn: Matrix<&'a mut T>,
    pub _marker: PhantomData<&'a Matrix<T>>,
}

impl<'a, T> Deref for BorrowedMatrix<'a, T> {
    type Target = Matrix<&'a T>;

    fn deref(&self) -> &Self::Target {
        &self.inn
    }
}

impl<'a, T> Deref for ExclusiveMatrix<'a, T> {
    type Target = Matrix<&'a mut T>;

    fn deref(&self) -> &Self::Target {
        &self.inn
    }
}

impl<T> DerefMut for ExclusiveMatrix<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inn
    }
}
