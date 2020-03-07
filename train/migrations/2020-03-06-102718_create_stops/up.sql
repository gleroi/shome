CREATE TABLE stops (
    stop_id  TEXT NOT NULL PRIMARY KEY,
    stop_name TEXT,
    stop_desc TEXT,
    stop_lat DOUBLE,
    stop_lon DOUBLE,
    zone_id TEXT,
    stop_url TEXT,
    parent_station TEXT
)