CREATE TABLE IF NOT EXISTS container_statistics (
    "id" String,
    "timestamp" DateTime,
    "memory_usage" UInt64, -- bytes
    "cpu_usage" Float32, -- %
    "io_usage_read" Float32, -- bytes/sec
    "io_usage_write" Float32, -- bytes/sec
    "network_usage_up" Float32, -- bytes/sec
    "network_usage_down" Float32, -- bytes/sec
    PRIMARY KEY ("id", "timestamp")
)
ENGINE = MergeTree
ORDER BY ("id", "timestamp")


CREATE TABLE IF NOT EXISTS volume_statistics (
    "id" String,
    "timestamp" DateTime,
    "memory_usage" UInt64, -- bytes
    PRIMARY KEY ("id", "timestamp")
)
ENGINE = MergeTree
ORDER BY ("id", "timestamp")