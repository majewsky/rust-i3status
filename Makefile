PREFIX := /usr

debug:
	cargo build

release:
	cargo build --release

install: release
	install -D -m 0755 target/release/custom-i3status "$(DESTDIR)$(PREFIX)/bin/custom-i3status"
