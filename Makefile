# Usage: make run EULER=1

EULER ?= 1
PROJECT := $(EULER)
BIN := euler$(EULER)

.PHONY: run build docker-build docker-run clean

run: docker-build docker-run

# Build the Docker image for the selected Euler problem
docker-build:
	docker build --no-cache --build-arg PROJECT=$(PROJECT) -t euler-runner-$(PROJECT) .

# Run in container (let Dockerfile CMD handle printing README and running binary)
docker-run:
	docker run --rm euler-runner-$(PROJECT)

clean:
	docker rmi euler-runner-$(PROJECT) || true
