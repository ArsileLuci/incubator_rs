use im::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    pub Id: UserId,
    pub Name: UserName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserName(String);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct UserId(u64);

impl From<u64> for UserId {
    fn from(val: u64) -> Self {
        UserId(val)
    }
}

struct UserRepository {
    _store: HashMap<UserId, User>,
}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {
            _store: HashMap::new(),
        }
    }

    pub fn insert(&mut self, u: User) -> Option<User> {
        self._store.insert(u.Id, u)
    }

    pub fn get<TKey: Into<UserId>>(&self, key: TKey) -> Option<User> {
        let id = key.into();
        self._store.get(&id).map(|u| u.clone())
    }

    pub fn get_users<T: Into<UserId>, TKeys: IntoIterator<Item = T>>(
        &self,
        keys: TKeys,
    ) -> HashSet<Option<User>> {
        keys.into_iter()
            .map(|key| self.get(key).map(|val| val.clone()))
            .collect()
    }

    pub fn search_users(&self, search_string: &str) -> HashMap<UserId, User> {
        self._store
            .values()
            .filter(|u| u.Name.0.contains(search_string))
            .map(|u| (u.Id, u.clone()))
            .collect()
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod spec {
    use super::*;

    fn setup() -> UserRepository {
        let mut rep = UserRepository::new();
        let u1 = User {
            Name: UserName("User1_bbbb_aaa".to_owned()),
            Id: UserId(1),
        };
        rep.insert(u1);
        let u2 = User {
            Name: UserName("User2_aaa_bbb".to_owned()),
            Id: UserId(2),
        };
        rep.insert(u2);
        let u3 = User {
            Name: UserName("User3_bbb".to_owned()),
            Id: UserId(3),
        };
        rep.insert(u3);
        let u4 = User {
            Name: UserName("User4_d_aadaa".to_owned()),
            Id: UserId(4),
        };
        rep.insert(u4);
        let u5 = User {
            Name: UserName("User5_abbaa".to_owned()),
            Id: UserId(5),
        };
        rep.insert(u5);
        let u6 = User {
            Name: UserName("User6_baaab".to_owned()),
            Id: UserId(6),
        };
        rep.insert(u6);
        let u7 = User {
            Name: UserName("User7_aaa".to_owned()),
            Id: UserId(7),
        };
        rep.insert(u7);
        rep
    }

    #[test]
    fn test_get_user_with_correct_id() {
        let r = setup();
        assert_eq!(UserId::from(1), r.get(1).unwrap().Id);
        assert_eq!(UserId::from(2), r.get(2).unwrap().Id);
        assert_eq!(UserId::from(3), r.get(3).unwrap().Id);
        assert_eq!(UserId::from(4), r.get(4).unwrap().Id);
        assert_eq!(UserId::from(5), r.get(5).unwrap().Id);
        assert_eq!(UserId::from(6), r.get(6).unwrap().Id);
        assert_eq!(UserId::from(7), r.get(7).unwrap().Id);
    }

    #[test]
    fn test_no_user_if_id_not_present() {
        let r = setup();
        assert_eq!(None, r.get(8));
        assert_eq!(None, r.get(9));
        assert_eq!(None, r.get(10));
        assert_eq!(None, r.get(11));
        assert_eq!(None, r.get(12));
        assert_eq!(None, r.get(13));
        assert_eq!(None, r.get(14));
    }

    #[test]
    fn test_get_users_by_multiple_ids() {
        let r = setup();
        let users = r.get_users(vec![1, 2, 3, 7]);
        let mut iter = users.into_iter();
        assert!(iter.next().unwrap().is_some());
        assert!(iter.next().unwrap().is_some());
        assert!(iter.next().unwrap().is_some());
        assert!(iter.next().unwrap().is_some());
    }

    #[test]
    fn test_search_users_by_name() {
        let r = setup();
        let users = r.search_users("aaa");
        for user in users {
            assert!(user.1.Name.0.contains("aaa"));
        }
    }
}
