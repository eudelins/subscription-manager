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