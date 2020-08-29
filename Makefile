.PHONY: test build-rom clean

TEST_ROM_DIR := rom
TEST_ROM_SRC := $(wildcard $(TEST_ROM_DIR)/*.s)
TEST_ROM_BIN := $(TEST_ROM_SRC:.s=.bin)

test: build-rom
	cargo test

build-rom: $(TEST_ROM_BIN)

clean:
	rm -f $(TEST_ROM_DIR)/*.{o,bin}
	cargo clean

%.bin: %.o
	ld65 --config config/nrom_256.cfg $< -o $@

%.o: %.s
	ca65 $<
