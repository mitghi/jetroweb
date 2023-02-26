trunk:
	trunk serve

build:
	cargo build --release

run:
	cargo run

dockerize:
	docker build -t jetrepl:latest .

run_cnt:
	docker run -it --env JETRO_HOST="0.0.0.0" -p 8080:8080 jetrepl:latest
