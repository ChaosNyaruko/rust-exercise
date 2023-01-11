fn main() {
    println!("This is a playground!");
    let x = 2;
    let r1 = &x;
    let r2 = &r1;
    let r3 = &&x;
    println!("{:}, {:p}, {:p}, {:p}",x, r1, r2, r3); 
}
