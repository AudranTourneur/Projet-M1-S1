use bollard::Docker;

pub fn get_docker_socket() -> Docker {
    let docker: Docker = Docker::connect_with_local_defaults().expect("Failed to connect to the Docker socket");
    // let docker = Docker::connect_with_unix("/rootfs/var/run/docker.sock", 120, API_DEFAULT_VERSION).unwrap();

    docker
}