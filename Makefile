.PHONY: release

release:
	@echo "Building release..."
	@cargo build --release
	@echo "Copying assets..."
	@cp -r assets target/release
