mod idt;

use core::arch::naked_asm;
use lazy_static::lazy_static;

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                naked_asm!("mov rdi, rsp; sub rsp, 8; call {0}", sym $name)
            }
        }
        wrapper
    }}
}

macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                naked_asm!("pop rsi; mov rdi, rsp; sub rsp, 8; call {0}", sym $name)
            }
        }
        wrapper
    }}
}

pub fn init() {
    IDT.load();
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(6, handler!(invalid_opcode_handler));
        idt.set_handler(14, handler_with_error_code!(page_fault_handler));
        log::info!("{:#?}", idt);
        idt
    };
}

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> ! {
    log::info!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {}
}

extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) -> ! {
    log::info!(
        "\nEXCEPTION: INVALID OPCODE AT {:#x}\n{:#?}",
        stack_frame.instruction_pointer, stack_frame
    );
    loop {}
}

extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame, error_code: u64) -> ! {
    log::info!(
        "\nEXCEPTION: PAGE FAULT WITH ERROR CODE {:?}\n{:#?}",
        error_code, stack_frame
    );
    loop {}
}
