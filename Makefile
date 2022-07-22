

.PHONY: patch svd2rust form clean-rs clean-patch clean-check clean-html clean rustfmt check
.PRECIOUS: svd/%.svd .deps/%.d

SHELL := /usr/bin/env bash
DEVICES ?= imxrt1011 imxrt1015 imxrt1021 imxrt1051 imxrt1052 imxrt1061 imxrt1062 imxrt1064 \
	imxrt1176_cm7 imxrt1176_cm4

all: patch crate rustfmt check

# Things to do in CI
ci: patch crate rustfmt

# All yaml files in devices/ will be used to patch an SVD
DEVICE_YAMLS := $(foreach device, $(DEVICES), \
	       $(wildcard devices/$(device)*.yaml))

# Each yaml file in devices/ exactly name-matches an SVD file in svd/
DEVICE_PATCHED_SVDS := $(patsubst devices/%.yaml, svd/%.svd.patched, $(DEVICE_YAMLS))

DEVICE_FORMATTED_SVDS := $(patsubst devices/%.yaml, svd/%.svd.formatted, $(DEVICE_YAMLS))

# Turn a devices/device.yaml and svd/device.svd into svd/device.svd.patched
svd/%.svd.patched: devices/%.yaml svd/%.svd .deps/%.d
	svd patch $<

svd/%.svd.formatted: svd/%.svd.patched
	xmllint $< --format -o $@

patch: $(DEVICE_PATCHED_SVDS)

format: $(DEVICE_FORMATTED_SVDS)

# Check vairous feature set combinations here for each SoC in the family
define check_template
.check/$(1).ok:
	@mkdir -p .check
	cargo check --features=$(1) && touch $$@

endef

$(foreach device,$(DEVICES),$(eval $(call check_template,$(device))))

check: $(patsubst %, .check/%.ok, $(DEVICES))

clean-check:
	rm -rf .check

html/index.html: $(PATCHED_SVDS)
	@mkdir -p html
	python3 scripts/makehtml.py html/ svd/stm32*.svd.patched

html: html/index.html

clean-patch:
	rm -f $(DEVICE_PATCHED_SVDS)
	rm -f $(DEVICE_FORMATTED_SVDS)

clean-html:
	rm -rf html

clean: clean-patch clean-html clean-check
	rm -rf .deps

# As alternative to `pip install --user svdtools`:
# run `make venv update-venv` and `source venv/bin/activate'
venv:
	python3 -m venv venv

update-venv:
	venv/bin/pip install -U pip
	venv/bin/pip install -U -r requirements.txt

# Generate dependencies for each device YAML
.deps/%.d: devices/%.yaml
	@mkdir -p .deps
	python3 scripts/makedeps.py $< > $@

crate: patch
	raltool generate svd/imxrt10*.svd.patched --transform raltool-cfg.yaml

rustfmt:
	cargo fmt
	
-include .deps/*
