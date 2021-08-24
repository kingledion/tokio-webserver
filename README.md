# tokio-webserver

## How to setup and run

### Create a CouchDB repository

The easiest way to do this is using the [official docker image](https://hub.docker.com/_/couchdb/). Pull the image locally and start it with the following command:
`docker run -e COUCHDB_USER=admin -e COUCHDB_PASSWORD=password -p 5984:5984 -d couchdb`

Verify your db instance is working with `curl http://admin:password@127.0.0.1:5984`

### Initialize the product database

The product database name must be the same as the name set in `src/data.rs` as `PRODUCT_DB`. Initialize the database with the included db setup script, `./db-setup.sh COUCHDB_USER COUCHDB_PASSWORD`. The username and password must be reused from the previous step.

### Build and run the rust executable
 
Note that this package will not compile with rustc version 1.54 due to a [reported linker issue](https://github.com/rust-lang/rust/issues/88246). Install version 1.53 with `rustup toolchain install 1.53`. You can then use v1.53 for this package only with `rustup override set 1.53`. See the current version in use for a given directory, use `rustup show` (or you can simply do `rustc --version`).

I never got around to injecting the settings through environmental variables, so you must manually set the COUCHDB_USER and COUCHDB_PASSWORD via `./config/Local.toml`. Don't commit your passwords! 

Buid and run locally with `cargo run`.

There exists a docker image, but since I never finished implementing settings loaded from environmental variables the docker image does not work. The docker image is intended to be used with K3S pushing the contents of the `./config/Local.tomal` into the deployment yaml file. 

## How to use

### GET `/test`

A test endpoint to validate the operation of the executable. 

### GET `/product/{product-id}`

Returns the name and price of the product with the given product-id with http code `200`. Example:
```
{
  "id": "125",
  "name": "cyclostyling snaffling",
  "value": 6925,
  "currency_code": "USD"
}
```

To emulate a call to external HTTP resource, the name is generated from a random number generator. Therefore, it is not consistent from call to call. The price information is retrieved from a CouchDB instance and is consistent.

If the item cannot be found, will return an http code `400` with error message.

If either the name service is unavailable or any error other than not found is returned from the database, this function will return an http code `500`.

### PUT `/product/{product-id}`

Sets the price for the product with the given id. An example JSON body is:
```
{
	"value": 2456,
	"currency_code": "USD"
}
```

The returned value is the price of the good with the attached id. An example return is: 
```
{
  "_id": "127",
  "value": 2456,
  "currency_code": "USD"
}
```
This operation will work the same whether or not the price corresponding to this id exists or not. 

Any error returned from the database will return an http code `500`.

## TODO

To productionalize this webapp, if I was working on this for anothe week, the following would need to be created:
 - Mocking the database and name client calls (several library options)
 - Testing the webapp using wiremock (`https://docs.rs/wiremock/0.5.6/wiremock/`)
 - Load the config optionally from environment (for cluster deployments) or local toml file (as currently implemented)
 - Validate the docker image works with k8s framework 
 - Developing a logging system based on cluster environment and add warp logging (https://docs.rs/warp/0.1.0/warp/filters/log/index.html)
 - Use github actions to implement deploy pipeline