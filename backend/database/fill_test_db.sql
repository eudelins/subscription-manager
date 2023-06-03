INSERT INTO Brands (name) VALUES
('Amazon'),
('Netflix'),
('EDF');

INSERT INTO Categories (name) VALUES
('Divertissement'),
('Energie');

INSERT INTO Subscriptions (brand_id, name, price, status) VALUES
(1, 'Prime Video', 10.1, true),
(2, 'Netflix', 9.99, true),
(3, 'EDF', 45.5, false);

INSERT INTO Belongs_To_Categories (subscription_id, category_id) VALUES
(1, 1),
(2, 1),
(1, 2);