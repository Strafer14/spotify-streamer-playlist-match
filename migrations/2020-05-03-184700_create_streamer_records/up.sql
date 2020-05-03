CREATE TABLE streamer_records (
  id SERIAL PRIMARY KEY,
  viewers INT NOT NULL DEFAULT 0,
  followersgained INT NOT NULL DEFAULT 0,
  rownum INT,
  logo VARCHAR NOT NULL,
  twitchurl VARCHAR NOT NULL,
  url VARCHAR NOT NULL,
  displayname VARCHAR NOT NULL
)