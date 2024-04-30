# Breakpoint

A collection of breakpoint solutions for C/C++.

## C/C++ -- breakpoint.h

### Example

```bash

$ cat > example.c <<END
#include "breakpoint.h"

int main(void) {

    breakpoint();  // will be trapped
    breakpoint(1 == 1); // condition is true, will be trapped
    breakpoint(0 == 1); // condition is false, won't be trapped

    return 0;
}
END
$ cc -g -o example example.c
$ gdb ./example
GNU gdb (GDB) 14.2
...
(gdb) start
Temporary breakpoint 1 at 0x1141: file example.c, line 3.
Starting program: /tmp/example
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Temporary breakpoint 1, main () at example.c:3
3       int main(void) {
(gdb) c
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
main () at example.c:6
6           breakpoint(1 == 1); // condition is true, will be trapped
(gdb) l
1       #include "breakpoint.h"
2
3       int main(void) {
4
5           breakpoint();  // will be trapped
6           breakpoint(1 == 1); // condition is true, will be trapped
7           breakpoint(0 == 1); // condition is false, won't be trapped
8
9           return 0;
10      }
(gdb) c
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
main () at example.c:7
7           breakpoint(0 == 1); // condition is false, won't be trapped
(gdb) c
Continuing.
[Inferior 1 (process 345061) exited normally]

```

### breakpoint.h confguration

You can define a following macros before include breakpoint.h:

 * NDEBUG: turn off breakpoint
 * MAGIC_BREAKPOINT: an alias of MAGIC_BREAKPOINT_GAS
 * MAGIC_BREAKPOINT_GAS: a bochs like magic break point in GAS syntax
 * MAGIC_BREAKPOINT_NASM: a bochs like magic break point in NASM syntax
 * BREAKPOINT_CODE: an assembly code as string, ex. `"int3"` or `"xchgw %bx, %bx"`
 * ASM_KEYWORD: a keyword or a function name which injects assembly code to the program, ex. `asm`
