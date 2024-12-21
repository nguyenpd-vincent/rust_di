### Rust actix-web with Mysql.

### Setup Mysql Docker
```bash
docker run -d \
  --name db \
  -e MYSQL_DATABASE=database_name \
  -e MYSQL_ROOT_USER=root \
  -e MYSQL_ROOT_PASSWORD=mypassword \
  -p 3306:3306 \
  -v db-mysql:/var/lib/mysql \
  -v $(pwd)/db-mysql/init.sql:/docker-entrypoint-initdb.d/init.sql \
  mysql:8.0
  ```
Create tables in `init.sql` file to make sure that tables are created when the container is started.



### Setup Environment
```bash
cp .env.example .env
```
Then change environment variables in `.env` file.

### Run
```bash
cargo run
```

### Test with curl
Find user by email
```bash
curl -X POST http://localhost:8080/auth/login -d '{"email":"nguyen@example.com"}' -H 'Content-Type: application/json'
```

Find posts by user id
```bash
curl -X GET http://localhost:8080/posts/1 -H 'Content-Type: application/json'
```