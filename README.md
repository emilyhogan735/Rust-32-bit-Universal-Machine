# Rust-32-bit-Universal-Machine
32-bit Virtual Universal Machine written in Rust, compatible with 64-bit hardware

My program reads in a program using the load module and allocates a new memory storage using the memory module. The program then initializes
an array to hold the registers and opcode. It loops through the instructions. As it goes throught the loop, the registers A, B, C and the
opcode are extracted using my bitpack module from the last assignment. Then, a match matches the opcode to the instruction. It goes through
and matches all instructions to thea 14 possible options. Some options are simple and are done in the main. Other instructions that utilize
memory in any way are done in the memory module. A program counter is used to keep track of what instruction the program is on.
The modules used are the main, which takes care of figuring out which instruction to run, load, which loads initial the program file, 
memory, which takes care of any instruction that utilizes memory in any way, and bitpack, which extracts the register and opcode from 
each instruction.
To test how long it takes for my UM to execute 50 million instructions, I added an instruction counter at the beginning of my loop in main, 
then output the value using a print statement just before the halt instruction. I then timed it in the terminal by running the command
time cargo run -r -- sandmark.um. The time came out to 1m27.776s and there were 2113497561 instructions run. Doing some math, I have
determined that my UM can run 50 million instructions in approximately 2.0766 seconds.
