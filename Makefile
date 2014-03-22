# Rust crates
RUSTCRATES            = skirt mddebug
skirt_BUILD_DEPS      = $(SUNDOWN_NAME)
mddebug_CRATE_DEPS    = skirt

DEBUG                 ?= 1

RUSTAUTORULES         = 0
RUSTDEBUG             = $(DEBUG)
include               rust-mk/rust.mk

# C/C++
## Paths
CC                    ?= gcc
CXX                   ?= g++

## Flags
CFLAGS                =
CXXFLAGS              =

ifeq ($(DEBUG),0)
CFLAGS                += -O3
else
CFLAGS                += -ggdb3
endif

# Sundown
SUNDOWN_DIR           = deps/sundown
SUNDOWN_NAME          = $(RUSTLIBDIR)/libsundown.a
SUNDOWN_SOURCES       = \
	$(wildcard $(SUNDOWN_DIR)/src/*.c) \
	$(wildcard $(SUNDOWN_DIR)/html/*.c)
SUNDOWN_OBJECTS       = $(SUNDOWN_SOURCES:.c=.o)
SUNDOWN_CFLAGS        = -fPIC -I $(SUNDOWN_DIR)/src -I $(SUNDOWN_DIR)/html

$(SUNDOWN_NAME):      $(SUNDOWN_OBJECTS)
	ar rc $(SUNDOWN_NAME) $(SUNDOWN_OBJECTS)
	ranlib $(SUNDOWN_NAME)

$(SUNDOWN_DIR)/%.o:   $(SUNDOWN_DIR)/%.c
	$(CC) $(CFLAGS) $(SUNDOWN_CFLAGS) -c -o $@ $<

clean_sundown:
	rm -f $(SUNDOWN_OBJECTS)
.PHONY clean:         clean_sundown

fclean_sundown:
	rm -f $(SUNDOWN_NAME)
.PHONY fclean:        fclean_sundown

# Rust rules
$(eval $(call RUST_CRATE_RULES))
