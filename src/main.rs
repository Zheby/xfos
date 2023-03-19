#![no_std]
#![no_main]
#[allow(special_module_name,non_snake_case)]
mod cores;
pub extern "C" fn _start() ->! {
    loop {}
}
