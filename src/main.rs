fn main() {
    use cpproio::{FullInput, InputSource, input};
    use std::io::Cursor;

    let cursor = Cursor::new(b"3 2 1 2 3 4 5 6 2 2");
    let mut source = FullInput::new(cursor);

    input! {
        source,
        n: usize,
        m: usize,
        mut arr: Vec<i32>[n; m],
        pair: (u8, u8),
    }
    arr[0][0] = 0;

    assert_eq!(n, 3);
    assert_eq!(m, 2);
    assert_eq!(arr, vec![vec![0, 2], vec![3, 4], vec![5, 6]]);
    assert_eq!(pair, (2, 2));
}
