#!/bin/sh
docker run --name okidocky-postgres -p 5555:5432 -d -e POSTGRES_PASSWORD=postgres postgres
