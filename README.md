# Redux Server (Rust Edition)

[![GitHub last commit](https://img.shields.io/github/last-commit/MichaelKaaden/redux-server-rust.svg)](https://github.com/MichaelKaaden/redux-server-rust/commits/master)
[![GitHub tag](https://img.shields.io/github/tag/MichaelKaaden/redux-server-rust.svg)](https://github.com/MichaelKaaden/redux-server-rust/releases)
[![GitHub issues](https://img.shields.io/github/issues/MichaelKaaden/redux-server-rust.svg)](https://github.com/MichaelKaaden/redux-server-rust/issues)
[![license](https://img.shields.io/github/license/MichaelKaaden/redux-server-rust.svg)](https://github.com/MichaelKaaden/redux-server-rust)

This is a tiny REST service managing counters. The counters are kept in memory, so they are reset every time you restart
the service.

Each counter has

- a unique index (a number greater or equal 0) and
- a value.

You can either get or set a counter. Of course, you shouldn't set any counter in a distributed environment. Instead, you
should get it and then use the increment or decrement operations on it. For presentations, it is a reasonable choice to
set some counters before showing anything to your audience.

The RESTful Web Service runs at [http://localhost:3000](http://localhost:3000).

## Running the server

You should have a working Rust installation on your computer. Then, the server will run on http://localhost:3000.

```bash
$ cargo run
```

## Building the Docker Image

The `Dockerfile` in this repository uses a multi-stage build to produce the image containing the binary. To build an
image as small as possible, I chose to build statically against [MUSL](https://en.wikipedia.org/wiki/Musl) and use
an `alpine` image for the runner stage (resulting in an image with about 25 MB).

To build and run the Docker image, use

```shell
$ docker build -t redux-server-rust .
$ docker run -p 3000:3000 --rm -d redux-server-rust
```

The service will now listen on your host's port 3000. 

## Alternative and Corresponding Implementations

This is only one possible solution to this kind of problem.

There are some implementations of single-page applications using the services which are implemented in different
programming languages and frameworks.

Here's the full picture.

## Client-Side Implementations

- [https://github.com/MichaelKaaden/redux-client-ngrx](https://github.com/MichaelKaaden/redux-client-ngrx) (Angular with
  NgRx)
- [https://github.com/MichaelKaaden/redux-client-ng5](https://github.com/MichaelKaaden/redux-client-ng5) (Angular
  with `angular-redux`)
- [https://github.com/MichaelKaaden/redux-client-ng](https://github.com/MichaelKaaden/redux-client-ng) (AngularJS
  with `ng-redux`)

## Server-Side Implementations

- [https://github.com/MichaelKaaden/redux-server-rust](https://github.com/MichaelKaaden/redux-server-rust) (Rust
  with `actix-web`)
- [https://github.com/MichaelKaaden/redux-server-golang](https://github.com/MichaelKaaden/redux-server-golang) (Go
  with `Gin`)
- [https://github.com/MichaelKaaden/redux-server-nest](https://github.com/MichaelKaaden/redux-server-nest) (Node.js
  with `Nest`)
- [https://github.com/MichaelKaaden/redux-server](https://github.com/MichaelKaaden/redux-server) (Node.js with `Exprss`)
