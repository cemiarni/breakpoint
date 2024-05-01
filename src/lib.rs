pub mod breakpoint {

    #[cfg(not(disable_breakpoint))]
    #[cfg(not(custom_breakpoint))]
    #[cfg(magic_breakpoint)]
    #[macro_export]
    macro_rules! breakpoint_code {
        () => {"xchg bx, bx"}
    }

    #[cfg(not(disable_breakpoint))]
    #[cfg(not(custom_breakpoint))]
    #[cfg(not(magic_breakpoint))]
    #[macro_export]
    macro_rules! breakpoint_code {
        () => {"int3"}
    }

    #[cfg(not(disable_breakpoint))]
    #[macro_export(local_inner_macros)]
    macro_rules! breakpoint_inner {
        ( $asm_code:expr, $condition:expr ) => {
            if($condition) {
                unsafe {
                    std::arch::asm!($asm_code);
                }
            }
        };
        ($asm_code:literal, $condition:expr) => {
            if($condition) {
                unsafe {
                    std::arch::asm!($asm_code);
                }
            }
        };
        ( $asm_code:expr ) => {
            unsafe {
                std::arch::asm!($asm_code);
            }
        };
        ( $asm_code:literal ) => {
            unsafe {
                std::arch::asm!($asm_code);
            }
        };
    }

    #[cfg(not(disable_breakpoint))]
    #[cfg(not(custom_breakpoint))]
    #[macro_export(local_inner_macros)]
    macro_rules! breakpoint {
        ( $condition: expr ) => {
            breakpoint_inner!(
                breakpoint_code!(),
                $condition
            );
        };
        () => {
            breakpoint_inner!(
                breakpoint_code!()
            );
        };
    }

    #[cfg(disable_breakpoint)]
    #[macro_export]
    macro_rules! breakpoint {
        ( $condition:expr ) => {};
        () => {};
    }

}
