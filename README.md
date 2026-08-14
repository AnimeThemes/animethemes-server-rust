# WIP

### Compile

```bash
docker compose up -d --build
```

### Run bin

```bash
docker compose run --rm server {bin_name}
```

docker exec -i animethemes-server-rust-mysql mysql -u root animethemes < content.sql