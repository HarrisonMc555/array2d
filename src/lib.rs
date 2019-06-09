use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq)]
pub struct Array2D<T: Clone> {
    array: Vec<T>,
    num_rows: usize,
    num_columns: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    RowsNotAllSameSize,
    ColumnsNotAllSameSize,
    DimensionsDoNotMatchSize,
    IndexOutOfBounds(usize, usize),
}

impl<T: Clone> Array2D<T> {
    pub fn fill_with(value: T, num_rows: usize, num_columns: usize) -> Self {
        let total_len = num_rows * num_columns;
        let array = vec![value; total_len];
        Array2D {
            array,
            num_rows,
            num_columns,
        }
    }

    pub fn from_rows(values: &[Vec<T>]) -> Result<Self, Error> {
        let row_len = values.get(0).map(Vec::len).unwrap_or(0);
        if !values.iter().all(|row| row.len() == row_len) {
            return Err(Error::RowsNotAllSameSize);
        }
        Ok(Array2D {
            array: flatten(values),
            num_rows: values.len(),
            num_columns: row_len,
        })
    }

    pub fn from_columns(values: &[Vec<T>]) -> Result<Self, Error> {
        let column_len = values.get(0).map(Vec::len).unwrap_or(0);
        if !values.iter().all(|column| column.len() == column_len) {
            return Err(Error::ColumnsNotAllSameSize);
        }
        Ok(Array2D {
            array: flatten(values),
            num_rows: column_len,
            num_columns: values.len(),
        })
    }

    pub fn from_row_major(
        values: &[T],
        num_rows: usize,
        num_columns: usize,
    ) -> Result<Self, Error> {
        let total_len = num_rows * num_columns;
        if total_len != values.len() {
            return Err(Error::DimensionsDoNotMatchSize);
        }
        Ok(Array2D {
            array: values.to_vec(),
            num_rows,
            num_columns,
        })
    }

    pub fn from_column_major(
        values: &[T],
        num_rows: usize,
        num_columns: usize,
    ) -> Result<Self, Error> {
        let total_len = num_rows * num_columns;
        if total_len != values.len() {
            return Err(Error::DimensionsDoNotMatchSize);
        }
        let indices_row_major =
            (0..num_rows).flat_map(move |row| (0..num_columns).map(move |column| (row, column)));
        let array = indices_row_major
            .map(|(row, column)| {
                let index = column * num_rows + row;
                values[index].clone()
            })
            .collect();
        Ok(Array2D {
            array,
            num_rows,
            num_columns,
        })
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_columns(&self) -> usize {
        self.num_columns
    }

    pub fn num_items(&self) -> usize {
        self.num_rows * self.num_columns
    }

    pub fn row_len(&self) -> usize {
        self.num_columns
    }

    pub fn column_len(&self) -> usize {
        self.num_rows
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.get_index(row, column).map(|index| &self.array[index])
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.get_index(row, column)
            .map(move |index| &mut self.array[index])
    }

    pub fn set(&mut self, row: usize, column: usize, value: T) -> Result<(), Error> {
        self.get_index(row, column)
            .map(|index| self.array[index] = value)
            .ok_or_else(|| Error::IndexOutOfBounds(row, column))
    }

    pub fn iter_items_row_major(&self) -> impl Iterator<Item = &T> {
        self.array.iter()
    }

    pub fn iter_items_column_major(&self) -> impl Iterator<Item = &T> {
        (0..self.num_columns)
            .flat_map(move |column| (0..self.num_rows).map(move |row| &self[(row, column)]))
    }

    pub fn iter_row(&self, row_index: usize) -> Option<impl Iterator<Item = &T>> {
        let start = self.get_index(row_index, 0)?;
        let end = start + self.row_len();
        Some(self.array[start..end].iter())
    }

    pub fn iter_column(&self, column_index: usize) -> Option<impl Iterator<Item = &T>> {
        if column_index < self.num_columns {
            Some((0..self.column_len()).map(move |row_index| &self[(row_index, column_index)]))
        } else {
            None
        }
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.num_rows()).map(move |row_index| self.iter_row(row_index).unwrap())
    }

    pub fn iter_columns(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.num_columns()).map(move |column_index| self.iter_column(column_index).unwrap())
    }

    fn get_index(&self, row: usize, column: usize) -> Option<usize> {
        if row < self.num_rows && column < self.num_columns {
            Some(row * self.row_len() + column)
        } else {
            None
        }
    }
}

impl<T: Clone> Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, indices: (usize, usize)) -> &Self::Output {
        let (row, column) = indices;
        self.get(row, column).unwrap()
    }
}

impl<T: Clone> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, indices: (usize, usize)) -> &mut Self::Output {
        let (row, column) = indices;
        self.get_mut(row, column).unwrap()
    }
}

fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flat_map(|row| row.clone()).collect()
}
