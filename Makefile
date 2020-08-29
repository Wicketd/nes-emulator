.PHONY: test build-rom clean

TEST_ROM_DIR := tests/rom
TEST_ROM_BIN := $(TEST_ROM_DIR)/main.bin

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
