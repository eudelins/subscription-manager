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
(1, 'Netflix (basic)', 10.1, FALSE),
(3, 'EDF (basic)', 50, FALSE),
(5, 'Free (basic)', 30, FALSE);