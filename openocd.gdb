# set gdb target to server running on port 3333
target extended-remote :3333

# print human readable, unmangled symbols
set print asm-demangle on

set print pretty on

# prevents infinite backtrace - is this necessary ?
set backtrace limit 32

break DefaultHandler
break HardFault
break rust_begin_unwind
break main

# commands to enable ITM
monitor tpiu config internal itm.txt uart off 8000000
monitor itm port 0 on

# flash the program
load

stepi
