#!/bin/sh

cd veryls/6502
quartus_map --read_settings_files=on --write_settings_files=off 6502 -c 6502
quartus_fit --read_settings_files=off --write_settings_files=off 6502 -c 6502
quartus_asm --read_settings_files=off --write_settings_files=off 6502 -c 6502
quartus_sta 6502 -c 6502
quartus_eda --read_settings_files=off --write_settings_files=off 6502 -c 6502
