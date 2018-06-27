#![feature(test)]
extern crate brainfuck;
extern crate test;

use brainfuck::*;
use std::io;
use test::Bencher;

struct BlackHole;

impl io::Write for BlackHole {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Read for BlackHole {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

static HELLO_WORLD: &str = "++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<.";
static HELLO_WORLD_N: &str = "++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++>+++++++++++<<++>+++++++++++++++>+++--------------<<+<";

#[bench]
fn hello_world_compile(bencher: &mut Bencher) {
    bencher.iter(|| {
        let _ = test::black_box(HELLO_WORLD).into_program();
    });
}

#[bench]
fn hello_world_execute(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut process = Process::new(HELLO_WORLD, 1024, BlackHole, BlackHole);
        process.execute().unwrap();
    });
}

#[bench]
fn hello_world_execute_no_output(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut process = Process::new(HELLO_WORLD_N, 1024, BlackHole, BlackHole);
        process.execute().unwrap();
    });
}

macro_rules! write_bb {
    () => {
        let mut bh = BlackHole;
        bh.write(test::black_box(&[0u8])).unwrap();
    };
}

#[bench]
fn simulated_hello_world(bencher: &mut Bencher) {
    use std::io::Write;
    bencher.iter(|| {
        let mut m = [0u8; 10];
        let mut mp = 0;

        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        while m[mp] != 0 {
            mp = mp + 1;
            m[mp] = m[mp] + 1;
            mp = mp + 1;
            m[mp] = m[mp] + 1;
            m[mp] = m[mp] + 1;
            m[mp] = m[mp] + 1;
            mp = mp + 1;
            m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
            m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
            m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
            m[mp] = m[mp] + 1;
            mp = mp + 1;
            m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
            m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
            m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
            m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
            m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
            mp = mp - 1; mp = mp - 1;
            mp = mp - 1; mp = mp - 1;
            m[mp] = m[mp] - 1;
        }
        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        mp = mp + 1;
        m[mp] = m[mp] + 1;m[mp] = m[mp] + 1;
        write_bb!();
        mp = mp + 1;
        m[mp] = m[mp] + 1;
        write_bb!();
        m[mp] = m[mp] + 1;m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1;m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1;m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1;
        write_bb!();
        write_bb!();
        m[mp] = m[mp] + 1;m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1;
        write_bb!();
        mp = mp - 1;mp = mp - 1;
        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        write_bb!();
        mp = mp + 1;
        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1; m[mp] = m[mp] + 1; m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1; m[mp] = m[mp] + 1; m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1; m[mp] = m[mp] + 1; m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        write_bb!();
        mp = mp + 1;
        write_bb!();
        m[mp] = m[mp] + 1; m[mp] = m[mp] + 1;
        m[mp] = m[mp] + 1;
        write_bb!();
        m[mp] = m[mp] - 1; m[mp] = m[mp] - 1;
        m[mp] = m[mp] - 1; m[mp] = m[mp] - 1;
        m[mp] = m[mp] - 1; m[mp] = m[mp] - 1;
        write_bb!();
        m[mp] = m[mp] - 1; m[mp] = m[mp] - 1;
        m[mp] = m[mp] - 1; m[mp] = m[mp] - 1;
        m[mp] = m[mp] - 1; m[mp] = m[mp] - 1;
        m[mp] = m[mp] - 1;
        write_bb!();
        mp = mp - 1;
        mp = mp - 1;
        m[mp] = m[mp] + 1;
        write_bb!();
        mp = mp - 1;
        write_bb!();
        std::mem::drop(mp);
    });
}

#[bench]
fn u32_xor_u32_x1000(bencher: &mut Bencher) {
    let a: u32 = test::black_box(666666666);
    let b: u32 = test::black_box(233333333);
    bencher.iter(|| {
        for _ in 1..1000 {
            let _ = test::black_box(a ^ b);
        }
    });
}
