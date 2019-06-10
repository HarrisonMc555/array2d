//! Array2D provides a statically-sized two-dimensional array. It is more
//! efficient and can be easier to use than nested vectors (`Vec<Vec<T>>`). It
//! enforces that all rows and columns are the same length.

#![deny(missing_docs)]

use std::ops::{Index, IndexMut};

/// A statically-sized two-dimensional array.
#[derive(Debug, Eq, PartialEq)]
pub struct Array2D<T: Clone> {
    array: Vec<T>,
    num_rows: usize,
    num_columns: usize,
}

/// An error that can arise during the use of an [`Array2D`].
///
/// [`Array2D`]: struct.Array2D.html
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// The given indices were out of bounds.
    IndexOutOfBounds(usize, usize),
}

impl<T: Clone> Array2D<T> {
    /// Creates a new [`Array2D`] with the specified number of rows and columns
    /// that contains `element` in every location.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let array = Array2D::fill_with(42, 2, 3);
    /// assert_eq!(array.as_rows(), vec![vec![42, 42, 42], vec![42, 42, 42]]);
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    pub fn fill_with(element: T, num_rows: usize, num_columns: usize) -> Self {
        let total_len = num_rows * num_columns;
        let array = vec![element; total_len];
        Array2D {
            array,
            num_rows,
            num_columns,
        }
    }

    /// Creates a new [`Array2D`] from a [`Vec`] of rows, each of which is a
    /// [`Vec`] of elements.
    ///
    /// # Panics
    ///
    /// Panics if the rows are not all the same size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows);
    /// assert_eq!(array[(1, 2)], 6);
    /// assert_eq!(array.as_rows(), rows);
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn from_rows(elements: &[Vec<T>]) -> Self {
        let row_len = elements.get(0).map(Vec::len).unwrap_or(0);
        if !elements.iter().all(|row| row.len() == row_len) {
            panic!("Rows were not all {} elements long", row_len);
        }
        Array2D {
            array: flatten(elements),
            num_rows: elements.len(),
            num_columns: row_len,
        }
    }

    /// Creates a new [`Array2D`] from a [`Vec`] of columns, each of which
    /// contains a [`Vec`] of elements..
    ///
    /// # Panics
    ///
    /// Panics if the columns are not all the same size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let columns = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    /// let array = Array2D::from_columns(&columns);
    /// assert_eq!(array[(1, 2)], 6);
    /// assert_eq!(array.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn from_columns(elements: &[Vec<T>]) -> Self {
        let column_len = elements.get(0).map(Vec::len).unwrap_or(0);
        if !elements.iter().all(|column| column.len() == column_len) {
            panic!("Columns were not all {} elements long", column_len);
        }
        let num_rows = column_len;
        let num_columns = elements.len();
        let indices_row_major =
            (0..num_rows).flat_map(move |row| (0..num_columns).map(move |column| (row, column)));
        let array = indices_row_major
            .map(|(row, column)| elements[column][row].clone())
            .collect();
        Array2D {
            array,
            num_rows,
            num_columns,
        }
    }

    /// Creates a new [`Array2D`] from the given flat [`Vec`] in [row major
    /// order].
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in `elements` is not the product of
    /// `num_rows` and `num_columns`, i.e. the dimensions do not match.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let row_major = vec![1, 2, 3, 4, 5, 6];
    /// let array = Array2D::from_row_major(&row_major, 2, 3);
    /// assert_eq!(array[(1, 2)], 6);
    /// assert_eq!(array.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_row_major(elements: &[T], num_rows: usize, num_columns: usize) -> Self {
        let total_len = num_rows * num_columns;
        if total_len != elements.len() {
            panic!(
                "The number of elements ({}) did not match the expected size ({})",
                elements.len(),
                total_len
            );
        }
        Array2D {
            array: elements.to_vec(),
            num_rows,
            num_columns,
        }
    }

    /// Creates a new [`Array2D`] from the given flat [`Vec`] in [column major
    /// order].
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in `elements` is not the product of
    /// `num_rows` and `num_columns`, i.e. the dimensions do not match.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let column_major = vec![1, 4, 2, 5, 3, 6];
    /// let array = Array2D::from_column_major(&column_major, 2, 3);
    /// assert_eq!(array[(1, 2)], 6);
    /// assert_eq!(array.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_column_major(elements: &[T], num_rows: usize, num_columns: usize) -> Self {
        let total_len = num_rows * num_columns;
        if total_len != elements.len() {
            panic!(
                "The number of elements ({}) did not match the expected size ({})",
                elements.len(),
                total_len
            );
        }
        let indices_row_major =
            (0..num_rows).flat_map(move |row| (0..num_columns).map(move |column| (row, column)));
        let array = indices_row_major
            .map(|(row, column)| {
                let index = column * num_rows + row;
                elements[index].clone()
            })
            .collect();
        Array2D {
            array,
            num_rows,
            num_columns,
        }
    }

    /// The number of rows.
    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    /// The number of columns.
    pub fn num_columns(&self) -> usize {
        self.num_columns
    }

    /// The total number of elements, i.e. the product of `num_rows` and
    /// `num_columns`
    pub fn num_elements(&self) -> usize {
        self.num_rows * self.num_columns
    }

    /// The length of each row, i.e. `num_columns`.
    pub fn row_len(&self) -> usize {
        self.num_columns
    }

    /// The length of each column, i.e. `num_rows`.
    pub fn column_len(&self) -> usize {
        self.num_rows
    }

    /// Returns a reference to the element at the given `row` and `column` if the
    /// index is in bounds (wrapped in [`Some`]). Returns [`None`] if the index
    /// is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let array = Array2D::fill_with(42, 2, 3);
    /// assert_eq!(array.get(0, 0), Some(&42));
    /// assert_eq!(array.get(10, 10), None);
    /// ```
    ///
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.get_index(row, column).map(|index| &self.array[index])
    }

    /// Returns a mutable reference to the element at the given `row` and
    /// `column` if the index is in bounds (wrapped in [`Some`]). Returns
    /// [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let mut array = Array2D::fill_with(42, 2, 3);
    ///
    /// assert_eq!(array.get_mut(0, 0), Some(&mut 42));
    /// assert_eq!(array.get_mut(10, 10), None);
    ///
    /// array.get_mut(0, 0).map(|x| *x = 100);
    /// assert_eq!(array.get(0, 0), Some(&100));
    ///
    /// array.get_mut(10, 10).map(|x| *x = 200);
    /// assert_eq!(array.get(10, 10), None);
    /// ```
    ///
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.get_index(row, column)
            .map(move |index| &mut self.array[index])
    }

    /// Changes the element at given `row` and `column` to `element`. Returns
    /// [`Ok(())`] if the indices were in bounds and returns an
    /// [`Err`]`(`[`Error`]`)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// let mut array = Array2D::fill_with(42, 2, 3);
    ///
    /// let result = array.set(0, 0, 100);
    /// assert_eq!(result, Ok(()));
    /// assert_eq!(array.get(0, 0), Some(&100));
    ///
    /// let result = array.set(10, 20, 200);
    /// assert_eq!(result, Err(Error::IndexOutOfBounds(10, 20)));
    /// ```
    ///
    /// [`Ok(())`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [Err]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [array2d::Error]: enum.Error.html
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`array2d::Error`]: enum.Error.html
    pub fn set(&mut self, row: usize, column: usize, element: T) -> Result<(), Error> {
        self.get_index(row, column)
            .map(|index| self.array[index] = element)
            .ok_or_else(|| Error::IndexOutOfBounds(row, column))
    }

    /// Returns an [`Iterator`] over references to all elements in [row major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let elements = vec![1, 2, 3, 4, 5, 6];
    /// let array = Array2D::from_rows(&rows);
    /// let row_major = array.elements_row_major_iter();
    /// assert_eq!(row_major.cloned().collect::<Vec<_>>(), elements);
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn elements_row_major_iter(&self) -> impl Iterator<Item = &T> {
        self.array.iter()
    }

    /// Returns an [`Iterator`] over references to all elements in [column major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let elements = vec![1, 4, 2, 5, 3, 6];
    /// let array = Array2D::from_rows(&rows);
    /// let column_major = array.elements_column_major_iter();
    /// assert_eq!(column_major.cloned().collect::<Vec<_>>(), elements);
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn elements_column_major_iter(&self) -> impl Iterator<Item = &T> {
        (0..self.num_columns)
            .flat_map(move |column| (0..self.num_rows).map(move |row| &self[(row, column)]))
    }

    /// Returns an [`Iterator`] over references to all elements in the given
    /// row.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let row = vec![4, 5, 6];
    /// let array = Array2D::from_rows(&rows);
    /// let row_iter = array.row_iter(1);
    /// assert_eq!(row_iter.cloned().collect::<Vec<_>>(), row);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `row_index` is out of bounds.
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    pub fn row_iter(&self, row_index: usize) -> impl Iterator<Item = &T> {
        let start = self.get_index(row_index, 0).expect(&format!(
            "Row index, {}, was out of bounds (>= number of rows, {})",
            row_index, self.num_rows,
        ));
        let end = start + self.row_len();
        self.array[start..end].iter()
    }

    /// Returns an [`Iterator`] over references to all elements in the given
    /// column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let column = vec![2, 5];
    /// let array = Array2D::from_rows(&rows);
    /// let column_iter = array.column_iter(1);
    /// assert_eq!(column_iter.cloned().collect::<Vec<_>>(), column);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `column_index` is out of bounds.
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    pub fn column_iter(&self, column_index: usize) -> impl Iterator<Item = &T> {
        if column_index > self.num_columns {
            panic!(
                "Column index, {}, was out of bounds (>= number of columns, {})",
                column_index, self.num_columns,
            );
        }
        (0..self.column_len()).map(move |row_index| &self[(row_index, column_index)])
    }

    /// Returns an [`Iterator`] over all rows. Each [`Item`] is itself another
    /// [`Iterator`] over references to the elements in that row.
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [`Item`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item
    pub fn rows_iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.num_rows()).map(move |row_index| self.row_iter(row_index))
    }

    /// Returns an [`Iterator`] over all columns. Each [`Item`] is itself
    /// another [`Iterator`] over references to the elements in that column.
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [`Item`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item
    pub fn columns_iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.num_columns()).map(move |column_index| self.column_iter(column_index))
    }

    /// Collects the [`Array2D`] into a [`Vec`] of rows, each of which contains
    /// a [`Vec`] of elements.
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn as_rows(&self) -> Vec<Vec<T>> {
        self.rows_iter()
            .map(|row_iter| row_iter.cloned().collect())
            .collect()
    }

    /// Collects the [`Array2D`] into a [`Vec`] of columns, each of which
    /// contains a [`Vec`] of elements.
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn as_columns(&self) -> Vec<Vec<T>> {
        self.columns_iter()
            .map(|column_iter| column_iter.cloned().collect())
            .collect()
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

    /// Returns the element at the given indices, given as `(row, column)`.
    ///
    /// # Panics
    ///
    /// Panics if the indices are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let array = Array2D::fill_with(42, 2, 3);
    /// assert_eq!(array[(0, 0)], 42);
    /// ```
    fn index(&self, indices: (usize, usize)) -> &Self::Output {
        let (row, column) = indices;
        self.get(row, column).unwrap()
    }
}

impl<T: Clone> IndexMut<(usize, usize)> for Array2D<T> {
    /// Returns a mutable version of the element at the given indices, given as
    /// `(row, column)`.
    ///
    /// # Panics
    ///
    /// Panics if the indices are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::Array2D;
    /// let mut array = Array2D::fill_with(42, 2, 3);
    /// array[(0, 0)] = 100;
    /// assert_eq!(array[(0, 0)], 100);
    /// ```
    fn index_mut(&mut self, indices: (usize, usize)) -> &mut Self::Output {
        let (row, column) = indices;
        self.get_mut(row, column).unwrap()
    }
}

fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flat_map(|row| row.clone()).collect()
}
