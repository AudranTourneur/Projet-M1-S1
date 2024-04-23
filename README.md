# Projet-M1-S1 - OkiDocky

<img src="https://github.com/AudranTourneur/Projet-M1-S1/blob/main/front/static/logo.png" width="200" height="200" />

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

- `okidocky-core`: a Rust application which communicates with the Docker daemon, the database and the frontend
- `okidocky-ui`: a SvelteKit web application which provides the user interface
- `okidocky-db`: a Clickhouse database to store and persist statistics

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

### Post-mortem & Recommendations for project continuation 

Sadly, this project hasn't reached full completion and some features initially planned haven't been implemented.  

As such this project will very likely be continued in the near future to finish what has been started. 

We will now reflect upon the choices we have made in regard of our newly acquired experience and give recommendations for the next group: 

#### Front-end 

The front-end uses SvelteKit with TypeScript and we have been very happy with this framework, it has been a highly productive experience and it's relatively easy to expand upon it.  

Similarly, the graphical interface uses the Pixi.js library and it allowed for rapid iterations and development. 

We recommend going forward with this choice. 

  

#### Back-end   

The choice of the Rust programming language for the backend was meant as a learning experience, while we learnt a lot and we are now way more confident in our understanding of Rust, the productivity hit it imposed on the project has been widely underestimated. 

Among other things, the following problems have been observed: 

- Less experienced programmers of the group were almost completely unable to contribute to backend code 

- More advanced programmers were able to contribute but at a significantly slower pace than in other languages 

- Slow compile times (a full rebuild of the backend can take up to a few minutes) 

As such, we hereby recommend for the next developers of the project to rewrite the backend in either Go or TypeScript (probably TypeScript to increase accessibility to all developers of the group). 

Since at least one member of the old group will be in common with the next group and the logic has already been written, it should truly take less than a week for a few experienced developers to reach feature parity with the current Rust codebase (which is about 3000 lines of code). 

Moreover, if the choice of TypeScript is made, it will allow for easy sharing of type definitions between the frontend and backend and favors code reuse, reducing friction. 

  

#### Statistics 

For the statistics subsystem, the Clickhouse database has been used. This choice seems to have proven fine and does not need to be reconsidered for now. 

However, to maximize portability in a real-world situation, it would probably be wise to add support for Prometheus and allow for data federation into an existing central system, instead of creating a custom ad-hoc solution.  

A real-world user might probably want to visualize their statistics using Grafana in a central location instead of relying on a custom interface since Grafana is already a highly advanced piece of software for graph visualization. 

#### Domain Name System / Reverse proxy support 

An idea we initially had was to add support for the NGINX reverse proxy to quickly be able to remap ports and domain names, however NGINX is probably a bad candidate for doing this (at least the free version of NGINX): 

- Hot configuration swapping is not well supported by the NGINX engine 

- NGINX does not expose an API to retrieve easily and edit the current configuration, requiring us to parse the configuration files using a non-trivial syntax 

- SSL/TLS certificate management is not handled automatically by NGINX and we will need to manually call Let's Encrypt Certbot mechanism to obtain and manage certificates, which could be uncertain and error-prone. 
