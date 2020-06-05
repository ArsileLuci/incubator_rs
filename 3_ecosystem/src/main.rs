#[macro_use] extern crate diesel;
#[macro_use] extern crate juniper;
use actix_web::HttpMessage;
use actix_web::{web, App, HttpServer, Error, Responder, HttpResponse};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use std::sync::Arc;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let schema = std::sync::Arc::new(models::create_schema());

    HttpServer::new(move ||
        App::new()
            .data(pool.clone())
            .data(schema.clone())
            .app_data(web::PayloadConfig::new(32 * 1024 * 1024 * 1024))
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 32 * 1024))
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/authorize").route(web::post().to(authorize)))
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
//separate auth function due safety reasons
async fn authorize(
    auth_data: web::Json<models::UserAuthData>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    let config = argon2::Config::default();
    let connection = pool.get().expect("cannot get database connection");
    let u = users
        .filter(name.eq(&auth_data.name))
        .first::<models::UserEntity>(&connection);

    //generates argon2 hash based on password
    let p_hash = argon2::hash_encoded(auth_data.password.as_bytes(), b"step3salt", &config).unwrap();

    match u {
        Ok(user) => {
            if p_hash == user.password {

                let ad = models::AuthData {
                    id : user.id.clone(),
                    pass_hash : p_hash,
                };
                //if hashes matches returns Ok 200 with cookie authtoken
                HttpResponse::Ok().cookie(
                    {
                        let mut cookie = actix_web::http::Cookie::new("authtoken", base64::encode(serde_json::to_string(&ad).unwrap()));
                        cookie.set_http_only(true);//sets HttpOnly flag for true, to client code interaction as a result token can't be stolen via XSS
                        cookie
                    }
                ).finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
        Err(_) =>
            HttpResponse::BadRequest().finish(),
    }
}


async fn graphql(
    st: web::Data<Arc<models::Schema>>,
    data: web::Json<GraphQLRequest>,
    pool: web::Data<DbPool>,
    request: web::HttpRequest,
) -> Result<HttpResponse, Error> {

    println!("{:?}", data);
    let auth_data = match request.cookies() {
        Ok(cookies) => {
            let auth_cookie = cookies.iter().filter(|x|x.name() == "authtoken").collect::<Vec<_>>();
            if auth_cookie.len() != 1 {
                None
            }
            else {
                base64::decode(auth_cookie[0].value()).ok()
                    .and_then(|x| String::from_utf8(x).ok())
                    .and_then(|x| serde_json::from_str::<models::AuthData>(&x).ok())
            }

        },
        Err(_) => None,
    };
    let user = web::block(move || {

        let db = models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : auth_data,
        };
        let res = data.execute(&st,  &db);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}


async fn graphiql(
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}



#[cfg(test)]
mod tests{
    use actix_web::{App, test, web};
    use diesel::r2d2::ConnectionManager;
    use diesel::pg::PgConnection;
    use juniper::http::GraphQLRequest;
    use serde::{Serialize, Deserialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    struct Response<T> {
        data: Option<T>,
        errors: Option<Vec<Error>>,
    }

    #[derive(Serialize, Deserialize)]
    struct CreateUser {
        #[serde(rename="createUser")]
        create_user: crate::models::User
    }
    #[derive(Serialize, Deserialize)]
    struct AddFriend {
        #[serde(rename="addFriend")]
        add_friend: crate::models::OpResult
    }

    #[derive(Serialize, Deserialize)]
    struct RemoveFriend {
        #[serde(rename="removeFriend")]
        remove_friend: crate::models::OpResult
    }

    #[derive(Serialize, Deserialize)]
    struct GetUser {
        #[serde(rename="getUser")]
        get_user: crate::models::User
    }

    #[derive(Serialize, Deserialize)]
    struct Error {
        message: String,
        locations : Vec<Location>,
        path: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize)]
    struct Location {
        line: u32,
        column: u32,
    }

    fn create_user(name: &str) -> GraphQLRequest {
        GraphQLRequest::<juniper::DefaultScalarValue>::new(
            format!(r#"mutation {{ createUser(newUser: {{name: "{}", password: "123456"}}) {{
                id
                name
                friends {{
                    id
                    name
                }}
              }}}}"#, name),
                None,
                     None)
    }

    fn add_friend(name: &str) -> GraphQLRequest {
        GraphQLRequest::<juniper::DefaultScalarValue>::new(
            format!(r#"mutation {{ addFriend(newFriend: {{name: "{}"}}) {{
                text
              }}}}"#, name),
                None,
                     None)
    }

    fn remove_friend(name: &str) -> GraphQLRequest {
        GraphQLRequest::<juniper::DefaultScalarValue>::new(
            format!(r#"mutation {{ removeFriend(friendInfo: {{name: "{}"}}) {{
                text
              }}}}"#, name),
                None,
                     None)
    }

    fn get_user() -> GraphQLRequest {
        GraphQLRequest::<juniper::DefaultScalarValue>::new(
            r#"query { getUser {
                id
                name
                friends {
                    id
                    name
                }
              }}"#.to_owned(),
                None,
                     None)
    }

    fn init_db_and_schema<'a>() -> (r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>, std::sync::Arc<juniper::RootNode<'a, crate::models::QueryRoot, crate::models::MutationRoot>>) {
        dotenv::dotenv().ok();

        let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");

        let manager = ConnectionManager::<PgConnection>::new(connspec);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let schema = std::sync::Arc::new(crate::models::create_schema());

        (pool, schema)
    }

    #[actix_rt::test]
    async fn test_cant_register_user_with_same_name() {

        let(pool, schema) = init_db_and_schema();

        //Create first user with this name
        let test_object = create_user("UserWithCommonName");

        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : None,
        };
        let r = test_object.execute(&schema, &db);

        //Create Second user with this name to be 100% sure that this name already exists in db
        let test_object2 = create_user("UserWithCommonName");
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : None,
        };

        let response = test_object2.execute(&schema, &db);
        let json = serde_json::to_string(&response).unwrap();
        let user=  serde_json::from_str::<Response<crate::models::User>>(&json);

        assert_eq!("User with this username already exists", &user.unwrap().errors.unwrap()[0].message);
    }

    #[actix_rt::test]
    async fn test_register_new_user() {

        let(pool, schema) = init_db_and_schema();
        let new_name = Uuid::new_v4().to_string();
        let test_object = create_user(&new_name);
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : None,
        };

        let response = test_object.execute(&schema, &db);
        let json = serde_json::to_string(&response).unwrap();
        let user=  serde_json::from_str::<Response<CreateUser>>(&json);

        assert!(user.unwrap().data.is_some());
    }

    #[actix_rt::test]
    async fn test_add_and_delete_friend() {

        let(pool, schema) = init_db_and_schema();

        let unique_name1 = Uuid::new_v4().to_string();
        let unique_name2 = Uuid::new_v4().to_string();

        //Creating first user
        let request1 = create_user(&unique_name1);
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : None,
        };
        let u1 = request1.execute(&schema, &db);
        let json = serde_json::to_string(&u1).unwrap();
        let user=  serde_json::from_str::<Response<CreateUser>>(&json);

        let id = user.unwrap().data.unwrap().create_user.id;

        //Creating second user
        let request2 = create_user(&unique_name2);
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : None,
        };
        request2.execute(&schema, &db);

        let request3 = add_friend(&unique_name2);

        let config = argon2::Config::default();
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : Some(crate::models::AuthData {
                id: id,
                pass_hash: argon2::hash_encoded(b"123456", b"step3salt", &config).unwrap()
            }),
        };
        let response = request3.execute(&schema, &db);

        let json = serde_json::to_string(&response).unwrap();
        let add=  serde_json::from_str::<Response<AddFriend>>(&json);
        assert_eq!(add.unwrap().data.unwrap().add_friend.text, "You have successfuly added new friend".to_owned());

        let request4 = remove_friend(&unique_name2);
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : Some(crate::models::AuthData {
                id: id,
                pass_hash: argon2::hash_encoded(b"123456", b"step3salt", &config).unwrap()
            }),
        };
        let response = request4.execute(&schema, &db);

        let json = serde_json::to_string(&response).unwrap();
        let remove = serde_json::from_str::<Response<RemoveFriend>>(&json);
        let expected = format!("user {} was successfully removed from your friends list", unique_name2);
        assert_eq!(remove.unwrap().data.unwrap().remove_friend.text, expected);
    }

    #[actix_rt::test]
    async fn get_user_info() {
        let(pool, schema) = init_db_and_schema();

        let unique_name = Uuid::new_v4().to_string();

        //Creating first user
        let request1 = create_user(&unique_name);
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : None,
        };
        let u1 = request1.execute(&schema, &db);
        let json = serde_json::to_string(&u1).unwrap();
        let user=  serde_json::from_str::<Response<CreateUser>>(&json);

        let id = user.unwrap().data.unwrap().create_user.id;

        let get_user_request = get_user();

        let config = argon2::Config::default();
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : Some(crate::models::AuthData {
                id: id,
                pass_hash: argon2::hash_encoded(b"123456", b"step3salt", &config).unwrap()
            }),
        };
        let response = get_user_request.execute(&schema, &db);

        let json = serde_json::to_string(&response).unwrap();
        let remove = serde_json::from_str::<Response<GetUser>>(&json);
        println!("{}", json);
        let user = remove.unwrap().data.unwrap().get_user;
        assert_eq!(user.id, id);
        assert_eq!(user.name, unique_name);
        assert_eq!(user.friends.len(), 0);
    }

    #[actix_rt::test]
    async fn add_friend_unauthorized() {
        let (pool, schema) = init_db_and_schema();
        let request = add_friend("test1");
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : None,
        };
        let response = request.execute(&schema, &db);
        let json = serde_json::to_string(&response).unwrap();
        let user=  serde_json::from_str::<Response<crate::models::User>>(&json);

        assert_eq!("Unauthorized", &user.unwrap().errors.unwrap()[0].message);
    }

    #[actix_rt::test]
    async fn remove_friend_unathorized() {
        let (pool, schema) = init_db_and_schema();
        let request = remove_friend("test1");
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : None,
        };
        let response = request.execute(&schema, &db);
        let json = serde_json::to_string(&response).unwrap();
        let user=  serde_json::from_str::<Response<crate::models::User>>(&json);

        assert_eq!("Unauthorized", &user.unwrap().errors.unwrap()[0].message);
    }

    #[actix_rt::test]
    async fn get_user_info_unathorized() {
        let (pool, schema) = init_db_and_schema();
        let request = get_user();
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : None,
        };
        let response = request.execute(&schema, &db);
        let json = serde_json::to_string(&response).unwrap();
        let user=  serde_json::from_str::<Response<crate::models::User>>(&json);

        assert_eq!("Unauthorized", &user.unwrap().errors.unwrap()[0].message);
    }

    #[actix_rt::test]
    async fn add_friend_bad_credentials() {
        let (pool, schema) = init_db_and_schema();
        let request = add_friend("test1");
        let config = argon2::Config::default();
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : Some(crate::models::AuthData {
                id: 1,
                pass_hash: argon2::hash_encoded(b"12345", b"step3salt", &config).unwrap()
            }),
        };
        let response = request.execute(&schema, &db);
        let json = serde_json::to_string(&response).unwrap();
        let user=  serde_json::from_str::<Response<crate::models::User>>(&json);

        assert_eq!("Unauthorized", &user.unwrap().errors.unwrap()[0].message);
    }

    #[actix_rt::test]
    async fn remove_friend_bad_credentials() {
        let (pool, schema) = init_db_and_schema();
        let request = remove_friend("test1");
        let config = argon2::Config::default();
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : Some(crate::models::AuthData {
                id: 1,
                pass_hash: argon2::hash_encoded(b"12345", b"step3salt", &config).unwrap()
            }),
        };
        let response = request.execute(&schema, &db);
        let json = serde_json::to_string(&response).unwrap();
        let user=  serde_json::from_str::<Response<crate::models::User>>(&json);

        assert_eq!("Unauthorized", &user.unwrap().errors.unwrap()[0].message);
    }

    #[actix_rt::test]
    async fn get_user_info_bad_credentials() {
        let (pool, schema) = init_db_and_schema();
        let request = get_user();
        let config = argon2::Config::default();
        let db = crate::models::GraphQLContext {
            connection: pool.get().expect("cannot get database connection"),
            auth_data : Some(crate::models::AuthData {
                id: 1,
                pass_hash: argon2::hash_encoded(b"12345", b"step3salt", &config).unwrap()
            }),
        };
        let response = request.execute(&schema, &db);
        let json = serde_json::to_string(&response).unwrap();
        let user=  serde_json::from_str::<Response<crate::models::User>>(&json);

        assert_eq!("Unauthorized", &user.unwrap().errors.unwrap()[0].message);
    }
}
