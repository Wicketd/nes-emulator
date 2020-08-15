.PHONY: test clean

TEST_ROM_DIR := tests/rom
TEST_ROM_BIN := $(TEST_ROM_DIR)/main.bin

test: $(TEST_ROM_BIN)
	cargo test

clean:
	rm -f $(TEST_ROM_DIR)/*.{o,bin}

%.bin: %.o
	ld65 --config config/nrom_256.cfg $< -o $@

%.o: %.s
	ca65 $<
