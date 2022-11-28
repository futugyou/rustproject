#[cfg(desktop)]
mod desktop;
mod cmd;

fn main() {
    #[cfg(desktop)]
    desktop::main()
}
