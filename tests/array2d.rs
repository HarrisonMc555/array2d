use array2d::{Array2D};

#[test]
fn test_fill_with() {
    let value = 7;
    let array = Array2D::fill_with(value, 4, 5);
    assert_eq!(array.num_rows(), 4);
    assert_eq!(array.num_columns(), 5);
    assert_eq!(array.num_items(), 20);
    for item in array.iter_items_row_major() {
        assert_eq!(item, &7);
    }
    for item in array.iter_items_column_major() {
        assert_eq!(item, &7);
    }
}

#[test]
fn test_from_rows() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows);
    assert_eq!(array.as_rows(), rows);
}

#[test]
fn test_from_columns() {
    let columns = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    let array = Array2D::from_columns(&columns);
    assert_eq!(array.as_columns(), columns);
}

#[test]
fn test_from_row_major() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let row_major = vec![1, 2, 3, 4, 5, 6];
    let num_rows = 2;
    let num_columns = 3;
    let array = Array2D::from_row_major(&row_major, num_rows, num_columns);
    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, item) in row.iter().enumerate() {
            assert_eq!(array.get(row_index, column_index), Some(item));
        }
    }
}

#[test]
fn test_from_column_major() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let column_major = vec![1, 4, 2, 5, 3, 6];
    let num_rows = 2;
    let num_columns = 3;
    let array = Array2D::from_column_major(&column_major, num_rows, num_columns);
    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, item) in row.iter().enumerate() {
            assert_eq!(array.get(row_index, column_index), Some(item));
        }
    }
}

#[test]
fn test_dimensions() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows);
    assert_eq!(array.num_rows(), 2);
    assert_eq!(array.num_columns(), 3);
    assert_eq!(array.row_len(), 3);
    assert_eq!(array.column_len(), 2);
}

#[test]
fn test_get() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows);
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            assert_eq!(array.get(row, column), Some(&rows[row][column]));
        }
    }
}

#[test]
fn test_get_mut() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows);
    let (set_row, set_column) = (0, 2);
    let value = 53;
    let item_ref_option = array.get_mut(set_row, set_column);
    assert!(item_ref_option.is_some());
    let item_ref = item_ref_option.unwrap();
    assert_eq!(item_ref, &rows[set_row][set_column]);
    *item_ref = value;
    assert_eq!(item_ref, &value);
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            let actual = array.get(row, column);
            if (row, column) == (set_row, set_column) {
                assert_eq!(actual, Some(&value));
            } else {
                assert_eq!(actual, Some(&rows[row][column]));
            }
        }
    }
}

#[test]
fn test_set() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows);
    let (set_row, set_column) = (1, 0);
    let value = 42;
    array.set(set_row, set_column, value).unwrap();
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            let actual = array.get(row, column);
            if (row, column) == (set_row, set_column) {
                assert_eq!(actual, Some(&value));
            } else {
                assert_eq!(actual, Some(&rows[row][column]));
            }
        }
    }
}

// #[test]
// fn test_get_rows() {
//     let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
//     let array = Array2D::from_rows(&rows);
//     for row in 0..rows.len() {
//         assert_eq!(array.get_row(row), rows[row].as_slice());
//     }
// }

// #[test]
// fn test_get_rows_mut() {
//     let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
//     let mut array = Array2D::from_rows(&rows);
//     let row_index = 1;
//     let row_values = vec![7, 8, 9];
//     let row_ref = array.get_row_mut(row_index);
//     assert_eq!(row_ref, rows[row_index].as_slice());
//     for (index, item) in row_ref.iter_mut().enumerate() {
//         *item = row_values[index];
//     }
//     for row in 0..rows.len() {
//         let actual = array.get_row(row);
//         if row == row_index {
//             assert_eq!(actual, row_values.as_slice());
//         } else {
//             assert_eq!(actual, rows[row].as_slice());
//         }
//     }
// }

// #[test]
// fn test_set_rows() {
//     let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
//     let mut array = Array2D::from_rows(&rows);
//     let row_index = 1;
//     let row_values = vec![7, 8, 9];
//     array.set_row(1, &row_values);
//     for row in 0..rows.len() {
//         let actual = array.get_row(row);
//         if row == row_index {
//             assert_eq!(actual, row_values.as_slice());
//         } else {
//             assert_eq!(actual, rows[row].as_slice());
//         }
//     }
// }

// #[test]
// fn test_iter_rows() {
//     let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
//     let array = Array2D::from_rows(&rows);
//     for (index, row) in array.iter_rows().enumerate() {
//         assert_eq!(row, rows[index].as_slice());
//     }
// }

#[test]
fn test_iter_items_row_major() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let row_major = vec![1, 2, 3, 4, 5, 6];
    let array = Array2D::from_rows(&rows);
    let row_len = rows[0].len();
    for (index, item) in array.iter_items_row_major().enumerate() {
        let row_index = index / row_len;
        let column_index = index % row_len;
        // Do it both ways to make sure we're doing this right
        assert_eq!(item, &rows[row_index][column_index]);
        assert_eq!(item, &row_major[index]);
    }
}

#[test]
fn test_iter_items_column_major() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let column_major = vec![1, 4, 2, 5, 3, 6];
    let array = Array2D::from_rows(&rows);
    let column_len = rows.len();
    for (index, item) in array.iter_items_column_major().enumerate() {
        let column_index = index / column_len;
        let row_index = index % column_len;
        // Do it both ways to make sure we're doing this right
        assert_eq!(item, &rows[row_index][column_index]);
        assert_eq!(item, &column_major[index]);
    }
}

#[test]
fn test_iter_row() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows);
    let first_row_iter_option = array.iter_row(0);
    assert!(first_row_iter_option.is_some());
    let first_row_iter = first_row_iter_option.unwrap();
    for (index, item) in first_row_iter.enumerate() {
        assert_eq!(item, &rows[0][index]);
    }
    let second_row_iter_option = array.iter_row(1);
    assert!(second_row_iter_option.is_some());
    let second_row_iter = second_row_iter_option.unwrap();
    for (index, item) in second_row_iter.enumerate() {
        assert_eq!(item, &rows[1][index]);
    }
}

#[test]
fn test_iter_column() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows);
    let first_column_iter_option = array.iter_column(0);
    assert!(first_column_iter_option.is_some());
    let first_column_iter = first_column_iter_option.unwrap();
    for (index, item) in first_column_iter.enumerate() {
        assert_eq!(item, &rows[index][0]);
    }
    let second_column_iter_option = array.iter_column(1);
    assert!(second_column_iter_option.is_some());
    let second_column_iter = second_column_iter_option.unwrap();
    for (index, item) in second_column_iter.enumerate() {
        assert_eq!(item, &rows[index][1]);
    }
}

#[test]
fn test_iter_rows() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows);
    for (row_index, row_iter) in array.iter_rows().enumerate() {
        for (column_index, item) in row_iter.enumerate() {
            assert_eq!(item, &rows[row_index][column_index]);
        }
    }
}

#[test]
fn test_iter_columns() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows);
    for (column_index, column_iter) in array.iter_columns().enumerate() {
        for (row_index, item) in column_iter.enumerate() {
            assert_eq!(item, &rows[row_index][column_index]);
        }
    }
}

#[test]
fn test_op_index() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let array = Array2D::from_rows(&rows);
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            assert_eq!(array[(row, column)], rows[row][column]);
        }
    }
}

#[test]
fn test_op_index_mut() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows);
    for row in 0..rows.len() {
        for column in 0..rows[0].len() {
            array[(row, column)] += 1;
            assert_eq!(array[(row, column)], rows[row][column] + 1);
        }
    }
}

#[test]
#[should_panic]
fn test_from_rows_not_all_same_size() {
    let rows = vec![vec![1, 2, 3], vec![4, 5]];
    Array2D::from_rows(&rows);
}

#[test]
#[should_panic]
fn test_from_columns_not_all_same_size() {
    let columns = vec![vec![1, 4], vec![2, 3], vec![4]];
    Array2D::from_columns(&columns);
}

#[test]
#[should_panic]
fn test_from_row_major_dimensions_do_not_match_size() {
    let row_major = vec![1, 2, 3, 4, 5, 6, 7];
    let num_rows = 2;
    let num_columns = 3;
    Array2D::from_row_major(&row_major, num_rows, num_columns);
}

#[test]
#[should_panic]
fn test_from_column_major_dimensions_do_not_match_size() {
    let column_major = vec![1, 4, 2, 5, 3];
    let num_rows = 2;
    let num_columns = 3;
    Array2D::from_column_major(&column_major, num_rows, num_columns);
}

#[test]
fn test_empty_array_from_rows() {
    let rows: Vec<Vec<i32>> = vec![];
    let array = Array2D::from_rows(&rows);
    assert_eq!(array.num_rows(), 0);
    assert_eq!(array.num_columns(), 0);
    assert_eq!(array.row_len(), 0);
    assert_eq!(array.column_len(), 0);
}

#[test]
fn test_empty_array_from_row_major() {
    let row_major: Vec<i32> = vec![];
    let array = Array2D::from_row_major(&row_major, 0, 0);
    assert_eq!(array.num_rows(), 0);
    assert_eq!(array.num_columns(), 0);
    assert_eq!(array.row_len(), 0);
    assert_eq!(array.column_len(), 0);
}

#[test]
fn test_empty_array_from_rows_many_empty_rows() {
    let rows: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
    let array = Array2D::from_rows(&rows);
    assert_eq!(array.num_rows(), 3);
    assert_eq!(array.num_columns(), 0);
    assert_eq!(array.row_len(), 0);
    assert_eq!(array.column_len(), 3);
}

#[test]
fn test_empty_array_from_row_major_non_zero_columns() {
    let row_major: Vec<i32> = vec![];
    let array = Array2D::from_row_major(&row_major, 0, 4);
    assert_eq!(array.num_rows(), 0);
    assert_eq!(array.num_columns(), 4);
    assert_eq!(array.row_len(), 4);
    assert_eq!(array.column_len(), 0);
}
