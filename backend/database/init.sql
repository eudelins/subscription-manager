-- Main tables

CREATE TABLE Brands (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL,
    logo VARCHAR(255)
);

CREATE TABLE Subscriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    brand_id INTEGER NOT NULL,
    name VARCHAR(50) NOT NULL,
    price REAL NOT NULL,
    status BOOLEAN NOT NULL,
    logo VARCHAR(255),
    subscribe_link VARCHAR(255),
    unsubscribe_link VARCHAR(255),
    start_date DATE,
    end_date DATE,
    FOREIGN KEY (brand_id) REFERENCES Brands(id)
);

CREATE TABLE Categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL
);


-- Linking tables

CREATE TABLE Belongs_To_Categories (
    subscription_id INTEGER,
    category_id INTEGER,
    FOREIGN KEY (subscription_id) REFERENCES Subscriptions(id), 
    FOREIGN KEY (category_id) REFERENCES Categories(id),
    PRIMARY KEY (subscription_id, category_id)
)