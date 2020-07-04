use crate::Jirachi;

#[test]
fn wish_key() {
    println!("Wishing for a key");

    let mut jirachi = Jirachi::new();
    println!("wish: {}", jirachi.wish());
}
