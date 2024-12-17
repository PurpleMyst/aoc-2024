fn main() {
    let mut a = 184811792;

    loop {
        let n = a & 0b111;
        let p = (a >> (n ^ 7)) & 7;
        let m = n ^ p;
        a >>= 3;                 // shift a right by 3 bits

        print!("{} ", m);
        if a != 0 {
            continue;
        }

        break;
    }
}
