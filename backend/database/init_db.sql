CREATE USER subscriptions_db_user WITH ENCRYPTED PASSWORD 'admin';
CREATE DATABASE subscriptions_db WITH OWNER subscriptions_db_user;
GRANT ALL PRIVILEGES ON DATABASE subscriptions_db TO subscriptions_db_user;