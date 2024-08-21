
## RUST-AXUM-REDIS Service

Example of REST API, reading and writing from Cassandra, written in Rust and using the Axum framework


### Build service docker
```
docker build --progress=plain -t rust-axum-cassandra .
```


### Run Cassandra
```
docker run --rm --name cassandra -p 9042:9042 -d cassandra

```

### Create Keyspace and Table

```
docker exec -it cassandra cqlsh

CREATE KEYSPACE rust  WITH replication = {'class':'SimpleStrategy', 'replication_factor' : 1};

CREATE TABLE rust.test (test_id text, test_bool boolean, test_set set<int>, test_map map<text, int>, PRIMARY KEY (test_id));

```

### Start service docker
```
docker run --rm -d --network host --name my-rust-axum-cassandra rust-axum-cassandra
```