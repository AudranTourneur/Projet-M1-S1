CREATE TABLE IF NOT EXISTS topology (
    container_id TEXT PRIMARY KEY, 
    position_x INTEGER NOT NULL,
    position_y INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS images {
    image_id TEXT PRIMARY KEY,
    image_name TEXT NOT NULL,
    docker_hub_response TEXT NOT NULL,
};