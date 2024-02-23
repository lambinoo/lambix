use std::process::Command;

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[derive(ValueEnum)]
enum OutputType {
    Qemu,
    Serial
}

#[derive(Parser, Debug)]
struct Lambemu {
    #[arg(long, default_value = "target/x86_64-unknown-lambix/release/lambix")]
    iso_path: String,

    #[arg(long, default_value = "qemu-system-x86_64")]
    qemu: String,

    #[arg(long, short, value_enum, default_value_t = OutputType::Serial)]
    output: OutputType,

    #[arg(long, short, default_value = "1G")]
    memory: String,

    #[arg(long, short, default_value_t = 1)]
    smp: usize
}

fn main() {
    let args = Lambemu::parse();

    let smp = args.smp.to_string();

    let mut command = Command::new(args.qemu)
        .args(["--enable-kvm"])
        .args(["-cdrom", &args.iso_path])
        .args(["-smp", &smp])
        .args(["-m", &args.memory])
        .args(["-no-reboot", "-no-shutdown"]);
}
