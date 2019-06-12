use array2d::Array2D;

fn format_board(board: &Array2D<String>) -> String {
    board
        .rows_iter()
        .map(|row_iter| row_iter.cloned().collect::<Vec<_>>().join("|"))
        .collect::<Vec<_>>()
        .join("\n-----\n")
}

fn main() {
    let mut board = Array2D::filled_with(" ".to_string(), 3, 3);
    println!("{}\n", format_board(&board));
    board[(0, 2)] = "X".to_string();
    println!("{}\n", format_board(&board));
}
