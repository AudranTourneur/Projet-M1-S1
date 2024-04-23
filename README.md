# Projet-M1-S1 - OkiDoki

## Description

Designed in Rust and deployable via Docker, the application aims to simplify the lives of IT users. Compatible with all Linux distributions, it offers a simple, efficient experience for a range of IT tasks, making it easy to manage your server.

Inside, you'll find management of your images, containers and volumes, as well as networks. The application comes with a topoligy so you can easily see your Dockers components.
![alt text](https://github.com/AudranTourneur/Projet-M1-S1/blob/main/front/static/logo.png?raw=true)
front/static/logo.png

### Responsible teacher

Pierre CAPIOD

### Students

- Audran TOURNEUR
- Judith LECOQ
- Amaury GIOT
- Simon BERNARD de LAJARTRE
- Xavier LEDUC
- Antoine FOURNET

### Aim

OkiDoki aims to provide a graphical interface to manage the Docker containers of a server. It is a web application that
allows the user to create, start, stop, delete and monitor the images, containers, volumes and networks.

### Technologies

The project is containerized itself in three Docker containers:

- `okidocky-db`: a clickhouse database to store and persist the statistics
- `okidocky-core`: a rust application that communicates with the docker daemon, the database and the frontend
- `okidocky-ui`: a svelte-kit web application that provides the user interface

## Setup

### Clone the project

```bash
git clone git@github.com:AudranTourneur/Projet-M1-S1.git
```

### Requirements

- You must have installed Docker on your machine to run the project.
Use the [official installation guide for your OS to install Docker](https://docs.docker.com/engine/install/).
- Create a `.env` file in the root of the project and set the variables. You can copy the `.env.example` file and for a quick start you can use the default values.

### Run the project

At the root of the project, run the following command:

```bash
docker compose -f docker-compose.prod.yml up -d
```

A script for starting the project is also available:

```bash
sh start-prod.sh
```

The project is now running and you can access the web application at [`http://localhost:7777`](http://localhost:7777).
