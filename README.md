# subscription-manager

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
# Enter password

sudo -u postgres createdb subscriptions_db -O subscriptions_db_user
```


### Write the created user password in a pass.pgpass file

```bash
export PGPASSFILE=./backend/database/pass.pgpass
echo "*:*:*:subscriptions_db_user:<password>" > $PGPASSFILE
chmod 0600 $PGPASSFILE  # Restrict access write to the file
```


### Run the initialization scripts with the created user

```bash
psql -d subscriptions_db -U subscriptions_db_user -a -f ./backend/database/init_table.sql
psql -d subscriptions_db -U subscriptions_db_user -a -f ./backend/database/fill_db.sql
```
<br>


## Frontend init

### Configure .env variables

Create a file named .env.dev and write VITE_BASEURL = "http://localhost:8000/".


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