use jirachi::collision_resistant::Jirachi;

#[test]
fn wish_key() {
    println!("Wishing for a keys...");
    let mut jirachi = Jirachi::new().unwrap();
    println!("wish: {}", jirachi.wish().unwrap());
}


