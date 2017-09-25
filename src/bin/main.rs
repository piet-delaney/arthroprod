extern crate arthroprod;
use arthroprod::ops;
use arthroprod::types::Alpha;
// use arthroprod::utils;


fn main() {
    // Make some Alphas
    let p = Alpha::new("p");
    let ab = Alpha::new("0");
    let ex = Alpha::new("-10");
    let ty = Alpha::new("031");
    let bz = Alpha::new("-12");
    let q = Alpha::new("-0123");

    // Find some products
    let ans = ops::find_prod(&bz, &p);
    println!("{} ^ {} = {}", bz, p, ans);
    let ans = ops::find_prod(&ab, &q);
    println!("{} ^ {} = {}", ab, q, ans);
    let ans = ops::find_prod(&ty, &ty);
    println!("{} ^ {} = {}", ty, ty, ans);
    let ans = ops::find_prod(&q, &ex);
    println!("{} ^ {} = {}", q, ex, ans);
}