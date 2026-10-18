//! Router
//!
//! Todo: Documentation

use crate::common::{Method, Request, Response};
use crate::err::HttpError;

type RouteHandler = Box<dyn Fn(Request) -> Result<Response, HttpError> + Send + Sync>;

/// `Route`
///
/// Todo: Documentation / Examples
struct Route {
    method: Method,
    path: String,
    handler: RouteHandler,
}

/// `Router`
///
/// Todo: Documentation / Examples
#[derive(Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    /// New
    ///
    /// Create a new empty [`Router`] instance.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::Router;
    ///
    /// let router = Router::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// With Delete
    ///
    /// Add a new Delete route to the router. Note that this consumes and returns
    /// a reference to self, supporting method chaining.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// let router = Router::new()
    ///     .with_delete("/", |req| {
    ///         Ok(Response::new(Version::Http11, 200))
    /// });
    /// ```
    #[must_use]
    pub fn with_delete<F>(self, path: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Request) -> Result<Response, HttpError> + Send + Sync + 'static,
    {
        self.with_route(Method::Delete, path, handler)
    }

    /// With Get
    ///
    /// Add a new Get route to the router. Note that this consumes and returns
    /// a reference to self, supporting method chaining.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// let router = Router::new()
    ///     .with_get("/", |req| {
    ///         Ok(Response::new(Version::Http11, 200))
    /// });
    /// ```
    #[must_use]
    pub fn with_get<F>(self, path: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Request) -> Result<Response, HttpError> + Send + Sync + 'static,
    {
        self.with_route(Method::Get, path, handler)
    }

    /// With Patch
    ///
    /// Add a new Patch route to the router. Note that this consumes and returns
    /// a reference to self, supporting method chaining.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// let router = Router::new()
    ///     .with_patch("/", |req| {
    ///         Ok(Response::new(Version::Http11, 200))
    /// });
    /// ```
    #[must_use]
    pub fn with_patch<F>(self, path: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Request) -> Result<Response, HttpError> + Send + Sync + 'static,
    {
        self.with_route(Method::Patch, path, handler)
    }

    /// With Post
    ///
    /// Add a new Post route to the router. Note that this consumes and returns
    /// a reference to self, supporting method chaining.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// let router = Router::new()
    ///     .with_post("/", |req| {
    ///         Ok(Response::new(Version::Http11, 200))
    /// });
    /// ```
    #[must_use]
    pub fn with_post<F>(self, path: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Request) -> Result<Response, HttpError> + Send + Sync + 'static,
    {
        self.with_route(Method::Post, path, handler)
    }

    /// With Put
    ///
    /// Add a new Put route to the router. Note that this consumes and returns
    /// a reference to self, supporting method chaining.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// let router = Router::new()
    ///     .with_put("/", |req| {
    ///         Ok(Response::new(Version::Http11, 200))
    /// });
    /// ```
    #[must_use]
    pub fn with_put<F>(self, path: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Request) -> Result<Response, HttpError> + Send + Sync + 'static,
    {
        self.with_route(Method::Put, path, handler)
    }

    /// With Route
    ///
    /// Add a new route to the router. Note that this consumes and returns a
    /// reference to self, supporting method chaining.
    ///
    /// # Examples
    /// ```rust
    /// use foxpaw_habanero::*;
    ///
    /// let router = Router::new()
    ///     .with_route(Method::Options, "/", |req| {
    ///         Ok(Response::new(Version::Http11, 200))
    /// });
    /// ```
    #[must_use]
    pub fn with_route<F>(mut self, method: Method, path: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Request) -> Result<Response, HttpError> + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method,
            path: path.into(),
            handler: Box::new(handler),
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Response, Version};

    #[test]
    fn router_new_correct() {
        let actual = Router::new();
        assert_eq!(actual.routes.len(), 0)
    }

    #[test]
    fn router_with_delete_correct() {
        let actual = Router::new().with_delete("/", |_| Ok(Response::new(Version::Http11, 200)));
        assert_eq!(actual.routes.len(), 1);
        assert_eq!(actual.routes[0].method, Method::Delete);
    }

    #[test]
    fn router_with_get_correct() {
        let actual = Router::new().with_get("/", |_| Ok(Response::new(Version::Http11, 200)));
        assert_eq!(actual.routes.len(), 1);
        assert_eq!(actual.routes[0].method, Method::Get);
    }

    #[test]
    fn router_with_patch_correct() {
        let actual = Router::new().with_patch("/", |_| Ok(Response::new(Version::Http11, 200)));
        assert_eq!(actual.routes.len(), 1);
        assert_eq!(actual.routes[0].method, Method::Patch);
    }

    #[test]
    fn router_with_post_correct() {
        let actual = Router::new().with_post("/", |_| Ok(Response::new(Version::Http11, 200)));
        assert_eq!(actual.routes.len(), 1);
        assert_eq!(actual.routes[0].method, Method::Post);
    }

    #[test]
    fn router_with_put_correct() {
        let actual = Router::new().with_put("/", |_| Ok(Response::new(Version::Http11, 200)));
        assert_eq!(actual.routes.len(), 1);
        assert_eq!(actual.routes[0].method, Method::Put);
    }

    #[test]
    fn router_with_route_correct() {
        let actual = Router::new().with_route(Method::Get, "/", |_| {
            Ok(Response::new(Version::Http11, 200))
        });
        assert_eq!(actual.routes.len(), 1);
    }
}
