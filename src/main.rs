
fn main() {
    let t1 = (1, true, 2.0);
    let t2= (true, 2.0, 1);

    println!("{:?}", t1);

    let i = t1.0;
    println!("{}",i);

    let (x, y, _) = t2;

    let u = ();
}

