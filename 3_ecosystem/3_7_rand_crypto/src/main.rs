#[macro_use]
extern crate lazy_static;
extern crate argon2;
use rand_core::RngCore;
use sha3::{Digest, Sha3_512};
use std::io::Read;
use std::iter::FromIterator;

fn generate_password(len: usize, symbols: &[char]) -> String {
    (0..len)
        .map(|_| symbols[rand::random::<usize>() % symbols.len()])
        .collect()
}

fn select_rand_val<T>(items: &[T]) -> &T {
    &items[rand::random::<usize>() % items.len()]
}

fn new_acces_token() -> String {
    let mut r_gen = rand::rngs::OsRng;
    lazy_static! {
        static ref ALPHABET: Vec<char> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                .chars()
                .collect();
    }

    (0..64)
        .map(|_| ALPHABET[(r_gen.next_u64() % 62) as usize])
        .collect()
}

fn get_file_hash<T: AsRef<std::path::Path>>(path: T) -> String {
    let mut HASHER: Sha3_512 = Sha3_512::new();
    let mut file = std::fs::File::open(path).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf);
    HASHER.input(buf);
    let res = HASHER.result().into_iter().collect::<Vec<_>>();

    String::from_utf8(res).unwrap()
}

fn get_password_hash(password: &String) -> String {
    use argon2::{self, Config};

    let password = password.as_bytes();
    let salt = b"MySuperSecretSalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    hash
}

fn main() {
    let alphabet: Vec<_> = "qwertyuiop[]asdfghjklzxcvbnm,.1234567890-=!@#$%^&*()_"
        .chars()
        .collect();
    let pass = generate_password(28, &alphabet[..]);
    println!("{}", pass);
    let hash = get_password_hash(&pass);
    println!("{}", hash);
}
