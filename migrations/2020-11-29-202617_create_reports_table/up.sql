CREATE TABLE reports (
    uiid UUID NOT NULL PRIMARY KEY,
    description VARCHAR NOT NULL,
    lat FLOAT NOT NULL ,
    lng FLOAT NOT NULL ,
    status VARCHAR NOT NULL DEFAULT 'new'
);

INSERT INTO reports (uiid, description, lat, lng) VALUES ('86e3aed3-1553-5d23-8d61-2286215e65f1', 'Test: Volhov swamp is in fire', 59.92079346407353, 32.31912547869565);
INSERT INTO reports (uiid, description, lat, lng) VALUES ('6eabff02-c968-5cbc-bc7f-3b672928a761', 'Test: Some Ladoga lake island is in fire.', 61.57500168623652, 31.371202935182904);
