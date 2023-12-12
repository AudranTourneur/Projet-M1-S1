-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.


-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```

CREATE TABLE users (
    "id" SERIAL PRIMARY KEY,
    "username" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "salt" TEXT NOT NULL,
    "topology" JSONB NOT NULL,
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);


CREATE TABLE sessions (
    "session_token" TEXT PRIMARY KEY,
    "user_id" INTEGER NOT NULL,
    "expires_at" TIMESTAMP NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE container_statistics (
    "id" TEXT,
    "timestamp" TIMESTAMP NOT NULL,
    "memory_usage" INTEGER NOT NULL, -- bytes
    "cpu_usage" FLOAT NOT NULL, -- %
    "io_usage_read" FLOAT NOT NULL, -- bytes/sec
    "io_usage_write" FLOAT NOT NULL, -- bytes/sec
    "network_usage_up" FLOAT NOT NULL, -- bytes/sec
    "network_usage_down" FLOAT NOT NULL, -- bytes/sec
    PRIMARY KEY ("id", "timestamp")
);


CREATE TABLE volume_statistics (
    "id" TEXT,
    "timestamp" TIMESTAMP NOT NULL,
    "disk_usage" INTEGER NOT NULL, -- bytes
    PRIMARY KEY ("id", "timestamp") 
);

CREATE TABLE network_statistics (
    "id" TEXT,
    "timestamp" TIMESTAMP NOT NULL,
    "network_usage_up" FLOAT NOT NULL, -- bytes/sec
    "network_usage_down" FLOAT NOT NULL, -- bytes/sec
    PRIMARY KEY ("id", "timestamp") 
);

CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
