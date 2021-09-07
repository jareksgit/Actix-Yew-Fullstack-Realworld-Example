pub mod comments;

use actix_web::{web::Data, web::Json, web::Path, web::Query, HttpRequest, HttpResponse, ResponseError};
use actix_web::{middleware::Logger};
use validator::Validate;

use super::AppState;
use crate::app::profiles::ProfileResponseInner;
use crate::prelude::*;
use crate::utils::{
    auth::{authenticate, Auth},
    CustomDateTime,
};

#[derive(Debug, Deserialize)]
pub struct In<T> {
    article: T,
}

// Extractors ↓

#[derive(Debug, Deserialize)]
pub struct ArticlePath {
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct ArticlesParams {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<usize>,  // <- if not set, is 20
    pub offset: Option<usize>, // <- if not set, is 0
}

#[derive(Debug, Deserialize)]
pub struct FeedParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

// Client Messages ↓

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticle {
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub description: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub body: String,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub tag_list: Vec<String>,
}

#[derive(Debug)]
pub struct CreateArticleOuter {
    pub auth: Auth,
    pub article: CreateArticle,
}

#[derive(Debug)]
pub struct GetArticle {
    pub auth: Option<Auth>,
    pub slug: String,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticle {
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub title: Option<String>,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub description: Option<String>,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub body: Option<String>,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub tag_list: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct UpdateArticleOuter {
    pub auth: Auth,
    pub slug: String,
    pub article: UpdateArticle,
}

#[derive(Debug)]
pub struct DeleteArticle {
    pub auth: Auth,
    pub slug: String,
}

#[derive(Debug)]
pub struct FavoriteArticle {
    pub auth: Auth,
    pub slug: String,
}

#[derive(Debug)]
pub struct UnfavoriteArticle {
    pub auth: Auth,
    pub slug: String,
}

#[derive(Debug)]
pub struct GetArticles {
    pub auth: Option<Auth>,
    pub params: ArticlesParams,
}

#[derive(Debug)]
pub struct GetFeed {
    pub auth: Auth,
    pub params: FeedParams,
}

// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct ArticleResponse {
    pub article: ArticleResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponseInner {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
    pub favorited: bool,
    pub favorites_count: usize,
    pub author: ProfileResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleListResponse {
    pub articles: Vec<ArticleResponseInner>,
    pub articles_count: usize,
}

// Route handlers ↓

pub async fn create(
    state: Data<AppState>,
    (form, req): (Json<In<CreateArticle>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    Logger::new(format!("create new article {:?}", form).as_ref());

    let article = form.into_inner().article;
    let db = state.db.clone();

    article.validate()?;

    let auth = authenticate(&state, &req).await?;

    match db.send(CreateArticleOuter { auth, article }).await? {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn get(
    state: Data<AppState>,
    (path, req): (Path<ArticlePath>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let db = state.db.clone();

    let auth = authenticate(&state, &req).await;

    match db
        .send(GetArticle {
            auth: auth.ok(),
            slug: path.slug.to_owned(),
        })
        .await?
    {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn update(
    state: Data<AppState>,
    (path, form, req): (Path<ArticlePath>, Json<In<UpdateArticle>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let article = form.into_inner().article;

    let db = state.db.clone();
    let auth = authenticate(&state, &req).await?;
    match db
        .send(UpdateArticleOuter {
            auth,
            slug: path.slug.to_owned(),
            article,
        })
        .await?
    {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn delete(
    state: Data<AppState>,
    (path, req): (Path<ArticlePath>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    match state
        .db
        .send(DeleteArticle {
            auth,
            slug: path.slug.to_owned(),
        })
        .await?
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn favorite(
    state: Data<AppState>,
    (path, req): (Path<ArticlePath>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    match state
        .db
        .send(FavoriteArticle {
            auth,
            slug: path.slug.to_owned(),
        })
        .await?
    {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn unfavorite(
    state: Data<AppState>,
    (path, req): (Path<ArticlePath>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    match state
        .db
        .send(UnfavoriteArticle {
            auth,
            slug: path.slug.to_owned(),
        })
        .await?
    {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn list(
    state: Data<AppState>,
    (req, params): (HttpRequest, Query<ArticlesParams>),
) -> Result<HttpResponse, Error> {
    let db = state.db.clone();

    let auth = authenticate(&state, &req).await;
    match db
        .send(GetArticles {
            auth: auth.ok(),
            params: params.into_inner(),
        })
        .await?
    {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(e.error_response()),
    }
}

pub async fn feed(
    state: Data<AppState>,
    (req, params): (HttpRequest, Query<FeedParams>),
) -> Result<HttpResponse, Error> {
    let db = state.db.clone();

    let auth = authenticate(&state, &req).await?;
    match db
        .send(GetFeed {
            auth,
            params: params.into_inner(),
        })
        .await?
    {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(e.error_response()),
    }
}
