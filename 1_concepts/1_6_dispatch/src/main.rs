use core::marker::PhantomData;
use core::hash::Hash;
use std::{collections::HashMap, borrow::Cow};

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

//Dynamic
struct UserRepositoryD {
    user_store: Box<dyn Storage<u64, User>>,
}


impl UserRepositoryD {
    pub fn new(user_store: Box<dyn Storage<u64, User>>) -> Self {
        UserRepositoryD {user_store : user_store}
    }

    pub fn get(&mut self, pk: &u64) -> Option<&User> {
        self.user_store.get(pk)
    }

    pub fn add(&mut self, key: u64, val: User) {
        self.user_store.set(key, val)
    } 

    pub fn remove(&mut self, key: &u64) -> Option<User> {
        self.user_store.remove(key)
    }

    pub fn update(&mut self, key: u64, val: User) {
        self.user_store.remove(&key);
        self.user_store.set(key, val)
    }
}

//Static
struct UserRepositoryS<T, K>
    where T : Storage<K, User> {
    user_store: T,
    ph_key : PhantomData<K>
}

impl<T : Storage<K, User>, K> UserRepositoryS<T, K> {
    pub fn new(user_store : T) -> Self {
        UserRepositoryS { user_store: user_store, ph_key: PhantomData}
    }

    pub fn get(&mut self, pk: &K) -> Option<&User> {
        self.user_store.get(pk)
    }

    pub fn add(&mut self, key: K, val: User) {
        self.user_store.set(key, val)
    } 

    pub fn remove(&mut self, key: &K) -> Option<User> {
        self.user_store.remove(key)
    }

    pub fn update(&mut self, key: K, val: User) {
        self.user_store.remove(&key);
        self.user_store.set(key, val)
    }
}

impl<K: Hash + Eq, V> Storage<K, V> for HashMap<K,V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.get(&key)
    }
    fn set(&mut self, key: K, val: V) {
        self.insert(key, val);
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}

fn main() {
    
    //Static Dispatch
    {
        let storage = HashMap::<u64, User>::new();
        let mut repo_s = UserRepositoryS::new(storage);
        repo_s.add(1, User{ id:1, email: Cow::Borrowed("email1@example.com"), activated: true});
        repo_s.add(2, User{ id:2, email: Cow::Borrowed("email2@example.com"), activated: false});
        println!("{:?}", repo_s.get(&2));
        let user = repo_s.get(&2);
        println!("{:?}", user);
        repo_s.update(2, User{ id:2, email: Cow::Borrowed("email2@example.com"), activated: true});
        let user1 = repo_s.remove(&1);
        println!("{:?}", user1);
        //
    }

    //Dynamic Dispatch
    {
        let storage2 = HashMap::<u64, User>::new();
        let mut repo_d =  UserRepositoryD::new(Box::new(storage2));
        repo_d.add(1, User{ id:1, email: Cow::Borrowed("email1@example.com"), activated: true});
        repo_d.add(2, User{ id:2, email: Cow::Borrowed("email2@example.com"), activated: false});
        println!("{:?}", repo_d.get(&2));
        let user = repo_d.get(&2);
        println!("{:?}", user);
        repo_d.update(2, User{ id:2, email: Cow::Borrowed("email2@example.com"), activated: true});
        let user1 = repo_d.remove(&1);
        println!("{:?}", user1);
    }
}
