use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use yew::callback::Callback;
use yew::services::fetch::FetchTask;
use log::debug;

use super::{limit, Requests};
use crate::error::Error;
use crate::types::*;

/// Apis for articles
#[derive(Default, Debug)]
pub struct Articles {
    requests: Requests,
}

impl Articles {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Get all articles
    pub fn all(
        &mut self,
        page: u32,
        callback: Callback<Result<ArticleListInfo, Error>>,
    ) -> FetchTask {
        debug!("Articles all");
        self.requests
            .get::<ArticleListInfo>(format!("/articles?{}", limit(10, page)), callback)
    }

    /// Get articles filtered by author
    pub fn by_author(
        &mut self,
        author: String,
        page: u32,
        callback: Callback<Result<ArticleListInfo, Error>>,
    ) -> FetchTask {
        self.requests.get::<ArticleListInfo>(
            format!("/articles?author={}&{}", query_encode(&author), limit(10, page)),
            callback,
        )
    }

    /// Get articles filtered by tag
    pub fn by_tag(
        &mut self,
        tag: String,
        page: u32,
        callback: Callback<Result<ArticleListInfo, Error>>,
    ) -> FetchTask {
        self.requests.get::<ArticleListInfo>(
            format!("/articles?tag={}&{}", query_encode(&tag), limit(10, page)),
            callback,
        )
    }

    /// Delete an article
    pub fn del(
        &mut self,
        slug: String,
        callback: Callback<Result<DeleteWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .delete::<DeleteWrapper>(format!("/articles/{}", slug), callback)
    }

    /// Favorite and article
    pub fn favorite(
        &mut self,
        slug: String,
        callback: Callback<Result<ArticleInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests.post::<(), ArticleInfoWrapper>(
            format!("/articles/{}/favorite", slug),
            (),
            callback,
        )
    }

    /// Unfavorite an article
    pub fn unfavorite(
        &mut self,
        slug: String,
        callback: Callback<Result<ArticleInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .delete::<ArticleInfoWrapper>(format!("/articles/{}/favorite", slug), callback)
    }

    /// Get articles favorited by an author
    pub fn favorited_by(
        &mut self,
        author: String,
        page: u32,
        callback: Callback<Result<ArticleListInfo, Error>>,
    ) -> FetchTask {
        self.requests.get::<ArticleListInfo>(
            format!("/articles?favorited={}&{}", query_encode(&author), limit(10, page)),
            callback,
        )
    }

    /// Get feed of articles
    pub fn feed(&mut self, callback: Callback<Result<ArticleListInfo, Error>>) -> FetchTask {
        self.requests
            .get::<ArticleListInfo>(format!("/articles/feed?{}", limit(10, 0)), callback)
    }

    /// Get an article
    pub fn get(
        &mut self,
        slug: String,
        callback: Callback<Result<ArticleInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .get::<ArticleInfoWrapper>(format!("/articles/{}", slug), callback)
    }

    /// Update an article
    pub fn update(
        &mut self,
        slug: String,
        article: ArticleCreateUpdateInfoWrapper,
        callback: Callback<Result<ArticleInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .put::<ArticleCreateUpdateInfoWrapper, ArticleInfoWrapper>(
                format!("/articles/{}", slug),
                article,
                callback,
            )
    }

    /// Create an article
    pub fn create(
        &mut self,
        article: ArticleCreateUpdateInfoWrapper,
        callback: Callback<Result<ArticleInfoWrapper, Error>>,
    ) -> FetchTask {
        self.requests
            .post::<ArticleCreateUpdateInfoWrapper, ArticleInfoWrapper>(
                "/articles".to_string(),
                article,
                callback,
            )
    }
}

/// Encode s for use as a query parameter value in a URL.
fn query_encode(s: &str) -> String {
    // The application/x-www-form-urlencoded percent-encode set. See
    // https://url.spec.whatwg.org/#application-x-www-form-urlencoded-percent-encode-set
    const QUERY: &AsciiSet = &CONTROLS
        .add(b' ').add(b'"').add(b'#').add(b'<').add(b'>')
        .add(b'?').add(b'`').add(b'{').add(b'}')
        .add(b'/').add(b':').add(b';').add(b'=').add(b'@')
        .add(b'[').add(b'\\').add(b']').add(b'^').add(b'|')
        .add(b'$').add(b'%').add(b'&').add(b'+').add(b',')
        .add(b'!').add(b'\'').add(b'(').add(b')').add(b'~');
    return utf8_percent_encode(s, QUERY).to_string();
}
