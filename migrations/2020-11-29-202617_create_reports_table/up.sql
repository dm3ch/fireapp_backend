CREATE TABLE reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description VARCHAR NOT NULL,
    lat FLOAT NOT NULL ,
    lng FLOAT NOT NULL ,
    completed BOOLEAN NOT NULL DEFAULT 0
);
INSERT INTO reports (description, lat, lng) VALUES ('Test: Volhov swamp is in fire', 59.92079346407353, 32.31912547869565);
INSERT INTO reports (description, lat, lng) VALUES ('Test: Some Ladoga lake island is in fire.', 61.57500168623652, 31.371202935182904);
