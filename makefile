build:
	cargo build --release
	-mkdir dist
	-rm dist/wallpapers.tar.gz
	tar -czvf dist/wallpapers.tar.gz -C target/release wallpapers
