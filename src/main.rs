#![feature(decl_macro)]
#![no_std]
#![no_main]
#[allow(special_module_name,non_snake_case)]
mod cores;
pub mod vga;
#[no_mangle]
pub extern "C" fn _start() ->! {
    println!("hello word");
    loop {}

}


use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}