use array2d::{Array2D, Error};

////////////////////////////////////////////////////////////////////////////////
// Normal Operation ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_from_rows() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    assert_eq!(array.as_rows(), rows);
    Ok(())
}

#[test]
fn test_from_columns() -> Result<(), Error> {
    let columns = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    let array = Array2D::from_columns(&columns)?;
    assert_eq!(array.as_columns(), columns);
    Ok(())
}

#[test]
fn test_from_row_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let row_major = vec![1, 2, 3, 4, 5, 6];
    let num_rows = 2;
    let num_columns = 3;
    let array = Array2D::from_row_major(&row_major, num_rows, num_columns)?;
    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, element) in row.iter().enumerate() {
            assert_eq!(array.get(row_index, column_index), Some(element));
        }
    }
    Ok(())
}

#[test]
fn test_from_column_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let column_major = vec![1, 4, 2, 5, 3, 6];
    let num_rows = 2;
    let num_columns = 3;
    let array = Array2D::from_column_major(&column_major, num_rows, num_columns)?;
    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, element) in row.iter().enumerate() {
            assert_eq!(array.get(row_index, column_index), Some(element));
        }
    }
    Ok(())
}

#[test]
fn test_filled_with() -> Result<(), Error> {
    let element = 7;
    let array = Array2D::filled_with(element, 4, 5);
    assert_eq!(array.num_rows(), 4);
    assert_eq!(array.num_columns(), 5);
    assert_eq!(array.num_elements(), 20);
    for element in array.elements_row_major_iter() {
        assert_eq!(element, &7);
    }
    for element in array.elements_column_major_iter() {
        assert_eq!(element, &7);
    }
    Ok(())
}

#[test]
fn test_filled_by_row_major() -> Result<(), Error> {
    let mut counter = 1;
    let increment = || {
        let tmp = counter;
        counter += 1;
        tmp
    };
    let array = Array2D::filled_by_row_major(increment, 2, 3);
    assert_eq!(array.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    Ok(())
}

#[test]
fn test_filled_by_column_major() -> Result<(), Error> {
    let mut counter = 1;
    let increment = || {
        let tmp = counter;
        counter += 1;
        tmp
    };
    let array = Array2D::filled_by_column_major(increment, 2, 3);
    assert_eq!(array.as_columns(), vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    Ok(())
}

#[test]
fn test_from_iter_row_major() -> Result<(), Error> {
    let array = Array2D::from_iter_row_major(1.., 2, 3)?;
    assert_eq!(array.as_rows(), vec![vec![1, 2, 3], vec![4, 5, 6]]);
    Ok(())
}

#[test]
fn test_from_iter_column_major() -> Result<(), Error> {
    let array = Array2D::from_iter_column_major(1.., 2, 3)?;
    assert_eq!(array.as_columns(), vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    Ok(())
}

#[test]
fn test_dimensions() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    assert_eq!(array.num_rows(), 2);
    assert_eq!(array.num_columns(), 3);
    assert_eq!(array.row_len(), 3);
    assert_eq!(array.column_len(), 2);
    Ok(())
}

#[test]
fn test_get() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            assert_eq!(array.get(row, column), Some(&rows[row][column]));
        }
    }
    Ok(())
}

#[test]
fn test_get_row_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    assert_eq!(array.get_row_major(0), Some(&1));
    assert_eq!(array.get_row_major(1), Some(&2));
    assert_eq!(array.get_row_major(2), Some(&3));
    assert_eq!(array.get_row_major(3), Some(&4));
    assert_eq!(array.get_row_major(4), Some(&5));
    assert_eq!(array.get_row_major(5), Some(&6));
    assert_eq!(array.get_row_major(6), None);
    Ok(())
}

#[test]
fn test_get_column_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    assert_eq!(array.get_column_major(0), Some(&1));
    assert_eq!(array.get_column_major(1), Some(&4));
    assert_eq!(array.get_column_major(2), Some(&2));
    assert_eq!(array.get_column_major(3), Some(&5));
    assert_eq!(array.get_column_major(4), Some(&3));
    assert_eq!(array.get_column_major(5), Some(&6));
    assert_eq!(array.get_column_major(6), None);
    Ok(())
}

#[test]
fn test_get_mut() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows)?;
    let (set_row, set_column) = (0, 2);
    let element = 53;
    let element_ref_option = array.get_mut(set_row, set_column);
    assert!(element_ref_option.is_some());
    let element_ref = element_ref_option.unwrap();
    assert_eq!(element_ref, &rows[set_row][set_column]);
    *element_ref = element;
    assert_eq!(element_ref, &element);
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            let actual = array.get(row, column);
            if (row, column) == (set_row, set_column) {
                assert_eq!(actual, Some(&element));
            } else {
                assert_eq!(actual, Some(&rows[row][column]));
            }
        }
    }
    Ok(())
}

#[test]
fn test_get_mut_row_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows)?;
    assert_eq!(array.get_mut_row_major(0), Some(&mut 1));
    assert_eq!(array.get_mut_row_major(1), Some(&mut 2));
    assert_eq!(array.get_mut_row_major(2), Some(&mut 3));
    assert_eq!(array.get_mut_row_major(3), Some(&mut 4));
    assert_eq!(array.get_mut_row_major(4), Some(&mut 5));
    assert_eq!(array.get_mut_row_major(5), Some(&mut 6));
    assert_eq!(array.get_mut_row_major(6), None);
    Ok(())
}

#[test]
fn test_get_mut_column_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows)?;
    assert_eq!(array.get_mut_column_major(0), Some(&mut 1));
    assert_eq!(array.get_mut_column_major(1), Some(&mut 4));
    assert_eq!(array.get_mut_column_major(2), Some(&mut 2));
    assert_eq!(array.get_mut_column_major(3), Some(&mut 5));
    assert_eq!(array.get_mut_column_major(4), Some(&mut 3));
    assert_eq!(array.get_mut_column_major(5), Some(&mut 6));
    assert_eq!(array.get_mut_column_major(6), None);
    Ok(())
}

#[test]
fn test_set() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows)?;
    let (set_row, set_column) = (1, 0);
    let element = 42;
    array.set(set_row, set_column, element).unwrap();
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            let actual = array.get(row, column);
            if (row, column) == (set_row, set_column) {
                assert_eq!(actual, Some(&element));
            } else {
                assert_eq!(actual, Some(&rows[row][column]));
            }
        }
    }
    Ok(())
}

#[test]
fn test_set_row_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows)?;
    let set_index = 4;
    let set_row = 1;
    let set_column = 1;
    let element = 42;
    array.set_row_major(set_index, element).unwrap();
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            let actual = array.get(row, column);
            if (row, column) == (set_row, set_column) {
                assert_eq!(actual, Some(&element));
            } else {
                assert_eq!(actual, Some(&rows[row][column]));
            }
        }
    }
    Ok(())
}

#[test]
fn test_set_column_major() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows)?;
    let set_index = 4;
    let set_row = 0;
    let set_column = 2;
    let element = 42;
    array.set_column_major(set_index, element).unwrap();
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            let actual = array.get(row, column);
            if (row, column) == (set_row, set_column) {
                assert_eq!(actual, Some(&element));
            } else {
                assert_eq!(actual, Some(&rows[row][column]));
            }
        }
    }
    Ok(())
}

#[test]
fn test_elements_row_major_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let row_major = vec![1, 2, 3, 4, 5, 6];
    let array = Array2D::from_rows(&rows)?;
    let row_len = rows[0].len();
    for (index, element) in array.elements_row_major_iter().enumerate() {
        let row_index = index / row_len;
        let column_index = index % row_len;
        // Do it both ways to make sure we're doing this right
        assert_eq!(element, &rows[row_index][column_index]);
        assert_eq!(element, &row_major[index]);
    }
    Ok(())
}

#[test]
fn test_elements_column_major_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let column_major = vec![1, 4, 2, 5, 3, 6];
    let array = Array2D::from_rows(&rows)?;
    let column_len = rows.len();
    for (index, element) in array.elements_column_major_iter().enumerate() {
        let column_index = index / column_len;
        let row_index = index % column_len;
        // Do it both ways to make sure we're doing this right
        assert_eq!(element, &rows[row_index][column_index]);
        assert_eq!(element, &column_major[index]);
    }
    Ok(())
}

#[test]
fn test_row_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    let first_row_iter = array.row_iter(0)?;
    for (index, element) in first_row_iter.enumerate() {
        assert_eq!(element, &rows[0][index]);
    }
    let second_row_iter = array.row_iter(1)?;
    for (index, element) in second_row_iter.enumerate() {
        assert_eq!(element, &rows[1][index]);
    }
    Ok(())
}

#[test]
fn test_column_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    let first_column_iter = array.column_iter(0)?;
    for (index, element) in first_column_iter.enumerate() {
        assert_eq!(element, &rows[index][0]);
    }
    let second_column_iter = array.column_iter(1)?;
    for (index, element) in second_column_iter.enumerate() {
        assert_eq!(element, &rows[index][1]);
    }
    Ok(())
}

#[test]
fn test_rows_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    for (row_index, row_iter) in array.rows_iter().enumerate() {
        for (column_index, element) in row_iter.enumerate() {
            assert_eq!(element, &rows[row_index][column_index]);
        }
    }
    Ok(())
}

#[test]
fn test_columns_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    for (column_index, column_iter) in array.columns_iter().enumerate() {
        for (row_index, element) in column_iter.enumerate() {
            assert_eq!(element, &rows[row_index][column_index]);
        }
    }
    Ok(())
}

#[test]
fn test_op_index() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            assert_eq!(array[(row, column)], rows[row][column]);
        }
    }
    Ok(())
}

#[test]
fn test_op_index_mut() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows)?;
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            array[(row, column)] += 1;
            assert_eq!(array[(row, column)], rows[row][column] + 1);
        }
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
// Error Handling //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_from_rows_not_all_same_size() {
    let rows = vec![vec![1, 2, 3], vec![4, 5]];
    let result = Array2D::from_rows(&rows);
    assert_eq!(result, Err(Error::DimensionMismatch));
}

#[test]
fn test_from_columns_not_all_same_size() {
    let columns = vec![vec![1, 4], vec![2, 3], vec![4]];
    let result = Array2D::from_columns(&columns);
    assert_eq!(result, Err(Error::DimensionMismatch));
}

#[test]
fn test_from_row_major_dimensions_do_not_match_size() {
    let row_major = vec![1, 2, 3, 4, 5, 6, 7];
    let num_rows = 2;
    let num_columns = 3;
    let result = Array2D::from_row_major(&row_major, num_rows, num_columns);
    assert_eq!(result, Err(Error::DimensionMismatch));
}

#[test]
fn test_from_column_major_dimensions_do_not_match_size() {
    let column_major = vec![1, 4, 2, 5, 3];
    let num_rows = 2;
    let num_columns = 3;
    let result = Array2D::from_column_major(&column_major, num_rows, num_columns);
    assert_eq!(result, Err(Error::DimensionMismatch));
}

#[test]
fn test_from_iter_row_major_not_enough() {
    let iter = 1..5;
    let num_rows = 2;
    let num_columns = 3;
    let result = Array2D::from_iter_row_major(iter, num_rows, num_columns);
    assert_eq!(result, Err(Error::NotEnoughElements));
}

#[test]
fn test_from_iter_column_major_not_enough() {
    let iter = 1..5;
    let num_rows = 2;
    let num_columns = 3;
    let result = Array2D::from_iter_column_major(iter, num_rows, num_columns);
    assert_eq!(result, Err(Error::NotEnoughElements));
}

#[test]
fn test_row_iter_out_of_bounds() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let array = Array2D::filled_with(element, num_rows, num_columns);
    let result = array.row_iter(num_rows);
    assert!(result.is_err());
}

#[test]
fn test_column_iter_out_of_bounds() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let array = Array2D::filled_with(element, num_rows, num_columns);
    let result = array.column_iter(num_columns);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_row() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let array = Array2D::filled_with(element, num_rows, num_columns);
    let _ = array[(num_rows, 0)];
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_column() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let array = Array2D::filled_with(element, num_rows, num_columns);
    let _ = array[(0, num_columns)];
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_row_and_column() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let array = Array2D::filled_with(element, num_rows, num_columns);
    let _ = array[(num_rows, num_columns)];
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds_row() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let mut array = Array2D::filled_with(element, num_rows, num_columns);
    array[(num_rows, 0)] += 1;
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds_column() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let mut array = Array2D::filled_with(element, num_rows, num_columns);
    array[(0, num_columns)] += 1;
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds_row_and_column() {
    let element = 42;
    let num_rows = 2;
    let num_columns = 3;
    let mut array = Array2D::filled_with(element, num_rows, num_columns);
    array[(num_rows, num_columns)] += 1;
}

////////////////////////////////////////////////////////////////////////////////
// Empty Arrays ////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_empty_array_from_rows() -> Result<(), Error> {
    let rows: Vec<Vec<i32>> = vec![];
    let array = Array2D::from_rows(&rows)?;
    assert_eq!(array.num_rows(), 0);
    assert_eq!(array.num_columns(), 0);
    assert_eq!(array.row_len(), 0);
    assert_eq!(array.column_len(), 0);
    Ok(())
}

#[test]
fn test_empty_array_from_row_major() -> Result<(), Error> {
    let row_major: Vec<i32> = vec![];
    let array = Array2D::from_row_major(&row_major, 0, 0)?;
    assert_eq!(array.num_rows(), 0);
    assert_eq!(array.num_columns(), 0);
    assert_eq!(array.row_len(), 0);
    assert_eq!(array.column_len(), 0);
    Ok(())
}

#[test]
fn test_empty_array_from_rows_many_empty_rows() -> Result<(), Error> {
    let rows: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
    let array = Array2D::from_rows(&rows)?;
    assert_eq!(array.num_rows(), 3);
    assert_eq!(array.num_columns(), 0);
    assert_eq!(array.row_len(), 0);
    assert_eq!(array.column_len(), 3);
    Ok(())
}

#[test]
fn test_empty_array_from_row_major_non_zero_columns() -> Result<(), Error> {
    let row_major: Vec<i32> = vec![];
    let array = Array2D::from_row_major(&row_major, 0, 4)?;
    assert_eq!(array.num_rows(), 0);
    assert_eq!(array.num_columns(), 4);
    assert_eq!(array.row_len(), 4);
    assert_eq!(array.column_len(), 0);
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
// Double-Ended Iterators //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_double_ended_iterator_elements_row_major_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    let reversed_columns = array
        .elements_row_major_iter()
        .cloned()
        .rev()
        .collect::<Vec<_>>();
    assert_eq!(reversed_columns, vec![6, 5, 4, 3, 2, 1]);
    Ok(())
}

#[test]
fn test_double_ended_iterator_elements_column_major_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    let reversed_columns = array
        .elements_column_major_iter()
        .cloned()
        .rev()
        .collect::<Vec<_>>();
    assert_eq!(reversed_columns, vec![6, 3, 5, 2, 4, 1]);
    Ok(())
}

#[test]
fn test_double_ended_iterator_row_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    let reversed_columns = array.row_iter(0)?.cloned().rev().collect::<Vec<_>>();
    assert_eq!(reversed_columns, vec![3, 2, 1]);
    Ok(())
}

#[test]
fn test_double_ended_iterator_column_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    let reversed_columns = array.column_iter(1)?.cloned().rev().collect::<Vec<_>>();
    assert_eq!(reversed_columns, vec![5, 2]);
    Ok(())
}

#[test]
fn test_double_ended_iterator_rows_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    let reversed_rows = array
        .rows_iter()
        .rev()
        .map(|row| row.cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert_eq!(reversed_rows, vec![vec![4, 5, 6], vec![1, 2, 3]]);
    Ok(())
}

#[test]
fn test_double_ended_iterator_columns_iter() -> Result<(), Error> {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows)?;
    let reversed_columns = array
        .columns_iter()
        .rev()
        .map(|row| row.cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert_eq!(reversed_columns, vec![vec![3, 6], vec![2, 5], vec![1, 4]]);
    Ok(())
}
