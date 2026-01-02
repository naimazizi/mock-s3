build:
	docker build -t mock-s3-rs:latest .

run:
	docker run -p 8080:8080 \
	--rm \
	--name mock-s3-rs \
	--network=host \
	-v .:/app \
	-e RUST_LOG=${RUST_LOG} \
	mock-s3-rs:latest
