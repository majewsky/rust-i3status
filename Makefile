PREFIX := /usr

debug: FORCE
	cargo build

run: debug FORCE
	target/debug/custom-i3status

release: FORCE
	cargo build --release

install: release FORCE
	install -D -m 0755 target/release/custom-i3status "$(DESTDIR)$(PREFIX)/bin/custom-i3status"

.PHONY: FORCE
