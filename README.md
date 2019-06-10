# Array2D

Array2D provides a statically-sized two-dimensional array. It is more
efficient and is easier to use than nested vectors, i.e. `Vec<Vec<T>>`.

This is beneficial when using a grid-like structure, which is common in
image processing, game boards, and other situations. This cannot be
used when rows or columns might have different lengths, as they are required
to all be the same length.

# How to use [`Array2D`]

## Creating an [`Array2D`]

An [`Array2D`] can be created by either pre-filling it with a repeated value
or by providing it with the data to create the array with. This can be done
by:
  - Providing the rows or the columns, which must all be the same size (see
    [`from_rows`] and [`from_columns`]).
  - Providing a "flat" slice of elements along with the dimensions, which
    must match the number of elements (see [`from_row_major`] and
    [`from_column_major`]).

## Accessing data from an [`Array2D`]

[`Array2D`] supports indexing using a tuple of `(usize, usize)` (which
panics on out-of-bounds accesses) or through the [`get`], [`get_mut`], and
[`set`] methods (which return an [`Option`] or a [`Result`] on out-of-bounds
accesses)

[`Array2D`] also supports several forms of iteration. You can iterate
through:
  - All of the elements, in either [row major or column major order] (see
    [`elements_row_major_iter`] and [`elements_column_major_iter`]).
  - Individual rows or columns (see [`row_iter`] and [`column_iter`]).
  - All rows or all columns (see [`rows_iter`] and [`columns_iter`]).

## Extracting all data from an [`Array2D`]

An [`Array2D`] can be converted back into a [`Vec`] through several
methods. You can extract the data as:
  - A [`Vec`] of rows or columns (see [`as_rows`] and [`as_columns`]).
  - A "flat" [`Vec`] of elements in either [row major or column major order]
    (see [`as_row_major`] and [`as_column_major`]).

# Examples

```rust
use array2d::Array2D;

pub fn main() {
    // Create an array filled with the same element.
    let prefilled = Array2D::fill_with(42, 2, 3);
    assert_eq!(prefilled.num_rows(), 2);
    assert_eq!(prefilled.num_columns(), 3);
    assert_eq!(prefilled[(0, 0)], 42);

    // Create an array from the given rows. You can also use columns
    // with the `from_columns` function
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let from_rows = Array2D::from_rows(&rows);
    assert_eq!(from_rows.num_rows(), 2);
    assert_eq!(from_rows.num_columns(), 3);
    assert_eq!(from_rows[(1, 1)], 5);

    // Create an array from a flat Vec of elements in row or column
    // major order.
    let column_major = vec![1, 4, 2, 5, 3, 6];
    let from_column_major =
        Array2D::from_column_major(&column_major, 2, 3);
    assert_eq!(from_column_major.num_rows(), 2);
    assert_eq!(from_column_major.num_columns(), 3);
    assert_eq!(from_column_major[(1, 1)], 5);

    // Implements `Eq` if the element type does.
    assert_eq!(from_rows, from_column_major);

    // Index into an array using a tuple of usize to access or alter
    // the array.
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows);
    array[(1, 1)] = 100;

    // Convert the array back into a nested Vec using `as_rows` or
    // `as_columns`.
    let array_rows = array.as_rows();
    assert_eq!(array_rows, vec![vec![1, 2, 3], vec![4, 100, 6]]);

    // Convert the array back into a flat Vec using `as_row_major` or
    // `as_column_major`.
    let array_column_major = array.as_column_major();
    assert_eq!(array_column_major, vec![1, 4, 2, 100, 3, 6]);

    // Iterate over a single row or column
    println!("First column:");
    for element in array.column_iter(0) {
        println!("{}", element);
    }

    // Iterate over all rows or columns.
    println!("All elements:");
    for row_iter in array.rows_iter() {
        for element in row_iter {
            print!("{} ", element);
        }
        println!();
    }
}
```

[`Array2D`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html
[`from_rows`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.from_rows
[`from_columns`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.from_columns
[`from_row_major`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.from_row_major
[`from_column_major`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.from_column_major
[`get`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.get
[`get_mut`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.get_mut
[`set`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.set
[`elements_row_major_iter`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.elements_row_major_iter
[`elements_column_major_iter`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.elements_column_major_iter
[`row_iter`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.row_iter
[`column_iter`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.column_iter
[`rows_iter`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.rows_iter
[`columns_iter`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.columns_iter
[`as_rows`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.as_rows
[`as_columns`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.as_columns
[`as_row_major`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.as_row_major
[`as_column_major`]: https://docs.rs/array2d/latest/array2d/struct.Array2D.html#method.as_column_major
[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[`Option`]: https://doc.rust-lang.org/std/option/
[`Result`]: https://doc.rust-lang.org/std/result/
[row major or column major order]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
