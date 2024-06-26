.PHONY: build programmer analysis execute_test execute_tests

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

execute_test-: $(addprefix execute_test-, $(LINE))

execute_test-%:
	cd packages/stm32l476rg && \
	cargo run --bin ${@:execute_test-%=%}

execute_tests:
	cd packages/test-executor && \
	cargo run

execute_sequence-: $(addprefix execute_sequence-, $(LINE))

execute_sequence-%:
	cd packages/dump_sequence && \
	cargo run ${@:execute_sequence-%=%} && \
	cd ../stm32l476rg && \
	cargo run --bin generated_sequence_test
