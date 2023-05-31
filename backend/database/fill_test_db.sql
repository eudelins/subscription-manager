INSERT INTO Brands (name) VALUES
('Amazon');

INSERT INTO Categories (name) VALUES
('Divertissement'),
('Energie');

INSERT INTO Subscriptions (brand_id, name, price, status) VALUES
(1, 'Prime Video', 10.1, false);

INSERT INTO Belongs_To_Categories (subscription_id, category_id) VALUES
(1, 1),
(1, 2);