RUSTC               = rustc
RUSTDOC             = rustdoc

SRC_DIR             = src
LIB_FILE            = $(SRC_DIR)/noise/lib.rs
EXAMPLE_FILES       = $(SRC_DIR)/examples/*.rs

CRATE_NAME          = $(shell $(RUSTC) --crate-name $(LIB_FILE))
CRATE_FILES         = $(shell $(RUSTC) --crate-file-name $(LIB_FILE))

DEPS_DIR            = deps
DOC_DIR             = doc
EXAMPLES_DIR        = examples
LIB_DIR             = lib

all: lib

lib:
	mkdir -p $(LIB_DIR)
	$(RUSTC) $(LIB_FILE) --out-dir=$(LIB_DIR) --opt-level 3

test:
	mkdir -p $(LIB_DIR)
	rustc --test $(LIB_FILE) --opt-level 3 -o test~ && ./test~
	rm test~
	rustdoc --test -L $(LIB_DIR) $(LIB_FILE)

bench:
	rustc --test $(LIB_FILE) --opt-level 3 -o bench~ && ./bench~ --bench
	rm bench~

doc:
	mkdir -p $(DOC_DIR)
	$(RUSTDOC) $(LIB_FILE) -o $(DOC_DIR)

examples-dir:
	mkdir -p $(EXAMPLES_DIR)

$(EXAMPLE_FILES): examples-dir
	$(RUSTC) -L $(LIB_DIR) --out-dir=$(EXAMPLES_DIR) --opt-level 3 $@

examples: $(EXAMPLE_FILES)

clean:
	rm -rf $(LIB_DIR)
	rm -rf $(EXAMPLES_DIR)
	rm -rf $(DOC_DIR)

.PHONY: \
	all \
	lib \
	test \
	bench \
	doc \
	examples \
	examples-dir \
	$(EXAMPLE_FILES) \
	clean
