use log::debug;

const FD_STDOUT: usize = 1;

unsafe fn r_sp() -> usize {
    let mut sp:usize;
    llvm_asm!("mv $0, sp": "=r"(sp) ::: "volatile");
    sp
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            println!("Unsupported fd in sys_write!");
            -1
        }
    }
}
