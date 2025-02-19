run-local-gpu:
	LOCAL=1 RUSTFLAGS="-C target-cpu=native" cargo run -F cuda -r

run-local-fake-gpu:
	LOCAL=1 RISC_DEV_MODE=1 RUSTFLAGS="-C target-cpu=native" cargo run -F cuda -r

run-local-cpu:
	LOCAL=1 cargo run

run-local-fake-cpu:
	LOCAL=1 RISC0_DEV_MODE=1 cargo run

run-gpu:
	RUSTFLAGS="-C target-cpu=native" cargo run -F cuda -r