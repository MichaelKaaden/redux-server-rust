# Redux Server (Rust Edition)

[![GitHub last commit](https://img.shields.io/github/last-commit/MichaelKaaden/redux-server-rust.svg)](https://github.com/MichaelKaaden/redux-server-rust/commits/master)
[![GitHub tag](https://img.shields.io/github/tag/MichaelKaaden/redux-server-rust.svg)](https://github.com/MichaelKaaden/redux-server-rust/releases)
[![GitHub version](https://img.shields.io/github/package-json/v/MichaelKaaden/redux-server-rust.svg)](https://github.com/MichaelKaaden/redux-server-rust/blob/master/package.json)
[![Build Status](https://travis-ci.com/MichaelKaaden/redux-server-rust.svg?branch=master)](https://travis-ci.com/MichaelKaaden/redux-server-rust)
[![dependencies](https://img.shields.io/david/MichaelKaaden/redux-server-rust.svg)](https://david-dm.org/MichaelKaaden/redux-server-rust)
[![devDependencies](https://img.shields.io/david/dev/MichaelKaaden/redux-server-rust.svg)](https://david-dm.org/MichaelKaaden/redux-server-rust?type=dev)
[![GitHub issues](https://img.shields.io/github/issues/MichaelKaaden/redux-server-rust.svg)](https://github.com/MichaelKaaden/redux-server-rust/issues)
[![license](https://img.shields.io/github/license/MichaelKaaden/redux-server-rust.svg)](https://github.com/MichaelKaaden/redux-server-rust)

This is a tiny REST service managing counters. The counters
are kept in memory, so they are reset every time you restart
the service.

Each counter has
- a unique index (a number greater or equal 0) and
- a value.

You can either get or set a counter. Of course, you shouldn't
set any counter in a distributed environment. Instead, you
should get it and then use the increment or decrement operations
on it. For presentations, it is a reasonable choice to set
some counters before showing anything to your audience.

The RESTful Web Service runs at [http://localhost:3000](http://localhost:3000).

The client side to this service resides in
[https://github.com/MichaelKaaden/redux-client-ngrx](https://github.com/MichaelKaaden/redux-client-ngrx).

## Running the server

You should have a working Rust installation on your computer.
Then, the server will run on http://localhost:3000.

```bash
$ cargo run
```
