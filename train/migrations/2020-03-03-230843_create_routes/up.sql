CREATE TABLE routes (
  route_id TEXT NOT NULL PRIMARY KEY,
  agency_id TEXT,
  route_short_name TEXT,
  route_long_name TEXT,
  route_desc TEXT,
  route_type INTEGER,
  route_url TEXT,
  route_color TEXT,
  route_text_color TEXT
)
