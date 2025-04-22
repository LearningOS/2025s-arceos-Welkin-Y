DOCKER_NAME ?= arceos
.PHONY: docker build_docker
	
docker:
	docker run --rm -it --privileged -v ${PWD}:/mnt -w /mnt ${DOCKER_NAME} bash

build_docker: 
	docker build -t ${DOCKER_NAME} .

