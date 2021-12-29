#!/usr/bin/env bash

POSTGRES_CONFIG='{
  "name": "postgres-connector",
  "config": {
    "connector.class": "io.debezium.connector.postgresql.PostgresConnector",
    "database.hostname": "database_dev",
    "database.port": "5432",
    "database.user": "postgres",
    "database.password": "postgres",
    "database.dbname" : "postgres",
    "database.server.name": "app"
  }
}'

echo "Waiting for Kafka Connect to start listening"

while [ $(curl -s -o /dev/null -w %{http_code} http://connect:8083/connectors) -eq 000 ] ; do
  echo -e $(date) " Kafka Connect listener not yet available"
  sleep 5
done

echo $POSTGRES_CONFIG
curl -i -X POST -H "Accept:application/json" -H "Content-Type:application/json" connect:8083/connectors/ -d "$POSTGRES_CONFIG"
