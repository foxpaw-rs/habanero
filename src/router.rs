//! Router
//!
//! This module houses the [`Router`] type, designed to implement the
//! [`Handler`] trait.
//!
//! # Architecture
//! This module exposes the [`Router`] type, an implementor of the [`Handler`].
//! It is designed to be provided to a [`Server`] to support more complex
//! routing.
//!
//! # Examples
//! ```rust
//! use foxpaw_habanero::*;
//!
//! let router = Router::new()
//!     .with_get("/", |req| Ok(Response::new(Version::Http11, 200)));
//! ```   

use crate::common::{Method, Request, Response};
use crate::err::HttpError;
use crate::server::Handler;

/// `RouteHandler`
///
/// The handler type alias for the callback used by the [`Route`].
type RouteHandler = Box<dyn Fn(Request) -> Result<Response, HttpError> + Send + Sync>;

/// `Route`
///
/// A single route housed by the router, defining the supporting [`Method`],
/// path and callback handler.
struct Route {
    method: Method,
    path: String,
    handler: RouteHandler,
}

/// `Router`
///
/// The `Router` it utilised as a more complex handler than a simple callback
/// method. The `Router` currently only supports defining a [`Method`] and
/// [`Path`], however, in future will support more complex features like route
/// bindings, dynamic paths, groups and middleware.
///
/// # Examples
/// ```rust
/// use foxpaw_habanero::*;
///
/// let router = Router::new()
///     .with_get("/", |req| Ok(Response::new(Version::Http11, 200)));
/// ```
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

impl Handler for Router {
    /// Handle
    ///
    /// Handle an incoming [`Request`] and return a [`Response`].
    ///
    /// # Errors
    /// This method can error for any reason by the implementing busines logic.
    ///
    /// # Examples
    /// ```rust, no_run
    /// use foxpaw_habanero::*;
    /// use foxpaw_habanero::err::ServerError;
    ///
    /// fn start_server() -> Result<(), ServerError> {
    ///     let server = Server::bind("127.0.0.1:8080")?;
    ///     let router = Router::new()
    ///         .with_get("/", |_| Ok(Response::new(Version::Http11, 200)))
    ///         .with_get("/about", |_| Ok(Response::new(Version::Http11, 200)));
    ///     server.run(router)?;
    ///     Ok(())
    /// }
    /// ```
    fn handle(&self, req: Request) -> Result<Response, HttpError> {
        self.routes
            .iter()
            .find(|route| route.method == req.method && route.path == req.uri)
            .map_or_else(|| Err(HttpError::NotFound), |route| (route.handler)(req))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Response, Version};

    // impl Router

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
