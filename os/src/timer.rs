use riscv::register::time;

pub fn sleep_us(us: usize) {
    let time = time::read();
    while time::read() - time < us * 10 {}
}