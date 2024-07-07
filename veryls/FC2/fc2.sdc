create_clock -name clk -period 20.000 [get_ports {clk}]
derive_clock_uncertainty
set_input_delay  -clock {clk} 1 [all_inputs]
set_output_delay -clock {clk} 1 [all_outputs]
