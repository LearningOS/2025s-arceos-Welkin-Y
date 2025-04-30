#![no_std]
#![no_main]

use core::panic::PanicInfo;

//#[no_mangle]
//unsafe extern "C" fn _start() -> ! {
//    core::arch::asm!(
//        "csrr a1, mhartid",
//        "ld a0, 64(zero)",
//        "li a7, 8",
//        "ecall",
//        options(noreturn)
//    )
//}
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    let a0 = 0x6688 as usize;
    let a1 = 0x1234 as usize;

    core::arch::asm!(
        "mv a0, {0}",
        "mv a1, {1}",
        "li a7, 8",    // SBI reset
        "ecall",
        in(reg) a0,
        in(reg) a1,
        options(noreturn)
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
