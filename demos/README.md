
# 6502 Demo programs
## blink.asm
blink.asm is Ben Eater's LED blinking program from his [6502 video series](https://eater.net/6502), but has been modified to start at address $C000 instead of $8000 for use with this emulator.

  

## Building
To build the blink demo, you can use the following command from the root of the project:

    ./demos/utils/vasm6592_oldstyle.exe -Fbin -dotdir ./demos/blink.asm -o ./demos/blink.bin

## Running
To run the blink demo, you can use the following command from the root of the project:

    go run ./emulator -r ./demos/blink.bin
