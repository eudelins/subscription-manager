CREATE USER subscriptions_db_user WITH ENCRYPTED PASSWORD 'admin';
CREATE DATABASE subscriptions_db WITH OWNER subscriptions_db_user;
GRANT ALL PRIVILEGES ON DATABASE subscriptions_db TO subscriptions_db_user;
\c subscriptions_db;
SET SESSION AUTHORIZATION subscriptions_db_user;
SELECT SESSION_USER, CURRENT_USER;


-- Main tables

CREATE TABLE Brands (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    logo VARCHAR(255)
);

CREATE TABLE Subscriptions (
    id SERIAL PRIMARY KEY,
    brand_id SERIAL NOT NULL,
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
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);


-- Linking tables

CREATE TABLE Belongs_To_Categories (
    subscription_id SERIAL,
    category_id SERIAL,
    FOREIGN KEY (subscription_id) REFERENCES Subscriptions(id), 
    FOREIGN KEY (category_id) REFERENCES Categories(id),
    PRIMARY KEY (subscription_id, category_id)
);


INSERT INTO Brands (name) VALUES
('Netflix'),
('Amazon'),
('EDF'),
('Spotify'),
('Free');

INSERT INTO Categories (name) VALUES
('Energie'),
('Divertissement'),
('Musique');

INSERT INTO Subscriptions (brand_id, name, price, status) VALUES
(1, 'Netflix (basic)', 10.1, false),
(3, 'EDF (basic)', 50, false),
(5, 'Free (basic)', 30, false);