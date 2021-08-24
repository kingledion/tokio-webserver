# tokio-webserver

## How to run

### Create a CouchDB repository

The easiest way to do this is using the [official docker image](https://hub.docker.com/_/couchdb/). Pull the image locally and start it with the following command:
`docker run -e COUCHDB_USER=admin -e COUCHDB_PASSWORD=password -p 5984:5984 -d couchdb`

Verify your db instance is working with `curl http://admin:password@127.0.0.1:5984`

### Initialize the product database

The product database name must be the same as the name set in `src/data.rs` as `PRODUCT_DB`. Initialize the database with the included db setup script, `./db-setup.sh COUCHDB_USER COUCHDB_PASSWORD`. The username and password must be reused from the previous step.

### Build and run the rust executable
 
Note that this package will not compile with rustc version 1.54 due to a [reported linker issue](https://github.com/rust-lang/rust/issues/88246). Install version 1.53 with `rustup toolchain install 1.53`. You can then use v1.53 for this package only with `rustup override set 1.53`. See the current version in use for a given directory, use `rustup show` (or you can simply do `rustc --version`)
