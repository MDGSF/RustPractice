# diesel

```sh
# 安装 diesel 命令行工具
sudo apt install libsqlite3-dev
sudo apt install libmysqlclient-dev
sudo apt install libpq-dev
cargo install diesel_cli
```


```sh
# 初始化项目的数据库配置

# mysql
echo DATABASE_URL=mysql://username:password@localhost/diesel_demo > .env

# postgres
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env

# sqlite3
echo DATABASE_URL=diesel_demo.sql > .env

diesel setup
```


```sh
diesel migration generate create_posts

# Each migration can be applied (up.sql) or reverted (down.sql).
# Applying and immediately reverting a migration should leave your database schema unchanged.
```

