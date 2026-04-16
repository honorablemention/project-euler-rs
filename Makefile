# Usage: make run EULER=1

EULER ?= 1
BENCH ?=
PROJECT := $(EULER)
BIN := euler$(EULER)
BENCH_ARG = $(if $(BENCH),--bench $(BENCH),)

.PHONY: run bench build docker-build docker-run docker-bench-build docker-bench-run clean

run: docker-build docker-run

bench: docker-bench-build docker-bench-run

# Build the Docker image for the selected Euler problem
docker-build:
	DOCKER_BUILDKIT=1 docker build --build-arg PROJECT=$(PROJECT) -t euler-runner-$(PROJECT) .

# Run in container, forwarding ARGS to the binary
docker-run:
	@status=0; \
	docker run --rm euler-runner-$(PROJECT) sh -c 'cat /app/README.md && echo && /app/euler${PROJECT} $(ARGS)' || status=$$?; \
	docker rmi euler-runner-$(PROJECT) >/dev/null 2>&1 || true; \
	exit $$status

# Build the Docker image through the Rust builder stage for benchmarks
docker-bench-build:
	DOCKER_BUILDKIT=1 docker build --target builder --build-arg PROJECT=$(PROJECT) -t euler-bench-$(PROJECT) .

# Run Criterion benchmarks in the builder container, forwarding BENCH and ARGS
docker-bench-run:
	@status=0; \
	docker run --rm -w /build/$(PROJECT) euler-bench-$(PROJECT) cargo bench $(BENCH_ARG) $(ARGS) || status=$$?; \
	docker rmi euler-bench-$(PROJECT) >/dev/null 2>&1 || true; \
	exit $$status

clean:
	docker rmi euler-runner-$(PROJECT) || true
	docker rmi euler-bench-$(PROJECT) || true
