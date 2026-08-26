[**AnimeThemes**](https://animethemes.moe/) is a simple and consistent repository of anime opening and ending themes. We provide direct links to high quality WebMs of your favorite OPs and EDs for your listening and discussion needs.

This is the repository for the server application that is responsible for AnimeThemes.moe resource management, API, and other services related to serving the AnimeThemes database.

# Installation

- [Prerequisites](#prerequisites)
- [Setup](#setup)
  - [Running](#running)
  - [Extra Configuration](#extra-configuration)
  - [Users](#users)
  - [Search](#search)
- [Resources](#resources)

## Prerequisites

* [Docker](https://www.docker.com/)

Docker will setup Rust, MySQL, Typesense and Redis for you. If you are on Windows, use the [WSL](https://learn.microsoft.com/windows/wsl/install) terminal.

## Setup

```bash
# Clone the repository
git clone git@github.com:AnimeThemes/animethemes-server-rust.git
cd animethemes-server-rust

# Copy the .env.example to .env and change it for your needs.
cp .env.example .env
```

Open the `/etc/hosts` file and paste the contents there:

```
127.0.0.1 animethemes-rust.test
```

```bash
# Generate a new APP_KEY
openssl rand -hex 32

# Set the APP_KEY in the .env file

# Build the containers
docker compose up -d --build

# Import dumps if you have one
docker exec -i animethemes-server-rust-mysql mysql -u root animethemes < content.sql

# Run the migrations
docker compose exec -w /app/migration rust cargo run -- up
```

### Running

* GraphQL: http://animethemes-rust.test/graphql
* GraphiQL: http://animethemes-rust.test

## Extra Configuration

### Users

TODO: Waiting feature implementation

### Search

Import models into our indices using:

```sh
docker compose exec server animethemes-server-rust task search:index_anime
docker compose exec server animethemes-server-rust task search:index_animetheme
docker compose exec server animethemes-server-rust task search:index_animethemeentry
docker compose exec server animethemes-server-rust task search:index_artist
docker compose exec server animethemes-server-rust task search:index_playlist
docker compose exec server animethemes-server-rust task search:index_series
docker compose exec server animethemes-server-rust task search:index_song
docker compose exec server animethemes-server-rust task search:index_studio
docker compose exec server animethemes-server-rust task search:index_video
```

### MySQL Terminal

To open the MySQL Terminal:

```bash
docker exec -it animethemes-server-rust-mysql mysql -u root animethemes
```

# Resources

Please make use of the #api channel in the [**Discord Server**](https://discordapp.com/invite/m9zbVyQ) for questions pertaining to the AnimeThemes database or API.