CREATE TABLE device(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    type TEXT,
    model TEXT,
    serial TEXT,
    UNIQUE(model, serial)
);

CREATE TABLE device_rest(
    id INTEGER PRIMARY KEY,
    address TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);