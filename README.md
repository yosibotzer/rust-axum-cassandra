
```

docker run --rm --name cassandra -p 9042:9042 -d cassandra

docker exec -it cassandra cqlsh

CREATE KEYSPACE rust  WITH replication = {'class':'SimpleStrategy', 'replication_factor' : 1};

CREATE TABLE rust.test (test_id text, test_bool boolean, test_set set<int>, test_map map<text, int>, PRIMARY KEY (test_id));

```