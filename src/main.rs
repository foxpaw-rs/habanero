use foxpaw_habanero::err::{HttpError, ServerError};
use foxpaw_habanero::*;
use std::fs;

/// Main
///
/// Main entry point of the application.
fn main() -> Result<(), ServerError> {
    let server = Server::bind("127.0.0.1:7878")?;
    server.run(|req: Request| {
        if let Some(body) = get_page(req.uri) {
            Ok(Response::new(Version::Http11, 200).with_html(body))
        } else {
            Err(HttpError::NotFound)
        }
    })
}

/// Get Page
///
/// Will obtain the html page for the specified uri. The application looks for
/// the file inside the _`/public`_ directory, appending the html extension.
/// If the uri request is for a directory, the index.html page is returned.
/// Additionally, if the index is manually entered into the request, None will
/// be returned. A small sample of how this works, in order of priority:
/// * / => public/index.html
/// * /greetings => public/greetings.html, public/greetings/index.html
/// * /greetings/en => public/greetings/en.html, public/greetings/en/index.html
fn get_page(uri: impl AsRef<str>) -> Option<Vec<u8>> {
    let page = uri.as_ref();
    if page.ends_with("/index") {
        return None;
    }

    if page.ends_with(".css") || page.ends_with(".js") {
        return fs::read(format!("public{}", page)).ok();
    }

    fs::read(format!("public{}.html", page))
        .or_else(|_| fs::read(format!("public{}/index.html", page)))
        .ok()
}
