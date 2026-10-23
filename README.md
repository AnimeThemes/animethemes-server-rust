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

# Open the .env file and set the APP_KEY
nano .env

# Build the shared services used by php and rust servers
docker compose -p animethemes-shared -f docker-compose.shared.yml pull
docker compose -p animethemes-shared -f docker-compose.shared.yml up -d

# Build the main Docker image and start the containers
docker build -t animethemes-server-rust:local .
docker compose up -d

# Import dumps if you have one
docker exec -i animethemes-shared-mysql mysql -u root animethemes < content.sql

# Run the migrations
docker compose run --rm server-rs db migrate

# Seed the database
docker compose run --rm server-rs db seed
```

### Running

* GraphQL: http://animethemes-rust.test/graphql
* GraphiQL: http://animethemes-rust.test

## Extra Configuration

### Users

```sh
# Create a new user
docker compose run --rm server-rs task create:user name:"Name" email:"example@example.com" password:"Password1."

# Assign a role to the user
docker compose run --rm server-rs task assign:role id:1 role:"Admin"
```

### Search

Import models into our indices using:

```sh
docker compose run --rm server-rs task search:index-anime
docker compose run --rm server-rs task search:index-artist
docker compose run --rm server-rs task search:index-entry
docker compose run --rm server-rs task search:index-playlist
docker compose run --rm server-rs task search:index-series
docker compose run --rm server-rs task search:index-song
docker compose run --rm server-rs task search:index-studio
docker compose run --rm server-rs task search:index-theme
docker compose run --rm server-rs task search:index-video
```

### MySQL Terminal

To open the MySQL Terminal:

```bash
docker exec -it animethemes-shared-mysql mysql -u root animethemes
```

# Resources

Please make use of the #api channel in the [**Discord Server**](https://discordapp.com/invite/m9zbVyQ) for questions pertaining to the AnimeThemes database or API.