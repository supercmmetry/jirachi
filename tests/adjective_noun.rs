use jirachi::adjective_noun::Jirachi;
use jirachi::Wishable;

#[test]
fn wish_key() {
    println!("Wishing for a key...");
    let mut jirachi = Jirachi::new().unwrap();
    println!("wish: {}", jirachi.wish().unwrap());
}