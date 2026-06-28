#![no_std]
#![no_main]

#[link(name = "c")]
unsafe extern "C" {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern "C" fn handle_exit(_signal: libc::c_int) {
    unsafe {
        libc::_exit(0);
    }
}

extern "C" fn handle_sigchld(_signal: libc::c_int) {
    unsafe { while libc::waitpid(-1, core::ptr::null_mut(), libc::WNOHANG) > 0 {} }
}

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        libc::signal(libc::SIGINT, handle_exit as *const () as libc::sighandler_t);
        libc::signal(
            libc::SIGTERM,
            handle_exit as *const () as libc::sighandler_t,
        );
        libc::signal(
            libc::SIGCHLD,
            handle_sigchld as *const () as libc::sighandler_t,
        );
        loop {
            libc::pause();
        }
    }
}
