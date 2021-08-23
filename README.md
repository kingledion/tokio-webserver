# tokio-webserver

## How to run

### Create a CouchDB repository

The easiest way to do this is using the [official docker image](https://hub.docker.com/_/couchdb/). Pull the image locally and start it with the following command:
`docker run -e COUCHDB_USER=admin -e COUCHDB_PASSWORD=password -p 5984:5984 -d couchdb`

Verify your db instance is working with `curl http://admin:password@127.0.0.1:5984`

