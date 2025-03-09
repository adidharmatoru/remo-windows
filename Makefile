# Define the target names and paths
TARGET_WINDOWS = x86_64-pc-windows-gnu

# Define the Dockerfile names
DOCKERFILE_WINDOWS = ./docker/Dockerfile.windows

# Binary name (change this to match your actual app name)
BINARY_NAME = remo

# Docker image name for building
DOCKER_IMAGE = remo-build
CONTAINER_NAME = remo-temp-container

# Build for Windows
windows:
	@echo "Building for Windows..."
	docker build -f $(DOCKERFILE_WINDOWS) -t $(DOCKER_IMAGE) .
	docker create --name $(CONTAINER_NAME) $(DOCKER_IMAGE)
	@if docker cp $(CONTAINER_NAME):/app/target/$(TARGET_WINDOWS)/release/$(BINARY_NAME).exe ./release/$(BINARY_NAME)-windows.exe; then \
		echo "Build for Windows complete: $(BINARY_NAME)-windows.exe"; \
	else \
		echo "File not found for Windows, skipping copy."; \
	fi
	docker rm $(CONTAINER_NAME)
	@$(MAKE) clean

# Clean up Docker images and containers
clean:
	@echo "Cleaning up Docker images and containers..."
	docker rmi $(DOCKER_IMAGE)
	@echo "Clean complete."

# Clean up Docker images and containers
format:
	cargo fmt --all
	cargo sort . --grouped
	@echo "Format complete."