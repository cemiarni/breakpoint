#ifndef BREAKPOINT_H
#define BREAKPOINT_H

    #ifndef NDEBUG

        #ifndef ASM_KEYWORD
            #define ASM_KEYWORD(code) __asm__(code)
        #endif

        #ifdef MAGIC_BREAKPOINT
            #define MAGIC_BREAKPOINT_GAS
        #endif

        #ifdef MAGIC_BREAKPOINT_NASM
            #define BREAKPOINT_CODE "xchg bx, bx"
        #endif

        #ifdef MAGIC_BREAKPOINT_GAS
            #define BREAKPOINT_CODE "xchgw %bx, %bx"
        #endif

        #ifndef BREAKPOINT_CODE
            #define BREAKPOINT_CODE "int3"
        #endif

        #define __BREAKPOINT() ASM_KEYWORD(BREAKPOINT_CODE)


        #define __NUMARGS(...) \
            ((sizeof((int[]){0, __VA_ARGS__}) / sizeof(int)) - 1)

        #define breakpoint(...) \
            if(((int[]){1, __VA_ARGS__})[__NUMARGS(__VA_ARGS__)]) { \
                __BREAKPOINT(); \
            }

    #else

        #define breakpoint(...)

    #endif

#endif
