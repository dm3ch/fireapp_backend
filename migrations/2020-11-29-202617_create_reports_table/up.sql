CREATE TABLE reports (
    uuid UUID NOT NULL PRIMARY KEY,
    description VARCHAR NOT NULL,
    lat FLOAT NOT NULL ,
    lng FLOAT NOT NULL ,
    status VARCHAR NOT NULL
);
