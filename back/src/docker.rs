use bollard::{Docker, API_DEFAULT_VERSION};

pub fn get_docker_socket() -> Docker {
    let docker: Docker = Docker::connect_with_local_defaults().unwrap();
    // let docker = Docker::connect_with_unix("/rootfs/var/run/docker.sock", 120, API_DEFAULT_VERSION);

    docker.unwrap()
}