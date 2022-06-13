
pub fn matrix_indexed<T, const W: usize, const H: usize>(
    trans: fn(usize, usize) -> T,
) -> [[T; H]; W] {
    let mut uninit = [[(0, 0); H]; W];
    for x in 0..W {
        for y in 0..H {
            uninit[x][y] = (x, y);
        }
    }

    uninit.map(|row| row.map(|(x, y)| trans(x, y)))
}

pub fn matrix<T, const W: usize, const H: usize>(trans: fn() -> T) -> [[T; H]; W] {
    let uninit = [[(); H]; W];
    uninit.map(|row| row.map(|_| trans()))
}
