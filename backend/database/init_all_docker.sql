\set db_user_password `echo "$DB_USER_PASSWORD"`
CREATE USER subscriptions_db_user WITH ENCRYPTED PASSWORD :'db_user_password';
CREATE DATABASE subscriptions_db WITH OWNER subscriptions_db_user;
GRANT ALL PRIVILEGES ON DATABASE subscriptions_db TO subscriptions_db_user;
\c subscriptions_db;
SET SESSION AUTHORIZATION subscriptions_db_user;
SELECT SESSION_USER, CURRENT_USER;


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
(1, 'Netflix (basic)', 10.1, true),
(3, 'EDF (basic)', 50, true),
(5, 'Free (basic)', 30, true);

INSERT INTO Belongs_To_Categories (subscription_id, category_id) VALUES
(1, 2),
(2, 1),
(3, 2);
