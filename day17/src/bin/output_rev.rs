fn main() {
    let mut a = 0;

    let expected = [2, 4, 1, 7, 7, 5, 0, 3, 1, 7, 4, 1, 5, 5, 3, 0];

    for &k in expected.iter().rev() {
        // let n = a & 0b111;
        // let p = (a >> (n ^ 7)) & 7;
        // we want n^p^k == 0
        a |= k;
        a = a & !(k << (k ^ 7));
        a <<= 3;
    }

    println!("{a}")
}
