test-loop:
	while RUST_LOG=$(RUST_LOG) RUST_BACKTRACE=$(RUST_BACKTRACE) cargo test --features mock_time $(TEST_TO_RUN) -- --nocapture --test-threads=1;do :;done
test:
	RUST_LOG=$(RUST_LOG) RUST_BACKTRACE=$(RUST_BACKTRACE) cargo test --features mock_time $(TEST_TO_RUN) -- --nocapture --test-threads=1
run-cluster:
	cargo run --bin single_value_store_cluster
client-get:
	cargo run --bin single_value_store_client -- --server-address 127.0.0.1:500$(SERVER) get
client-set:
	cargo run --bin single_value_store_client -- --server-address 127.0.0.1:500$(SERVER) set $(VALUE)