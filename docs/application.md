# Habanero Application
This document aims to walk you through the installation and use of the Habanero application to use it as a web server.

## Installation
To compile the repository, please first install the [rust toolchain](https://rust-lang.org/tools/install/). Clone or download the repository, navigate to it in a terminal and run `cargo build --release` to compile the application. This will place the application executable under the _`target/release/`_ directory, called _`foxpaw_habanero`_ or _`foxpaw-habanero.exe`_ depending on your operating system.

Currently Habanero does not support TLS, therefore, does not support HTTPS. It is currently highly recommended to run your application behind a traditional web server using a reverse proxy. 

## View Pages
Execute the _`foxpaw-habanero`_ application to run the server. Once your server is running, its important to understand where it looks for pages to display, and how the URL scheme works.

All the following references are from your current working directory. It is recommended to place the application executable as a sibling to the _`public/`_ directory the application searches for.
my-project/
```
my-project/
├── foxpaw-habanero
└── public/
    ├── index.html
    ├── other.html
    ├── my-subdirectory/
    |    ├── index.html
    |	 └── other.html
    ├── styles.css
    └── script.js
```

Any _`.html`_ file within the _`public/`_ directory will now be accessible via the localhost with a port of 7878 on whatever browser you use (`127.0.0.1:7878`). The URL scheme expects no file extension on the page (`127.0.0.1:7878/other`). The server will automatically use the _`index`_ file for folder roots. The search hierarchy in order is:
* / => public/index.html
* /greetings => public/greetings.html, public/greetings/index.html
* /greetings/en => public/greetings/en.html, public/greetings/en/index.html

The application will serve any `.css` or `.js` files from any path within the _`public/`_ directory.