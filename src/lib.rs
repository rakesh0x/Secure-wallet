use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Hash)]
struct Credentials {
    id: u32,
    name: String,
    password: u64,
}

fn secure_store<T: Hash>(username: &T, password: &T) -> u64 {
    let mut s = DefaultHasher::new();
    username.hash(&mut s);
    password.hash(&mut s);
    s.finish()
}

fn main() {
    let user = Credentials {
        id: 1,
        name: "rakesh".to_string(),
        password: 12345,
    };

    let pass = Credentials {
        id: 2,
        name: "secret".to_string(),
        password: 54321,
    };

    let hashed = secure_store(&user, &pass);
    println!("hash = {}", hashed);

    assert!(hashed > 0);
}
