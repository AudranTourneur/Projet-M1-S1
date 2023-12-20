CREATE TABLE IF NOT EXISTS volume_statistics (
    "id" String,
    "timestamp" DateTime,
    "memory_usage" UInt64, -- bytes
    PRIMARY KEY ("id", "timestamp")
)
ENGINE = MergeTree
ORDER BY ("id", "timestamp");
