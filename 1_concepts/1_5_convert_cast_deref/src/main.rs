extern crate rand;

use std::convert::{TryFrom};

#[derive(Debug)]
struct EmailString {
    login: String,
    domain: String,
}

impl TryFrom<&str> for EmailString {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split("@");
        let login: String;
        let domain: String;
        match split.next() {
            Some(login_part) => {
                login = String::from(login_part);
            }
            None => {
                return Err("can't find first part of email");
            }
        }
        match split.next() {
            Some(domain_part) => {
                domain = String::from(domain_part);
            }
            None => {
                return Err("can't find domain");
            }
        }
        match split.next() {
            Some(_) => {
                return Err("Not a valid email address");
            }
            None => {
                return Ok(EmailString{login, domain});
            }
        }
    } 
}

impl ToString for EmailString {
    fn to_string(&self) -> String {
        format!("{}@{}", self.login, self.domain)
    }
}

struct Random<'a, T> {
    item1: &'a T,
    item2: &'a T,
    item3: &'a T,
}

impl<'a,T> Random<'a,T> {
    pub fn new(item1: &'a T, item2: &'a T, item3: &'a T) 
    -> Self {
        Random {
            item1,
            item2,
            item3
        }
    }
}

impl<'a, T> std::ops::Deref for Random<'a, T> {
    type Target = T;
    fn deref(&self) 
    -> &Self::Target { 
        let rnd = rand::random::<u8>();
        match rnd%3 {
            0 => { 
                self.item1
            },
            1 => {
                self.item2
            },
            2 => {
                self.item3
            }
            _ => {
                panic!("unreachable");
            }
        }
    }
}


fn main() {
    let email1 : EmailString = EmailString::try_from("myemail@mail.ru").unwrap();
    let email2 : EmailString = EmailString::try_from("myemail@gmail.com").unwrap();
    println!("{:?}", EmailString::try_from("not_email"));
    let email3 : EmailString = EmailString::try_from("myemailwithsubdomen@subdomen.domen.com").unwrap();
    println!("{:?}", EmailString::try_from("to@many@a"));
    let rnd : Random<EmailString> = Random::new(&email1, &email2, &email3);
    
    for i in 1..10 {
        println!("i is {}, Random email is {}", i, (*rnd).to_string());
    } 

}
