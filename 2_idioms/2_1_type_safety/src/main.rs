mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct New;
    #[derive(Clone, Debug, PartialEq)]
    pub struct Unmoderated;
    #[derive(Clone, Debug, PartialEq)]
    pub struct Published;
    #[derive(Clone, Debug, PartialEq)]
    pub struct Deleted;
    pub trait PostState {}
    impl PostState for New {}
    impl PostState for Unmoderated {}
    impl PostState for Published {}
    impl PostState for Deleted {}
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);
}
mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
}

#[derive(Clone)]
struct Post<S: post::PostState> {
    state: S,
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}

impl Post<post::New> {
    pub fn publish(self) -> Post<post::Unmoderated> {
        Post {
            state: post::Unmoderated,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

impl Post<post::Unmoderated> {
    pub fn allow(self) -> Post<post::Published> {
        Post {
            state: post::Published,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }

    pub fn deny(self) -> Post<post::Deleted> {
        Post {
            state: post::Deleted,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}
impl Post<post::Published> {
    pub fn delete(self) -> Post<post::Deleted> {
        Post {
            state: post::Deleted,
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

fn main() {
    let new_post1 = Post {
        state: post::New,
        id: post::Id(1),
        user_id: user::Id(1),
        title: post::Title("aa".to_owned()),
        body: post::Body("body".to_owned()),
    };
    let new_post2 = new_post1.clone();

    let unm_post = new_post1.publish();
    let denied = unm_post.deny();
    let unm_post2 = new_post2.publish();
    let post = unm_post2.allow();
    let deleted = post.delete();
}
