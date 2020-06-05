#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;
extern crate dotenv;


use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_web::{get, post, delete, web, App, HttpServer, Responder,HttpResponse};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

        actix_web::HttpServer::new(move ||
            actix_web::App::new()
                .data(pool.clone())
                .app_data(web::PayloadConfig::new(32 * 1024 * 1024 * 1024))
                .app_data(web::JsonConfig::default().limit(1024 * 1024 * 32 * 1024))
                .service(get_articles)
                .service(add_article)
                .service(get_article)
                .service(delete_article)
            )
            .bind("127.0.0.1:8080")?
            .run()
            .await
}
#[get("/articles")]
async fn get_articles(
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::articles::dsl::*;
    use crate::schema::labels::dsl::*;
    use diesel::prelude::BelongingToDsl;

    let db_connection  = pool.get().unwrap();
    let _articles = articles.load::<Article>(&db_connection).unwrap();
    let res = Label::belonging_to(&_articles).load::<Label>(&db_connection).unwrap();
    let _labels = labels.load::<Label>(&db_connection).unwrap();
    let mut res = Vec::new();
    for _a in _articles {
        res.push(ArticleDTO {
            id: Some(_a.id.clone()),
            title: _a.title.clone(),
            body: _a.body.clone(),
            labels: _labels.iter().filter(|x|x.article_id == _a.id).map(|x|x.name.clone()).collect()
        })
    }

    serde_json::to_string(&res)
}


#[get("/article/{id}")]
async fn get_article(
    pool: web::Data<DbPool>,
    info: web::Path<uuid::Uuid>,
) -> impl Responder {
    use crate::schema::articles::dsl::*;
    use crate::schema::labels::dsl::*;
    use diesel::prelude::BelongingToDsl;

    let a_id = info.into_inner().to_string();


    let db_connection  = pool.get().unwrap();

    let article = articles.filter(crate::schema::articles::dsl::id.eq(&a_id)).first::<Article>(&db_connection);
    match article {
        Ok(_a) => {
            let _labels = Label::belonging_to(&_a).load::<Label>(&db_connection).unwrap();
            let _article = ArticleDTO {
                title:  _a.title,
                body: _a.body,
                id: Some(_a.id),
                labels: _labels.into_iter().map(|x|x.name).collect(),
            };
            HttpResponse::Ok().body(serde_json::to_string(&_article).unwrap())
        },
        Err(_) => HttpResponse::NotFound().finish()
    }
}


#[post("/article")]
async fn add_article(
    pool: web::Data<DbPool>,
    mut model: web::Json<ArticleDTO>,
) -> impl Responder {
    use crate::schema::articles::dsl::*;
    use crate::schema::labels::dsl::*;

    let db_connection  = pool.get().unwrap();

    let article = Article {
        id: uuid::Uuid::new_v4().to_string(),
        title: model.title.clone(),
        body: model.body.clone(),
    };
    let result = diesel::insert_into(articles).values(&article).execute(&db_connection);
    for label in &model.labels {
        let _l = NewLabel {
            name: label.clone(),
            article_id: article.id.clone(),
        };
        diesel::insert_into(labels).values(&_l).execute(&db_connection);
    }

    serde_json::to_string(&article)
}

#[delete("/article/{id}")]
async fn delete_article(
    pool: web::Data<DbPool>,
    info: web::Path<uuid::Uuid>,
) -> impl Responder {

    use crate::schema::articles::dsl::*;
    use crate::schema::labels::dsl::*;

    let a_id = info.into_inner().to_string();
    let db_connection  = pool.get().unwrap();

    diesel::delete(labels).filter(article_id.eq(&a_id)).execute(&db_connection);
    diesel::delete(articles).filter(crate::schema::articles::dsl::id.eq(&a_id)).execute(&db_connection);

    HttpResponse::Ok()
}


mod schema;
use schema::*;
    #[derive(Identifiable, Serialize, Deserialize, Queryable, Insertable, Associations)]
    struct Article {
        pub id: String,
        pub title: String,
        pub body: String
    }

    #[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Insertable, Associations)]
    #[belongs_to(Article, foreign_key = "article_id")]
    struct Label {
        pub id: i32,
        pub name: String,
        pub article_id: String,
    }
    #[derive(Debug, Serialize, Deserialize, Insertable, Associations)]
    #[table_name="labels"]
    struct NewLabel {
        pub name: String,
        pub article_id: String,
    }


    #[derive(Serialize, Deserialize)]
    struct ArticleDTO {
        pub id: Option<String>,
        pub title: String,
        pub body: String,

        pub labels: Vec<String>,
    }
