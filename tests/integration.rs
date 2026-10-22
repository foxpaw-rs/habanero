#[cfg(test)]
mod tests {
    use foxpaw_habanero::prelude::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn client_server_found() {
        // 1. Set up Router
        let router = Router::new().with_get("/hello", |_| {
            Ok(Response::new(Version::Http11, 200).with_body("Hello from Rust!"))
        });

        // 2. Set up the server, run in background
        let server = Server::bind("127.0.0.1:7000").unwrap();
        thread::spawn(move || server.run(router).unwrap());

        // 3. Allow server to spin up
        thread::sleep(Duration::from_millis(50));

        // 4. Create client and send request
        let client = Client::new();
        let res = client
            .get(&"http://127.0.0.1:7000/hello".parse().unwrap())
            .unwrap();

        // 5. Assert expected results
        assert_eq!(res.version, Version::Http11);
        assert_eq!(res.status, 200);
        assert_eq!(res.body, b"Hello from Rust!");
    }

    #[test]
    fn client_server_not_found() {
        // 1. Set up Router
        let router = Router::new().with_get("/hello", |_| {
            Ok(Response::new(Version::Http11, 200).with_body("Hello from Rust!"))
        });

        // 2. Set up the server, run in background
        let server = Server::bind("127.0.0.1:7001").unwrap();
        thread::spawn(move || server.run(router).unwrap());

        // 3. Allow server to spin up
        thread::sleep(Duration::from_millis(50));

        // 4. Create client and send request
        let client = Client::new();
        let res = client
            .get(&"http://127.0.0.1:7001/".parse().unwrap())
            .unwrap();

        // 5. Assert expected results
        assert_eq!(res.version, Version::Http11);
        assert_eq!(res.status, 404);
        assert_eq!(res.body, b"");
    }
}
