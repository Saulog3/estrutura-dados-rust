fn main() {
    let mut v = vec![14, 33, 27, 35, 10];
    let n = v.len();

    for i in 0..n{
        println!("{:?}", v);
        for j in 0..n - 1 - i {
            if v[j] < v[j + 1] {
                let temp = v[j];
                v[j] = v[j + 1];
                v[j + 1] = temp;
            }
        }
    }
    println!("{:?}", v);
}