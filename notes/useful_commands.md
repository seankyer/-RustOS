# Useful Commands

When debugging linker scripts and making sure they're doing what they're supposed to be doing,
I've found the following commands extremely useful.

## Get VirtAddr and PhysAddr of an elf file

```bash
readelf -l <binary.elf>
```
Result:
```bash
Elf file type is EXEC (Executable file)
Entry point 0x41
There are 6 program headers, starting at offset 52

Program Headers:
  Type           Offset   VirtAddr   PhysAddr   FileSiz MemSiz  Flg Align
  LOAD           0x010000 0x00000000 0x00000000 0x00040 0x00040 R   0x10000
  LOAD           0x010040 0x00000040 0x00000040 0x0013c 0x0013c R E 0x10000
  LOAD           0x020000 0x20000000 0x0000017c 0x00004 0x02008 RW  0x10000
  LOAD           0x022008 0x20002008 0x00002184 0x00010 0x00010 R   0x10000
  GNU_STACK      0x000000 0x00000000 0x00000000 0x00000 0x00000 RW  0
  EXIDX          0x022008 0x20002008 0x00002184 0x00010 0x00010 R   0x4

 Section to Segment mapping:
  Segment Sections...
   00     .vector_table
   01     .text
   02     .data .bss .stack .heap
   03     .ARM.exidx
   04
   05     .ARM.exidx
```
Virtual address tells your where the machine code is going to look for that variable while PhysAddr tells you where it is literally. The bootloader or startup code needs to copy these sections to where the device expects to find them.

What I would then do as part of my workflow is check with GDB if the section was copied properly or not. `x/x 0x20000000` checking if .data got copied and `x/x 0x00000040` checking what the value of .data is.

## Printout the sections of your elf

```bash
readelf -SW <binary.elf>
```
Result:
```bash
There are 21 section headers, starting at offset 0x24850:

Section Headers:
  [Nr] Name              Type            Addr     Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            00000000 000000 000000 00      0   0  0
  [ 1] .vector_table     PROGBITS        00000000 010000 000040 00   A  0   0  4
  [ 2] .text             PROGBITS        00000040 010040 00013c 00  AX  0   0  2
  [ 3] .data             PROGBITS        20000000 020000 000004 00  WA  0   0  4
  [ 4] .bss              NOBITS          20000004 020004 000004 00  WA  0   0  4
  [ 5] .stack            NOBITS          20000008 020004 001000 00  WA  0   0  1
  [ 6] .heap             NOBITS          20001008 020004 001000 00  WA  0   0  1
  [ 7] .ARM.exidx        ARM_EXIDX       20002008 022008 000010 00  AL  2   0  4
  [ 8] .debug_loc        PROGBITS        00000000 022018 0004b5 00      0   0  1
  [ 9] .debug_abbrev     PROGBITS        00000000 0224cd 0002bc 00      0   0  1
  [10] .debug_info       PROGBITS        00000000 022789 000a54 00      0   0  1
  [11] .debug_aranges    PROGBITS        00000000 0231dd 000078 00      0   0  1
  [12] .debug_ranges     PROGBITS        00000000 023255 0000a8 00      0   0  1
  [13] .debug_str        PROGBITS        00000000 0232fd 000ae7 01  MS  0   0  1
  [14] .comment          PROGBITS        00000000 023de4 000093 01  MS  0   0  1
  [15] .ARM.attributes   ARM_ATTRIBUTES  00000000 023e77 00003a 00      0   0  1
  [16] .debug_frame      PROGBITS        00000000 023eb4 0000d0 00      0   0  4
  [17] .debug_line       PROGBITS        00000000 023f84 00040c 00      0   0  1
  [18] .symtab           SYMTAB          00000000 024390 000220 10     20  10  4
  [19] .shstrtab         STRTAB          00000000 0245b0 0000d1 00      0   0  1
  [20] .strtab           STRTAB          00000000 024681 0001cc 00      0   0  1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  D (mbind), y (purecode), p (processor specific)
```
Found this useful for when I wasn't sure if a section was being populated properly or not.

## Disassemble the .text section

```bash
arm-none-eabi-objdump -d <binary.elf>
```
Results:
```asm
target/thumbv7em-none-eabihf/release/rustos:     file format elf32-littlearm


Disassembly of section .text:

00000040 <reset_handler>:
  40:   b580            push    {r7, lr}
  42:   466f            mov     r7, sp
  44:   f000 f803       bl      4e <_rust_start>

00000048 <debug_monitor_handler>:
  48:   b580            push    {r7, lr}
  4a:   466f            mov     r7, sp
  4c:   e7fe            b.n     4c <debug_monitor_handler+0x4>

0000004e <_rust_start>:
  4e:   b580            push    {r7, lr}
  50:   466f            mov     r7, sp
  52:   2000            movs    r0, #0
  54:   f240 0108       movw    r1, #8
  58:   f2c2 0020       movt    r0, #8224       ; 0x2020
  5c:   4685            mov     sp, r0
  5e:   f2c2 0100       movt    r1, #8192       ; 0x2000
  ...
```
Just good for debugging in general.
