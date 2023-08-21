-- Main tables

CREATE TABLE Brands (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE Subscriptions (
    id SERIAL PRIMARY KEY,
    brand_id SERIAL NOT NULL,
    name VARCHAR(50) NOT NULL,
    price REAL NOT NULL,
    status BOOLEAN NOT NULL,
    FOREIGN KEY (brand_id) REFERENCES Brands(id) ON DELETE RESTRICT
);

CREATE TABLE Categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);


-- Linking tables

CREATE TABLE Belongs_To_Categories (
    subscription_id SERIAL,
    category_id SERIAL,
    FOREIGN KEY (subscription_id) REFERENCES Subscriptions(id) ON DELETE CASCADE, 
    FOREIGN KEY (category_id) REFERENCES Categories(id) ON DELETE CASCADE,
    PRIMARY KEY (subscription_id, category_id)
)