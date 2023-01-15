#!/bin/sh
docker run --name tea-orm__postgres \
    --rm \
    --env POSTGRES_USER=user \
    --env POSTGRES_DB=tea-orm \
    --env POSTGRES_PASSWORD=password \
    -p 5432:5432 \
    --detach postgres:15.1
