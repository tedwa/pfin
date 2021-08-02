.POSIX:
.PHONY: clean install uninstall

PREFIX = $(HOME)/.local/bin
ARTIFACT_PREFIX = target/release

pfin:
	cargo build --release

clean:
	cargo clean

install: pfin
	mkdir -p $(PREFIX)
	cp -f $(ARTIFACT_PREFIX)/pfin $(PREFIX)/

uninstall:
	rm -f $(PREFIX)/pfin
