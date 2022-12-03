default: build

check:
	cargo check

build:
	cargo build

build/optimized:
	chmod +x scripts/app/build.sh && ./scripts/app/build.sh

lint:
	cargo fmt --all --check && cargo clippy --no-deps -- -D warnings

fmt:
	cargo fmt --all

audit:
	cargo audit

test:
	cargo test

coverage:
	cargo tarpaulin --ignore-tests --avoid-cfg-tarpaulin

db/up:
	chmod +x scripts/db/init_db.sh && ./scripts/db/init_db.sh

db/rm:
	chmod +x scripts/db/drop_db.sh && ./scripts/db/drop_db.sh