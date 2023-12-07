# Documentation API

## POST /login
### Request
{
    username: string,
    password: string
}
### Response
{
    success: true,
    data: {
        token: string,
        expiresAt: number,         // timestamp ms
    }
} | {
    success: false,
}

Toutes les routes suivantes nécessitent d'être authentifié et d'avoir le token de session 
en tant que cookie nommé "token" envoyé dans la requête

## GET /overview
Response:
{
    versionDocker: string,
    versionDockerApi: string,
    versionLinux: string,
    images: number,
    containers: number,
    volumes: number,
    networks: number,
}

## GET /images
{
    images: Array<{
        id: string,
        size: number,
        created: number,    // timestamp milliseconds
        iconUrl: string | null,
        tags: Array<string>,
    }>
}

## GET /images/:id
{ 
    id: string,
    size: number, // bytes
    created: number, // timestamp ms
    iconUrl: string | null,
    tags: Array<string>,
}


## GET /volumes

## GET /volumes/:id

## GET /volumes/:id/download

## GET /containers

## GET /containers/:id

## POST /containers/start

## POST /containers/stop

## GET /networks

## GET /networks/:id

## GET /topology
{
    containers: Array<
        name: string,
        image: string,
        iconUrl: string | null,
        exposedPorts: number[],
        exposedVolumes: string[],
        connectedTo: Array<string>      // Array<EntityID> 
    >,
    ports: [
        interface: string,              // 127.0.0.1, etc
        number: number                  // port number, 80, 443, etc
        connectedTo: Array<string>      // Array<EntityID> 
    ],
    volumes: Array<
        name: string,
        size: number,                   // bytes
        connectedTo: Array<string>      // Array<EntityID> 
    >
}