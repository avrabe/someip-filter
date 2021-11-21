#![no_std]
#![no_main]

use aya_bpf::{
    macros::sock_ops,
    programs::SockOpsContext,
};

#[sock_ops(name="someip_filter")]
pub fn someip_filter(ctx: SockOpsContext) -> u32 {
    match unsafe { try_someip_filter(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_someip_filter(_ctx: SockOpsContext) -> Result<u32, u32> {
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
