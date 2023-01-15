fn main() {
    println!("This is a playground!");
    let x = 2;
    let r1 = &x;
    let r2 = &r1;
    let r3 = &&x;
    println!("{:}, {:p}, {:p}, {:p}", x, r1, r2, r3);

    let mut v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    for x in v.iter_mut() {
        *x += 1;
    }
    println!("{:?}", v);
}
