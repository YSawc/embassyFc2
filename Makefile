.PHONY: build programmer analysis execute_tests

build:
	veryl fmt
	veryl build; \
	cd veryls/6502 && \
	quartus_map --read_settings_files=on --write_settings_files=off 6502 -c 6502 && \
	quartus_fit --read_settings_files=off --write_settings_files=off 6502 -c 6502 && \
	quartus_asm --read_settings_files=off --write_settings_files=off 6502 -c 6502 && \
	quartus_sta 6502 -c 6502
	# quartus_sta 6502 -c 6502 && \
	# quartus_eda --read_settings_files=off --write_settings_files=off 6502 -c 6502

programmer:
	cd veryls/6502 && \
	quartus_pgm -m jtag -o "p;6502.sof"

analysis:
	veryl fmt
	veryl build; \
	cd veryls/6502 && \
	quartus_map --read_settings_files=on --write_settings_files=off 6502 -c 6502 --analysis_and_elaboration

execute_tests:
	cd packages/test-executor && \
	cargo run
