
all:
	cargo build

pack:

	
clean:
	cargo clean

test:
	cargo test -p mathlib

doc:
	cargo doc --all --no-deps --open

run:
	cargo tauri dev

profile:
	cargo flamegraph -p stddev -o ../profiling/flamegraph.svg
	mv perf.data ../profiling/
