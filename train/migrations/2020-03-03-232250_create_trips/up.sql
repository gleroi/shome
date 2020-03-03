CREATE TABLE trips (
    route_id  TEXT,
    service_id  INTEGER,
    trip_id  TEXT NOT NULL PRIMARY KEY,
    trip_headsign  TEXT,
    direction_id  INTEGER,
    block_id  TEXT,
    shape_id  TEXT,
    FOREIGN KEY(route_id) REFERENCES routes(route_id)
)