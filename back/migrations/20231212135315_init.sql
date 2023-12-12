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
