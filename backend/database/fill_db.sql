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