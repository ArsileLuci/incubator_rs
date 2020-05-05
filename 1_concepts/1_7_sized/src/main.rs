use core::hash::Hash;
use core::marker::PhantomData;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug, Clone)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

struct UserRepositoryS<T, K>
where
    T: Storage<K, User>,
{
    lock: Arc<Mutex<T>>,
    ph_key: PhantomData<K>,
}

impl<T: Storage<K, User>, K> UserRepositoryS<T, K> {
    pub fn new(user_store: T) -> Self {
        UserRepositoryS {
            lock: Arc::new(Mutex::new(user_store)),
            ph_key: PhantomData,
        }
    }

    pub fn get(&self, pk: &K) -> Option<User> {
        match self.lock.lock().unwrap().get(&pk) {
            None => None,
            Some(u) => Some(u.clone()),
        }
    }

    pub fn add(&self, key: K, val: User) {
        self.lock.lock().unwrap().set(key, val)
    }

    pub fn remove(&self, key: &K) -> Option<User> {
        self.lock.lock().unwrap().remove(key)
    }

    pub fn update(&self, key: K, val: User) {
        self.lock.lock().unwrap().remove(&key);
        self.lock.lock().unwrap().set(key, val)
    }
}

impl<TStorage> UserRepository for UserRepositoryS<TStorage, u64>
where
    TStorage: Storage<u64, User>,
{
    fn get(&self, pk: &u64) -> Option<User> {
        self.get(pk)
    }
    fn add(&self, key: u64, val: User) {
        &self.add(key, val);
    }
    fn remove(&self, key: &u64) -> Option<User> {
        self.remove(key)
    }
    fn update(&self, key: u64, val: User) {
        &self.update(key, val);
    }
}

impl<K: Hash + Eq, V> Storage<K, V> for HashMap<K, V> {
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

trait Command {}
trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

trait UserRepository {
    fn get(&self, pk: &u64) -> Option<User>;

    fn add(&self, key: u64, val: User);

    fn remove(&self, key: &u64) -> Option<User>;

    fn update(&self, key: u64, val: User);
}

#[derive(Debug)]
struct CreateUser {}
impl Command for CreateUser {}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), &'static str>;

    fn handle_command(&self, cmd: &CreateUser, ctx: &Self::Context) -> Self::Result {
        match ctx.get(&self.id) {
            Some(_) => {
                return Err("User with this Id already exists");
            }
            None => {
                ctx.add(self.id, self.clone());
                return Ok(());
            }
        }
    }
}
fn main() {
    let storage = HashMap::<u64, User>::new();
    let repo_s = UserRepositoryS::new(storage);
    repo_s.add(
        1,
        User {
            id: 1,
            email: Cow::Borrowed("email1@example.com"),
            activated: true,
        },
    );
    repo_s.add(
        2,
        User {
            id: 2,
            email: Cow::Borrowed("email2@example.com"),
            activated: false,
        },
    );
    println!("{:?}", repo_s.get(&2));
    let user = repo_s.get(&2);
    println!("{:?}", user);
    repo_s.update(
        2,
        User {
            id: 2,
            email: Cow::Borrowed("email2@example.com"),
            activated: true,
        },
    );
    let user1 = repo_s.remove(&1);
    println!("{:?}", user1);
    let cmd = CreateUser {};
    let result_create1 = user1.unwrap().handle_command(&cmd, &repo_s);
    println!("{:?}", result_create1);
    let bx: Box<dyn UserRepository> = Box::new(repo_s);
    let result_create2 = user.unwrap().handle_command(&cmd, &*bx);
    println!("{:?}", result_create2);
}
