CREATE TABLE IF NOT EXISTS volume_statistics (
    "path" String NOT NULL,
    "volume_id" String NULL,
    "timestamp" DateTime NOT NULL,
    "disk_usage" UInt64 NOT NULL, -- bytes
    PRIMARY KEY ("path", "timestamp")
)
ENGINE = MergeTree
ORDER BY ("path", "timestamp");
