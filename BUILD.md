# Building the app with Docker

```bash
# Configure database passwords
export PGPASSFILE=./backend/database/pass.pgpass
echo "*:*:*:subscriptions_db_user:<password>" > $PGPASSFILE
chmod 0600 $PGPASSFILE  # Restrict access write to the file
echo "DB_USER_PASSWORD=<password>" > .env.production.db
echo "POSTGRES_PASSWORD=<another_password>" >> .env.production.db

# Configure /etc/hosts
sudo echo "<machine_ip>	subscription-manager" >> /etc/hosts

# Start the app
docker compose up -d  # Could take several minutes on th first launch
                      # You can access the app on http://<machine_ip>:1420/

docker compose down   # Stops the app
```

<br />

# Building the app locally 

## Database init

### Configure postgres to allow user local connection with a password

```bash
sudo nano /etc/postgresql/14/main/pg_hba.conf

# Replace peer by scram-sha-256 :
local   all             all                                     scram-sha-256
```

### Create a postgres user and the database

```bash
sudo -u postgres createuser subscriptions_db_user -P -l
# Enter the subscriptions_db_user password you want

sudo -u postgres createdb subscriptions_db -O subscriptions_db_user
sudo -u postgres createdb subscriptions_db_test -O subscriptions_db_user
```

### Write the created user password in a pass.pgpass file and the .env.production.db file

```bash
export PGPASSFILE=./backend/database/pass.pgpass
echo "*:*:*:subscriptions_db_user:<password>" > $PGPASSFILE
chmod 0600 $PGPASSFILE  # Restrict access write to the file

echo "DB_USER_PASSWORD=<password>" > .env.production.db
echo "POSTGRES_PASSWORD=<another_password>" >> .env.production.db
```

### Run the initialization scripts with the created user

```bash
psql -d subscriptions_db -U subscriptions_db_user -a -f ./backend/database/init_table.sql
psql -d subscriptions_db -U subscriptions_db_user -a -f ./backend/database/fill_db.sql

psql -d subscriptions_db_test -U subscriptions_db_user -a -f ./backend/database/init_table.sql
psql -d subscriptions_db_test -U subscriptions_db_user -a -f ./backend/database/fill_test_db.sql
```
<br>


## Frontend init

### Linter

Follow https://dev.to/knowankit/setup-eslint-and-prettier-in-react-app-357b

Usage :

```bash
yarn lint
yarn lint --fix
```


### Start frontend

```bash
cd frontend
yarn install
yarn run dev
```

## Backend init

```bash
cd backend
cargo run
```
