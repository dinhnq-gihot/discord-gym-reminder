up-db:
	cd ./docker && docker compose up db -d

down-db:
	cd ./docker && docker compose down && sudo rm -rf lib

run-bot:
	RUST_BACKTRACE=full RUST_LOG=info,hyper=info,discord_bot=trace${RUST_LOG} cargo run --release