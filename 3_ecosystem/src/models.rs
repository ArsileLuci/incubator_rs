use juniper::FieldResult;
use juniper::RootNode;

use serde::{Deserialize, Serialize};

use juniper::{GraphQLInputObject, GraphQLObject};

/// Method to check is user authorized
/// Important! Runs database query on each call
fn is_authorized(ctx: &GraphQLContext) -> bool {
    use crate::diesel::{ExpressionMethods, QueryDsl};
    use crate::schema::users::dsl::*;

    //if authtoken wasn't set returns false
    if ctx.auth_data.is_none() {
        return false;
    }

    let ad: &AuthData = ctx.auth_data.as_ref().unwrap();

    match users
        .filter(id.eq(ad.id))
        .first::<UserEntity>(&ctx.connection)
    {
        Ok(u) => return u.password == ad.pass_hash,
        Err(_) => return false,
    }
}
pub struct QueryRoot;

#[juniper::object(Context = GraphQLContext,)]
impl QueryRoot {
    /// get current user information such as id, name and friend list
    /// If user isn't authorized or authorized with bad credentials method
    /// will return error, with message `Unauthorized`
    pub fn getUser(context: &GraphQLContext) -> FieldResult<User> {
        use crate::diesel::{ExpressionMethods, JoinOnDsl, QueryDsl};
        use crate::schema::friends::dsl::*;
        use crate::schema::users::dsl::*;

        //Checks user authorization status and returns error if user is not authorized
        if !is_authorized(context) {
            return Err(juniper::FieldError::new(
                "Unauthorized",
                juniper::Value::null(),
            ));
        }

        //Query user from database
        let user = users
            .filter(id.eq(context.auth_data.as_ref().unwrap().id))
            .first::<UserEntity>(&context.connection)
            .unwrap();

        //Query user's friends from database by joining friend links and users on friend_id == id
        let _friends = friends
            .filter(user_id.eq(user.id))
            .inner_join(users.on(id.eq(friend_id)))
            .load::<(FriendLink, UserEntity)>(&context.connection)
            .map(|x| {
                x.into_iter()
                    .map(|item| Friend {
                        id: item.1.id,
                        name: item.1.name,
                    })
                    .collect::<Vec<_>>()
            })
            .ok()
            .or(Some(Vec::new()))
            .unwrap();

        Ok(User {
            name: user.name,
            id: user.id,
            friends: _friends,
        })
    }
}
pub struct MutationRoot;

pub struct GraphQLContext {
    ///Database connection
    pub connection: crate::Connection,
    ///Authentification data from cookies, required to method which require authorized user
    pub auth_data: Option<AuthData>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthData {
    pub id: i32,
    pub pass_hash: String,
}

impl juniper::Context for GraphQLContext {}

#[juniper::object(Context = GraphQLContext,)]
impl MutationRoot {
    pub fn createUser(new_user: UserAuthData, context: &GraphQLContext) -> FieldResult<User> {
        use crate::schema::users::dsl::*;

        let config = argon2::Config::default();
        let u = NewUserEntity {
            name: new_user.name,
            password: argon2::hash_encoded(new_user.password.as_bytes(), b"step3salt", &config)
                .unwrap(),
        };
        let user = diesel::insert_into(users)
            .values(&u)
            .get_result::<UserEntity>(&context.connection);

        match user {
            Ok(_u) => Ok(User {
                name: _u.name,
                id: _u.id,
                friends: Vec::new(),
            }),
            Err(err) => Err(juniper::FieldError::new(
                "User with this username already exists",
                juniper::Value::null(),
            )),
        }
    }

    /// function to remove user from friends list of current user
    /// authorized with authorize method. If user is not authorized
    /// or uses wrong credentials it will return Error with message
    /// `Unauthorized`. If user with `friend_info`'s name  doesn't exists
    /// method will return message indicating it. If authorized user
    /// doesn't have user with `friend_info`'s name in friend list
    /// method will return error indicating that user already has
    /// `friend_info` in friend list
    pub fn removeFriend(
        friend_info: FriendNameDto,
        context: &GraphQLContext,
    ) -> FieldResult<OpResult> {
        use crate::diesel::{ExpressionMethods, QueryDsl};

        use crate::schema::friends::dsl::*;
        use crate::schema::users::dsl::*;

        if !is_authorized(context) {
            return Err(juniper::FieldError::new(
                "Unauthorized",
                juniper::Value::null(),
            ));
        }

        let u = users
            .filter(name.eq(friend_info.name))
            .first::<UserEntity>(&context.connection)
            .ok();

        match u {
            None => {
                return Err(juniper::FieldError::new(
                    "this user doesn't exist",
                    juniper::Value::null(),
                ))
            }
            Some(user) => {
                let r = diesel::delete(
                    friends
                        .filter(user_id.eq(context.auth_data.as_ref().unwrap().id))
                        .filter(friend_id.eq(user.id)),
                )
                .execute(&context.connection)
                .ok();

                match r {
                    Some(l) => {
                        if l == 1 {
                            let msg = format!(
                                "user {} was successfully removed from your friends list",
                                user.name
                            );
                            return Ok(OpResult { text: msg });
                        } else {
                            let msg = format!("user {} is not your friend", user.name);
                            return Ok(OpResult { text: msg });
                        }
                    }
                    None => {
                        return Err(juniper::FieldError::new(
                            "error ocured",
                            juniper::Value::null(),
                        ));
                    }
                }
            }
        }
    }

    /// function to add user to friends list of current user
    /// authorized with authorize method. If user is not authorized
    /// or uses wrong credentials it will return Error with message
    /// `Unauthorized`. If user with `new_friend`'s name  doesn't exists
    /// method will return message indicating it. If authorized user
    /// already friend with `new_friend` method will return error
    /// indicating that user already has `new_friend` in friend list
    pub fn addFriend(new_friend: FriendNameDto, context: &GraphQLContext) -> FieldResult<OpResult> {
        use crate::diesel::ExpressionMethods;
        use crate::diesel::QueryDsl;

        use crate::schema::friends::dsl::*;
        use crate::schema::users::dsl::*;

        if !is_authorized(context) {
            return Err(juniper::FieldError::new(
                "Unauthorized",
                juniper::Value::null(),
            ));
        }

        let u = users
            .filter(name.eq(new_friend.name))
            .first::<UserEntity>(&context.connection)
            .ok();

        match u {
            Some(user) => {
                let insert = diesel::insert_into(friends)
                    .values(FriendLink {
                        user_id: context.auth_data.as_ref().unwrap().id,
                        friend_id: user.id,
                    })
                    .execute(&context.connection);
                if insert.is_err() {
                    let err = format!("You already friend with {}", user.name);
                    return Err(juniper::FieldError::new(&err, juniper::Value::null()));
                }
            }
            None => {
                return Err(juniper::FieldError::new(
                    "Not Found user with this name",
                    juniper::Value::null(),
                ));
            }
        }

        Ok(OpResult {
            text: "You have successfuly added new friend".to_owned(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
#[derive(Deserialize, Serialize, GraphQLInputObject)]
//represents user info, that used in registration and authorization, like username and password
pub struct UserAuthData {
    pub name: String,
    //raw password from registration form
    pub password: String,
}

use crate::schema::*;
use diesel::RunQueryDsl;

#[derive(Serialize, Insertable, Queryable)]
#[table_name = "users"]
pub struct UserEntity {
    pub id: i32,
    //Unique!
    pub name: String,
    //HASH argon2 of password
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUserEntity {
    name: String,
    //HASH argon2 of password
    password: String,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
/// User Data Transfer Object,
/// used only in responses
pub struct User {
    pub id: i32,
    pub name: String,
    pub friends: Vec<Friend>,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Queryable)]
/// User Data Transfer Object, but without friends field
/// Used to Send data about users friends as part of `User` entity
/// Also can be used to query data from `users` table
pub struct Friend {
    id: i32,
    name: String,
}

#[derive(Serialize, GraphQLInputObject)]
/// Data Transfer Object which represents User's name
/// used to query data from `users` table
struct FriendNameDto {
    //Username to search
    name: String,
}

#[derive(Insertable, Queryable)]
#[table_name = "friends"]
///One directional user - friend link
struct FriendLink {
    user_id: i32,
    friend_id: i32,
}
#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct OpResult {
    ///Operation result message
    pub text: String,
}
