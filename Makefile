.PHONY: build_fc2 programmer_fc2 analysis execute_test execute_tests

build_fc2:
	cd ./veryls/FC2 && \
	veryl fmt && \
	veryl build; \
	quartus_map --read_settings_files=on --write_settings_files=off fc2 -c fc2 && \
	quartus_fit --read_settings_files=off --write_settings_files=off fc2 -c fc2 && \
	quartus_asm --read_settings_files=off --write_settings_files=off fc2 -c fc2 && \
	quartus_sta fc2 -c fc2

programmer_fc2:
	cd veryls/FC2/output_files && \
	quartus_pgm -m jtag -o "p;fc2.sof"

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
