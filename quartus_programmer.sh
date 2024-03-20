#!/bin/sh

cd veryls/6502
quartus_pgm -m jtag -o "p;output_files/6502.sof"
