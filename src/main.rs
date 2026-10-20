use foxpaw_habanero::*;
use foxpaw_habanero::err::ServerError;

fn main() -> Result<(), ServerError> {
	let server = Server::bind("127.0.0.1:7878")?;
	server.run(|_| {
		Ok(Response::new(Version::Http11, 200).with_html("<h1>Hello World!</h1>"))
	})
}