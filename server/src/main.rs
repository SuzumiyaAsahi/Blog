#[path = "../article/mod.rs"]
mod article;
#[path = "../comment/mod.rs"]
mod comment;
#[path = "./errors.rs"]
mod errors;
#[path = "../models/mod.rs"]
mod models;
#[path = "../user/mod.rs"]
mod user;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use article::{delete, edit, new, search, view};
use env_logger::Env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, io, sync::Arc};
use user::login;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let db_url = env::var("DATABASE_URL").expect("Please set 'DATABASE_URL'");
    let app_state = Arc::new(AppState {
        db_pool: PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .unwrap(),
    });
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(Arc::clone(&app_state)))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .configure(route)
    })
    .bind("127.0.0.1:12345")?
    .run()
    .await
}

fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/article")
            .route("/{id}", web::get().to(view::get_article))
            .route("", web::post().to(new::new_article))
            .route("", web::put().to(edit::edit_article))
            .route("/{id}", web::delete().to(delete::delete_article))
            .route("/search/{keyword}", web::get().to(search::search_article)),
    )
    .service(web::scope("/articles").route("", web::get().to(view::get_articles_preview)))
    .service(
        web::scope("/user")
            .route("/login", web::post().to(login::github_login))
            .route("/info", web::get().to(user::info::get_user_info)),
    )
    .service(
        web::scope("/comment")
            .route(
                "/{article_id}",
                web::get().to(comment::view::get_comments_for_article),
            )
            .route("", web::post().to(comment::new::new_comment))
            .route(
                "/{comment_id}",
                web::delete().to(comment::delete::delete_comment),
            ),
    );
}
