//! Array2D provides a fixed sized two-dimensional array. It is more efficient
//! and is easier to use than a vector of vectors, i.e. `Vec<Vec<T>>`.
//!
//! This is beneficial when using a grid-like structure, which is common in
//! image processing, game boards, and other situations. Array2D cannot be used
//! when rows or columns might have different lengths⁠—all rows and columns must
//! be the same length.
//!
//! # How to use [`Array2D`]
//!
//! ## Creating an [`Array2D`]
//!
//! An [`Array2D`] can be created in many different ways. These include:
//!   - Providing the rows or the columns, which must all be the same size (see
//!     [`from_rows`] and [`from_columns`]).
//!   - Providing a "flat" slice of elements in either [row major or column
//!     major order] along with the dimensions, which must match the number of
//!     elements in the slice (see [`from_row_major`] and
//!     [`from_column_major`]).
//!   - Providing a value to repeatedly put in every location (see
//!     [`filled_with`]).
//!   - Providing a generator function that is repeatedly called to produce
//!     values to fill the array (see [`filled_by_row_major`] and
//!     [`filled_by_column_major`]).
//!   - Providing an iterator that is used to produce values to fill the array
//!     (see [`from_iter_row_major`] and [`from_iter_column_major`]).
//!
//! ## Accessing data from an [`Array2D`]
//!
//! [`Array2D`] supports several forms of indexing:
//!   - Using the indexing syntax (square brackets) with a tuple of [`(usize,
//!     usize)`], which panics on out-of-bounds accesses.
//!   - Using the [`get`], [`get_mut`], and [`set`] methods, which return an
//!     [`Option`] or a [`Result`] on out-of-bounds accesses.
//!   - Using the row major or column major version of these methods,
//!     i.e. [`get_row_major`], [`get_mut_row_major`], [`set_row_major`],
//!     [`get_column_major`], [`get_mut_column_major`],
//!     [`set_column_major`]. These perform the same tasks as the non row/column
//!     major methods, but take one index instead of two.
//!
//! [`Array2D`] also supports several forms of iteration. You can iterate
//! through:
//!   - All the elements, in either [row major or column major order] (see
//!     [`elements_row_major_iter`] and [`elements_column_major_iter`]).
//!   - Individual rows or columns (see [`row_iter`] and [`column_iter`]).
//!   - All rows or all columns (see [`rows_iter`] and [`columns_iter`]).
//!
//! ## Extracting all data from an [`Array2D`]
//!
//! An [`Array2D`] can be converted back into a [`Vec`] through several
//! methods. You can extract the data as:
//!   - A [`Vec`] of rows or columns (see [`as_rows`] and [`as_columns`]).
//!   - A "flat" [`Vec`] of elements in either [row major or column major order]
//!     (see [`as_row_major`] and [`as_column_major`]).
//!
//! # Examples
//!
//! ```rust
//! use array2d::{Array2D, Error};
//!
//! pub fn main() -> Result<(), Error> {
//!     // Create an array filled with the same element.
//!     let prefilled = Array2D::filled_with( 2, 3,42);
//!     assert_eq!(prefilled.num_rows(), 2);
//!     assert_eq!(prefilled.num_columns(), 3);
//!     assert_eq!(prefilled[(0, 0)], 42);
//!
//!     // Create an array from the given rows. You can also use columns
//!     // with the `columns` function
//!     let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
//!     let from_rows = Array2D::from_rows(&rows)?;
//!     assert_eq!(from_rows.num_rows(), 2);
//!     assert_eq!(from_rows.num_columns(), 3);
//!     assert_eq!(from_rows[(1, 1)], 5);
//!
//!     // Create an array from a flat Vec of elements in row major or
//!     // column major order.
//!     let column_major = vec![1, 4, 2, 5, 3, 6];
//!     let from_column_major =
//!         Array2D::from_column_major( 2, 3,&column_major)?;
//!     assert_eq!(from_column_major.num_rows(), 2);
//!     assert_eq!(from_column_major.num_columns(), 3);
//!     assert_eq!(from_column_major[(1, 1)], 5);
//!
//!     // Implements `Eq` if the element type does.
//!     assert_eq!(from_rows, from_column_major);
//!
//!     // Index into an array using a tuple of usize to access or alter
//!     // the array.
//!     let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
//!     let mut array = Array2D::from_rows(&rows)?;
//!     array[(1, 1)] = 100;
//!
//!     // Convert the array back into a nested Vec using `as_rows` or
//!     // `as_columns`.
//!     let array_rows = array.as_rows();
//!     assert_eq!(array_rows, vec![vec![1, 2, 3], vec![4, 100, 6]]);
//!
//!     // Convert the array back into a flat Vec using `as_row_major` or
//!     // `as_column_major`.
//!     let array_column_major = array.as_column_major();
//!     assert_eq!(array_column_major, vec![1, 4, 2, 100, 3, 6]);
//!
//!     // Iterate over a single row or column
//!     println!("First column:");
//!     for element in array.column_iter(0)? {
//!         println!("{}", element);
//!     }
//!
//!     // Iterate over all rows or columns.
//!     println!("All elements:");
//!     for row_iter in array.rows_iter() {
//!         for element in row_iter {
//!             print!("{} ", element);
//!         }
//!         println!();
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! [`Array2D`]: struct.Array2D.html
//! [`from_rows`]: struct.Array2D.html#method.from_rows
//! [`from_columns`]: struct.Array2D.html#method.from_columns
//! [`from_row_major`]: struct.Array2D.html#method.from_row_major
//! [`from_column_major`]: struct.Array2D.html#method.from_column_major
//! [`filled_with`]: struct.Array2D.html#method.filled_with
//! [`filled_by_row_major`]: struct.Array2D.html#method.filled_by_row_major
//! [`filled_by_column_major`]: struct.Array2D.html#method.filled_by_column_major
//! [`from_iter_row_major`]: struct.Array2D.html#method.from_iter_row_major
//! [`from_iter_column_major`]: struct.Array2D.html#method.from_iter_column_major
//! [`get`]: struct.Array2D.html#method.get
//! [`get_mut`]: struct.Array2D.html#method.get_mut
//! [`set`]: struct.Array2D.html#method.set
//! [`get_row_major`]: struct.Array2D.html#method.get_row_major
//! [`get_mut_row_major`]: struct.Array2D.html#method.get_mut_row_major
//! [`set_row_major`]: struct.Array2D.html#method.set_row_major
//! [`get_column_major`]: struct.Array2D.html#method.get_column_major
//! [`get_mut_column_major`]: struct.Array2D.html#method.get_mut_column_major
//! [`set_column_major`]: struct.Array2D.html#method.set_column_major
//! [`elements_row_major_iter`]: struct.Array2D.html#method.elements_row_major_iter
//! [`elements_column_major_iter`]: struct.Array2D.html#method.elements_column_major_iter
//! [`row_iter`]: struct.Array2D.html#method.row_iter
//! [`column_iter`]: struct.Array2D.html#method.column_iter
//! [`rows_iter`]: struct.Array2D.html#method.rows_iter
//! [`columns_iter`]: struct.Array2D.html#method.columns_iter
//! [`as_rows`]: struct.Array2D.html#method.as_rows
//! [`as_columns`]: struct.Array2D.html#method.as_columns
//! [`as_row_major`]: struct.Array2D.html#method.as_row_major
//! [`as_column_major`]: struct.Array2D.html#method.as_column_major
//! [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
//! [`Option`]: https://doc.rust-lang.org/std/option/
//! [`Result`]: https://doc.rust-lang.org/std/result/
//! [`(usize, usize)`]: https://doc.rust-lang.org/std/primitive.usize.html
//! [row major or column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order

#![deny(missing_docs)]

use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A fixed sized two-dimensional array.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Array2D<T> {
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
    IndicesOutOfBounds(usize, usize),
    /// The given index in row or column major order was out of bounds.
    IndexOutOfBounds(usize),
    /// The dimensions given did not match the elements provided
    DimensionMismatch,
    /// There were not enough elements to fill the array.
    NotEnoughElements,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IndicesOutOfBounds(row, column) => {
                write!(f, "indices ({row}, {column}) out of bounds")
            }
            Error::IndexOutOfBounds(index) => write!(f, "index {index} out of bounds"),
            Error::DimensionMismatch => write!(f, "dimension mismatch"),
            Error::NotEnoughElements => write!(f, "not enough elements"),
        }
    }
}

impl std::error::Error for Error {}

impl<T> Array2D<T> {
    /// Creates a new [`Array2D`] from a slice of rows, each of which is a
    /// [`Vec`] of elements.
    ///
    /// Returns an error if the rows are not all the same size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// assert_eq!(array[(1, 2)], 6);
    /// assert_eq!(array.as_rows(), rows);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn from_rows(elements: &[Vec<T>]) -> Result<Self, Error>
    where
        T: Clone,
    {
        let row_len = elements.get(0).map(Vec::len).unwrap_or(0);
        if !elements.iter().all(|row| row.len() == row_len) {
            return Err(Error::DimensionMismatch);
        }
        Ok(Array2D {
            array: flatten(elements),
            num_rows: elements.len(),
            num_columns: row_len,
        })
    }

    /// Creates a new [`Array2D`] from a slice of columns, each of which
    /// contains a [`Vec`] of elements.
    ///
    /// Returns an error if the columns are not all the same size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let columns = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    /// let array = Array2D::from_columns(&columns)?;
    /// assert_eq!(array[(1, 2)], 6);
    /// assert_eq!(array.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn from_columns(elements: &[Vec<T>]) -> Result<Self, Error>
    where
        T: Clone,
    {
        let column_len = elements.get(0).map(Vec::len).unwrap_or(0);
        if !elements.iter().all(|column| column.len() == column_len) {
            return Err(Error::DimensionMismatch);
        }
        let num_rows = column_len;
        let num_columns = elements.len();
        let array = indices_row_major(num_rows, num_columns)
            .map(|(row, column)| elements[column][row].clone())
            .collect();
        Ok(Array2D {
            array,
            num_rows,
            num_columns,
        })
    }

    /// Creates a new [`Array2D`] from the given flat slice in [row major
    /// order].
    ///
    /// Returns an error if the number of elements in `elements` is not the
    /// product of `num_rows` and `num_columns`, i.e. the dimensions do not
    /// match.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let row_major = vec![1, 2, 3, 4, 5, 6];
    /// let array = Array2D::from_row_major( 2, 3,&row_major)?;
    /// assert_eq!(array[(1, 2)], 6);
    /// assert_eq!(array.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_row_major(
        num_rows: usize,
        num_columns: usize,
        elements: &[T],
    ) -> Result<Self, Error>
    where
        T: Clone,
    {
        let total_len = num_rows * num_columns;
        if total_len != elements.len() {
            return Err(Error::DimensionMismatch);
        }
        Ok(Array2D {
            array: elements.to_vec(),
            num_rows,
            num_columns,
        })
    }

    /// Creates a new [`Array2D`] from the given flat slice in [column major
    /// order].
    ///
    /// Return an error if the number of elements in `elements` is not the
    /// product of `num_rows` and `num_columns`, i.e. the dimensions do not
    /// match.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let column_major = vec![1, 4, 2, 5, 3, 6];
    /// let array = Array2D::from_column_major( 2, 3,&column_major)?;
    /// assert_eq!(array[(1, 2)], 6);
    /// assert_eq!(array.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_column_major(
        num_rows: usize,
        num_columns: usize,
        elements: &[T],
    ) -> Result<Self, Error>
    where
        T: Clone,
    {
        let total_len = num_rows * num_columns;
        if total_len != elements.len() {
            return Err(Error::DimensionMismatch);
        }
        let indices_row_major =
            (0..num_rows).flat_map(move |row| (0..num_columns).map(move |column| (row, column)));
        let array = indices_row_major
            .map(|(row, column)| {
                let index = column * num_rows + row;
                elements[index].clone()
            })
            .collect();
        Ok(Array2D {
            array,
            num_rows,
            num_columns,
        })
    }

    /// Creates a new [`Array2D`] with the specified number of rows and columns
    /// that contains `element` in every location.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// let array = Array2D::filled_with( 2, 3,42);
    /// assert_eq!(array.as_rows(), vec![vec![42, 42, 42], vec![42, 42, 42]]);
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    pub fn filled_with(num_rows: usize, num_columns: usize, element: T) -> Self
    where
        T: Clone,
    {
        let total_len = num_rows * num_columns;
        let array = vec![element; total_len];
        Array2D {
            array,
            num_rows,
            num_columns,
        }
    }

    #[deprecated(since = "0.2.0", note = "Renamed to filled_with")]
    /// Renamed to filled_with.
    pub fn fill_with(element: T, num_rows: usize, num_columns: usize) -> Self
    where
        T: Clone,
    {
        Array2D::filled_with(num_rows, num_columns, element)
    }

    /// Creates a new [`Array2D`] with the specified number of rows and columns
    /// and fills each element with the result of calling the given
    /// function. The function is called once for every location going in
    /// row major order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// let mut counter = 1;
    /// let increment = || {
    ///     let tmp = counter;
    ///     counter += 1;
    ///     tmp
    /// };
    /// let array = Array2D::filled_by_row_major( 2, 3,increment);
    /// assert_eq!(array.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    pub fn filled_by_row_major<F>(num_rows: usize, num_columns: usize, mut generator: F) -> Self
    where
        F: FnMut() -> T,
    {
        let total_len = num_rows * num_columns;
        let array = (0..total_len).map(|_| generator()).collect();
        Array2D {
            array,
            num_rows,
            num_columns,
        }
    }

    /// Creates a new [`Array2D`] with the specified number of rows and columns
    /// and fills each element with the result of calling the given
    /// function. The function is called once for every location going in
    /// column major order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// let mut counter = 1;
    /// let increment = || {
    ///     let tmp = counter;
    ///     counter += 1;
    ///     tmp
    /// };
    /// let array = Array2D::filled_by_column_major( 2, 3,increment);
    /// assert_eq!(array.as_columns(), vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    pub fn filled_by_column_major<F>(num_rows: usize, num_columns: usize, mut generator: F) -> Self
    where
        F: FnMut() -> T,
        T: Clone,
    {
        let total_len = num_rows * num_columns;
        let array_column_major = (0..total_len).map(|_| generator()).collect::<Vec<_>>();
        Array2D::from_column_major(num_rows, num_columns, &array_column_major)
            .expect("Filled by should never fail")
    }

    /// Creates a new [`Array2D`] with the specified number of rows and columns
    /// and fills each element with the elements produced from the provided
    /// iterator. If the iterator produces more than enough elements, the
    /// remaining are unused. Returns an error if the iterator does not produce
    /// enough elements.
    ///
    /// The elements are inserted into the array in [row major order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let iterator = 1..;
    /// let array = Array2D::from_iter_row_major( 2, 3,iterator)?;
    /// assert_eq!(array.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_iter_row_major<I>(
        num_rows: usize,
        num_columns: usize,
        iterator: I,
    ) -> Result<Self, Error>
    where
        I: Iterator<Item = T>,
    {
        let total_len = num_rows * num_columns;
        let array = iterator.take(total_len).collect::<Vec<_>>();
        if array.len() != total_len {
            return Err(Error::NotEnoughElements);
        }
        Ok(Array2D {
            array,
            num_rows,
            num_columns,
        })
    }

    /// Creates a new [`Array2D`] with the specified number of rows and columns
    /// and fills each element with the elements produced from the provided
    /// iterator. If the iterator produces more than enough elements, the
    /// remaining are unused. Returns an error if the iterator does not produce
    /// enough elements.
    ///
    /// The elements are inserted into the array in [column major order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let iterator = 1..;
    /// let array = Array2D::from_iter_column_major( 2, 3,iterator)?;
    /// assert_eq!(array.as_rows(), vec![vec![1, 3, 5], vec![2, 4, 6]]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_iter_column_major<I>(
        num_rows: usize,
        num_columns: usize,
        iterator: I,
    ) -> Result<Self, Error>
    where
        I: Iterator<Item = T>,
        T: Clone,
    {
        let total_len = num_rows * num_columns;
        let array_column_major = iterator.take(total_len).collect::<Vec<_>>();
        Array2D::from_column_major(num_rows, num_columns, &array_column_major)
            .map_err(|_| Error::NotEnoughElements)
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
    /// `num_columns`.
    pub fn num_elements(&self) -> usize {
        self.num_rows * self.num_columns
    }

    /// The number of elements in each row, i.e. the number of columns.
    pub fn row_len(&self) -> usize {
        self.num_columns
    }

    /// The number of elements in each column, i.e. the number of rows.
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
    /// # use array2d::{Array2D, Error};
    /// let array = Array2D::filled_with( 2, 3,42);
    /// assert_eq!(array.get(0, 0), Some(&42));
    /// assert_eq!(array.get(10, 10), None);
    /// ```
    ///
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.get_index(row, column).map(|index| &self.array[index])
    }

    /// Returns a reference to the element at the given index in row major
    /// order. Returns [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// assert_eq!(array.get_row_major(2), Some(&3));
    /// assert_eq!(array.get_row_major(4), Some(&5));
    /// assert_eq!(array.get_row_major(10), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get_row_major(&self, index: usize) -> Option<&T> {
        self.array.get(index)
    }

    /// Returns a reference to the element at the given index in column major
    /// order. Returns [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// assert_eq!(array.get_column_major(2), Some(&2));
    /// assert_eq!(array.get_column_major(4), Some(&3));
    /// assert_eq!(array.get_column_major(10), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get_column_major(&self, index: usize) -> Option<&T> {
        let column = dbg!(dbg!(index) / self.num_rows);
        let row = dbg!(index % self.num_rows);
        self.get(row, column)
    }

    /// Returns a mutable reference to the element at the given `row` and
    /// `column` if the index is in bounds (wrapped in [`Some`]). Returns
    /// [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// let mut array = Array2D::filled_with( 2, 3,42);
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

    /// Returns a mutable reference to the element at the given index in row
    /// major order. Returns [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut array = Array2D::from_rows(&rows)?;
    ///
    /// assert_eq!(array.get_mut_row_major(1), Some(&mut 2));
    /// assert_eq!(array.get_mut_row_major(10), None);
    ///
    /// array.get_mut_row_major(3).map(|x| *x = 100);
    /// assert_eq!(array.get(1, 0), Some(&100));
    ///
    /// array.get_mut_row_major(10).map(|x| *x = 200);
    /// assert_eq!(array.get(10, 10), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get_mut_row_major(&mut self, index: usize) -> Option<&mut T> {
        self.array.get_mut(index)
    }

    /// Returns a mutable reference to the element at the given index in row
    /// major order. Returns [`None`] if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let mut array = Array2D::from_rows(&rows)?;
    ///
    /// assert_eq!(array.get_mut_column_major(1), Some(&mut 4));
    /// assert_eq!(array.get_mut_column_major(10), None);
    ///
    /// array.get_mut_column_major(4).map(|x| *x = 100);
    /// assert_eq!(array.get(0, 2), Some(&100));
    ///
    /// array.get_mut_column_major(10).map(|x| *x = 200);
    /// assert_eq!(array.get(10, 10), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn get_mut_column_major(&mut self, index: usize) -> Option<&mut T> {
        let column = index / self.num_rows;
        let row = index % self.num_rows;
        self.get_mut(row, column)
    }

    /// Changes the element at given `row` and `column` to `element`. Returns
    /// [`Ok(())`] if the indices were in bounds and returns an [`Err`]
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// let mut array = Array2D::filled_with( 2, 3,42);
    ///
    /// let result = array.set(0, 0, 100);
    /// assert_eq!(result, Ok(()));
    /// assert_eq!(array.get(0, 0), Some(&100));
    ///
    /// let result = array.set(10, 20, 200);
    /// assert_eq!(result, Err(Error::IndicesOutOfBounds(10, 20)));
    /// ```
    ///
    /// [`Ok(())`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [array2d::Error]: enum.Error.html
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`array2d::Error`]: enum.Error.html
    pub fn set(&mut self, row: usize, column: usize, element: T) -> Result<(), Error> {
        self.get_mut(row, column)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndicesOutOfBounds(row, column))
    }

    /// Changes the element at the given `index` to `element`, in row major
    /// order. Returns [`Ok(())`] if the index is in bounds and returns an
    /// [`Err`] otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// let mut array = Array2D::filled_with( 2, 3,42);
    ///
    /// let result = array.set_row_major(4, 100);
    /// assert_eq!(result, Ok(()));
    /// assert_eq!(array.get(1, 1), Some(&100));
    ///
    /// let result = array.set_row_major(10, 200);
    /// assert_eq!(result, Err(Error::IndexOutOfBounds(10)));
    /// ```
    ///
    /// [`Ok(())`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [array2d::Error]: enum.Error.html
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`array2d::Error`]: enum.Error.html
    pub fn set_row_major(&mut self, index: usize, element: T) -> Result<(), Error> {
        self.get_mut_row_major(index)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndexOutOfBounds(index))
    }

    /// Changes the element at the given `index` to `element`, in column major
    /// order. Returns [`Ok(())`] if the index is in bounds and returns an
    /// [`Err`] otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// let mut array = Array2D::filled_with( 2, 3,42);
    ///
    /// let result = array.set_column_major(4, 100);
    /// assert_eq!(result, Ok(()));
    /// assert_eq!(array.get(0, 2), Some(&100));
    ///
    /// let result = array.set_column_major(10, 200);
    /// assert_eq!(result, Err(Error::IndexOutOfBounds(10)));
    /// ```
    ///
    /// [`Ok(())`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [array2d::Error]: enum.Error.html
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [`array2d::Error`]: enum.Error.html
    pub fn set_column_major(&mut self, index: usize, element: T) -> Result<(), Error> {
        self.get_mut_column_major(index)
            .map(|location| {
                *location = element;
            })
            .ok_or(Error::IndexOutOfBounds(index))
    }

    /// Returns an [`Iterator`] over references to all elements in [row major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let elements = vec![1, 2, 3, 4, 5, 6];
    /// let array = Array2D::from_rows(&rows)?;
    /// let row_major = array.elements_row_major_iter();
    /// assert_eq!(row_major.cloned().collect::<Vec<_>>(), elements);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn elements_row_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone {
        self.array.iter()
    }

    /// Returns an [`Iterator`] over references to all elements in [column major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let elements = vec![1, 4, 2, 5, 3, 6];
    /// let array = Array2D::from_rows(&rows)?;
    /// let column_major = array.elements_column_major_iter();
    /// assert_eq!(column_major.cloned().collect::<Vec<_>>(), elements);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn elements_column_major_iter(&self) -> impl DoubleEndedIterator<Item = &T> + Clone {
        self.indices_column_major().map(move |i| &self[i])
    }

    /// Returns an [`Iterator`] over references to all elements in the given
    /// row. Returns an error if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// let mut row_iter = array.row_iter(1)?;
    /// assert_eq!(row_iter.next(), Some(&4));
    /// assert_eq!(row_iter.next(), Some(&5));
    /// assert_eq!(row_iter.next(), Some(&6));
    /// assert_eq!(row_iter.next(), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    pub fn row_iter(
        &self,
        row_index: usize,
    ) -> Result<impl DoubleEndedIterator<Item = &T> + Clone, Error> {
        let start = self
            .get_index(row_index, 0)
            .ok_or(Error::IndicesOutOfBounds(row_index, 0))?;
        let end = start + self.row_len();
        Ok(self.array[start..end].iter())
    }

    /// Returns an [`Iterator`] over references to all elements in the given
    /// column. Returns an error if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// let mut column_iter = array.column_iter(1)?;
    /// assert_eq!(column_iter.next(), Some(&2));
    /// assert_eq!(column_iter.next(), Some(&5));
    /// assert_eq!(column_iter.next(), None);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    pub fn column_iter(
        &self,
        column_index: usize,
    ) -> Result<impl DoubleEndedIterator<Item = &T> + Clone, Error> {
        if column_index >= self.num_columns {
            return Err(Error::IndicesOutOfBounds(0, column_index));
        }
        Ok((0..self.column_len()).map(move |row_index| &self[(row_index, column_index)]))
    }

    /// Returns an [`Iterator`] over all rows. Each [`Item`] is itself another
    /// [`Iterator`] over references to the elements in that row.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// for row_iter in array.rows_iter() {
    ///     for element in row_iter {
    ///         print!("{} ", element);
    ///     }
    ///     println!();
    /// }
    ///
    /// let mut rows_iter = array.rows_iter();
    ///
    /// let mut first_row_iter = rows_iter.next().unwrap();
    /// assert_eq!(first_row_iter.next(), Some(&1));
    /// assert_eq!(first_row_iter.next(), Some(&2));
    /// assert_eq!(first_row_iter.next(), Some(&3));
    /// assert_eq!(first_row_iter.next(), None);
    ///
    /// let mut second_row_iter = rows_iter.next().unwrap();
    /// assert_eq!(second_row_iter.next(), Some(&4));
    /// assert_eq!(second_row_iter.next(), Some(&5));
    /// assert_eq!(second_row_iter.next(), Some(&6));
    /// assert_eq!(second_row_iter.next(), None);
    ///
    /// assert!(rows_iter.next().is_none());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [`Item`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item
    pub fn rows_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T> + Clone> + Clone {
        (0..self.num_rows()).map(move |row_index| {
            self.row_iter(row_index)
                .expect("rows_iter should never fail")
        })
    }

    /// Returns an [`Iterator`] over all columns. Each [`Item`] is itself
    /// another [`Iterator`] over references to the elements in that column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// for column_iter in array.columns_iter() {
    ///     for element in column_iter {
    ///         print!("{} ", element);
    ///     }
    ///     println!();
    /// }
    ///
    /// let mut columns_iter = array.columns_iter();
    ///
    /// let mut first_column_iter = columns_iter.next().unwrap();
    /// assert_eq!(first_column_iter.next(), Some(&1));
    /// assert_eq!(first_column_iter.next(), Some(&4));
    /// assert_eq!(first_column_iter.next(), None);
    ///
    /// let mut second_column_iter = columns_iter.next().unwrap();
    /// assert_eq!(second_column_iter.next(), Some(&2));
    /// assert_eq!(second_column_iter.next(), Some(&5));
    /// assert_eq!(second_column_iter.next(), None);
    ///
    /// let mut third_column_iter = columns_iter.next().unwrap();
    /// assert_eq!(third_column_iter.next(), Some(&3));
    /// assert_eq!(third_column_iter.next(), Some(&6));
    /// assert_eq!(third_column_iter.next(), None);
    ///
    /// assert!(columns_iter.next().is_none());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// [`Item`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item
    pub fn columns_iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T> + Clone> + Clone {
        (0..self.num_columns).map(move |column_index| {
            self.column_iter(column_index)
                .expect("columns_iter should never fail")
        })
    }

    /// Collects the [`Array2D`] into a [`Vec`] of rows, each of which contains
    /// a [`Vec`] of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// assert_eq!(array.as_rows(), rows);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn as_rows(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        self.rows_iter()
            .map(|row_iter| row_iter.cloned().collect())
            .collect()
    }

    /// Collects the [`Array2D`] into a [`Vec`] of columns, each of which
    /// contains a [`Vec`] of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let columns = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    /// let array = Array2D::from_columns(&columns)?;
    /// assert_eq!(array.as_columns(), columns);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn as_columns(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        self.columns_iter()
            .map(|column_iter| column_iter.cloned().collect())
            .collect()
    }

    /// Collects the [`Array2D`] into a [`Vec`] of elements in [row major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// assert_eq!(array.as_row_major(), vec![1, 2, 3, 4, 5, 6]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [row major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn as_row_major(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.elements_row_major_iter().cloned().collect()
    }

    /// Collects the [`Array2D`] into a [`Vec`] of elements in [column major
    /// order].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// assert_eq!(array.as_column_major(), vec![1, 4, 2, 5, 3, 6]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Array2D`]: struct.Array2D.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn as_column_major(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.elements_column_major_iter().cloned().collect()
    }

    /// Returns the indices of the array in row major order. Each index is a tuple of [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// let indices_row_major = array.indices_row_major().collect::<Vec<_>>();
    /// assert_eq!(
    ///     indices_row_major,
    ///     vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    pub fn indices_row_major(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
        indices_row_major(self.num_rows, self.num_columns)
    }

    /// Returns the indices of the array in column major order. Each index is a tuple of [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// let indices_column_major = array.indices_column_major().collect::<Vec<_>>();
    /// assert_eq!(
    ///     indices_column_major,
    ///     vec![(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)]
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    pub fn indices_column_major(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
        indices_column_major(self.num_rows, self.num_columns)
    }

    /// Iterate through the array in row major order along with the corresponding indices. Each
    /// index is a tuple of [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// let enumerate_row_major = array.enumerate_row_major().collect::<Vec<_>>();
    /// assert_eq!(
    ///     enumerate_row_major,
    ///     vec![
    ///         ((0, 0), &1),
    ///         ((0, 1), &2),
    ///         ((0, 2), &3),
    ///         ((1, 0), &4),
    ///         ((1, 1), &5),
    ///         ((1, 2), &6)
    ///     ]
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    pub fn enumerate_row_major(
        &self,
    ) -> impl DoubleEndedIterator<Item = ((usize, usize), &T)> + Clone {
        self.indices_row_major().map(move |i| (i, &self[i]))
    }

    /// Iterate through the array in column major order along with the corresponding indices. Each
    /// index is a tuple of [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// # fn main() -> Result<(), Error> {
    /// let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    /// let array = Array2D::from_rows(&rows)?;
    /// let enumerate_column_major = array.enumerate_column_major().collect::<Vec<_>>();
    /// assert_eq!(
    ///     enumerate_column_major,
    ///     vec![
    ///         ((0, 0), &1),
    ///         ((1, 0), &4),
    ///         ((0, 1), &2),
    ///         ((1, 1), &5),
    ///         ((0, 2), &3),
    ///         ((1, 2), &6)
    ///     ]
    /// );
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    pub fn enumerate_column_major(
        &self,
    ) -> impl DoubleEndedIterator<Item = ((usize, usize), &T)> + Clone {
        self.indices_column_major().map(move |i| (i, &self[i]))
    }

    fn get_index(&self, row: usize, column: usize) -> Option<usize> {
        if row < self.num_rows && column < self.num_columns {
            Some(row * self.row_len() + column)
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    /// Returns the element at the given indices, given as `(row, column)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// let array = Array2D::filled_with( 2, 3,42);
    /// assert_eq!(array[(0, 0)], 42);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the indices are out of bounds.
    ///
    /// ```rust,should_panic
    /// # use array2d::Array2D;
    /// let array = Array2D::filled_with( 2, 3,42);
    /// let element = array[(10, 10)];
    /// ```
    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        self.get(row, column)
            .unwrap_or_else(|| panic!("Index indices {}, {} out of bounds", row, column))
    }
}

impl<T> IndexMut<(usize, usize)> for Array2D<T> {
    /// Returns a mutable version of the element at the given indices, given as
    /// `(row, column)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use array2d::{Array2D, Error};
    /// let mut array = Array2D::filled_with( 2, 3,42);
    /// array[(0, 0)] = 100;
    /// assert_eq!(array[(0, 0)], 100);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the indices are out of bounds.
    ///
    /// ```rust,should_panic
    /// # use array2d::Array2D;
    /// let mut array = Array2D::filled_with( 2, 3,42);
    /// array[(10, 10)] = 7;
    /// ```
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, column)
            .unwrap_or_else(|| panic!("Index mut indices {}, {} out of bounds", row, column))
    }
}

fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flat_map(|row| row.clone()).collect()
}

fn indices_row_major(
    num_rows: usize,
    num_columns: usize,
) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
    (0..num_rows).flat_map(move |row| (0..num_columns).map(move |column| (row, column)))
}

fn indices_column_major(
    num_rows: usize,
    num_columns: usize,
) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone {
    (0..num_columns).flat_map(move |column| (0..num_rows).map(move |row| (row, column)))
}
