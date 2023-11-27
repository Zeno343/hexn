#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() {
    use core::arch::asm;

    #[allow(dead_code)]
    extern "C" {
        fn exit(_: core::ffi::c_int);
    }

    asm!(
        "mov rdi, rsp",
        "call main",
        "mov rax, 0",
        "call exit",
        options(noreturn)
    )
}

#[global_allocator]
static MALLOC: Malloc = Malloc {};
struct Malloc;
unsafe impl core::alloc::GlobalAlloc for Malloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        extern "C" {
            fn malloc(_: usize) -> *mut core::ffi::c_void;
        }

        malloc(layout.size() as _) as _
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _: core::alloc::Layout) {
        extern "C" {
            fn free(_: *mut core::ffi::c_void);
        }

        free(_ptr as _);
    }
}
