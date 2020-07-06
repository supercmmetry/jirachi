use jirachi::collision_resistant::Jirachi;
use jirachi::Wishable;

#[test]
fn wish_key() {
    println!("Wishing for a collision-resistant key...");
    let mut jirachi = Jirachi::new().unwrap();
    println!("wish: {}", jirachi.wish().unwrap());
}


