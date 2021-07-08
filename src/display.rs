use super::cpu::CPU;
use std::fmt::Display;

pub fn print_freq(f: i32, raw: bool) {
    if raw {
        println!("{}", f);
    } else {
        println!("CPU freq is {} MHz", f)
    }
}

pub fn print_turbo(t: bool, raw: bool) {
    if raw {
        println!("{}", t);
    } else {
        println!(
            "{}",
            if t {
                "Turbo is enabled"
            } else {
                "Turbo is not enabled"
            }
        )
    }
}

fn print_vec<T: Display>(t: Vec<T>, raw: bool) {
    if raw {
        for x in t {
            println!("{}", x);
        }
    } else {
        print!("[ ");
        for x in t {
            print!("\"{}\" ", x);
        }
        print!("]");
    }
}

pub fn print_available_governors(available_governors: Vec<String>, raw: bool) {
    print_vec(available_governors, raw);
}

pub fn print_cpus(cpus: Vec<CPU>) {
    for x in cpus {
        println!("{}", x.name);
    }
}

pub fn print_cpu(cpu: &CPU) {
    println!(
        "{}: {}\t{}\t{}\t{}",
        cpu.name, cpu.max_freq, cpu.min_freq, cpu.cur_freq, cpu.gov
    );
}

pub fn print_cpu_speeds(cpu_speeds: Vec<i32>, raw: bool) {
    print_vec(cpu_speeds, raw);
}

pub fn print_cpu_governors(cpu_governors: Vec<String>, raw: bool) {
    print_vec(cpu_governors, raw);
}
