
run:
	cargo run --release

doc:
	cargo doc --document-private-items --open

build-doc: 
	cargo doc --document-private-items

clean:
	cargo clean

.PHONY: run, doc, build-doc, clean