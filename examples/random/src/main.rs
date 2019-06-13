use array2d::Array2D;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let board = Array2D::filled_by_row_major(|| rng.gen_range(0, 10), 3, 2);
    println!("{:?}", board);

    let mut counter = 1;
    let f = || {
        let tmp = counter;
        counter += 1;
        tmp
    };
    let board2 = Array2D::filled_by_column_major(f, 2, 3);
    println!("{:?}", board2);
}
