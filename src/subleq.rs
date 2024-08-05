#![crate_name = "subleq"]
use std::io::prelude::*;
mod eforth;

/// * `CORE_SIZE` is the total number of cells addressable by the virtual machine
const CORE_SIZE: usize = 0x10000;

/// `fputc` writes a single character of output to a file, and returns
/// all bits set on an error. It emulates the C function of the same name,
/// and is not a recommended way to output data in Rust, but is required for
/// the VM.
///
/// # Arguments
///
/// * `output`  - Output stream to write to
/// * `t`       - Single byte to write
///
/// # Returns
///
/// This function returns `t` on success and `0xffff` on error
///
fn fputc(output: &mut dyn Write, t: u8) -> u16 {
	let u: [u8; 1] = [t as u8];
	if 1 == output.write(&u).unwrap() { t as u16 } else { 0xffff }
}

/// `fputc` gets a single character from an input stream, like the C function
/// with the same name, it returns all bits set (-1) on error. This is not a
/// very idiomatic way of doing things from a Rust point of view, but this
/// function is used by the virtual machine to get input, and it expects
/// errors to be signaled in the message.
///
/// # Arguments
///
/// `input` - Input stream to read from
///
/// # Returns
///
/// This function returns a single byte on success in the lower half a
/// 16-bit value, and all bits set (or `0xffff`) on failure.
fn fgetc(input: &mut dyn Read) -> u16 {
	let mut u: [u8; 1] = [0];
	if 1 == input.read(&mut u).unwrap() { u[0] as u16 } else { 0xffff }
}

/// # SUBLEQ Virtual Machine in Rust
///
/// * LICENSE:    MIT
/// * AUTHOR:     Richard James Howe
/// * COPYRIGHT:  Richard James Howe (2024)
/// * CONTACT:    <howe.r.j.89@gmail.com>
/// * REPOSITORY: <https://github.com/howerj/subleq-rust>
/// 
pub struct VM {
	/// The virtual machine has minimal state, a program counter (`pc`).
	pc: u16,
	/// into `core` with `rp` and `sp`
	//#[derive(Copy, Clone)]
	core: [u16; CORE_SIZE] 
}

impl VM {

	/// `new` constructs a new virtual machine image that can be passed to `run`
	/// straight away, as the program memory is copied from a default image
	/// that contains an eForth interpreter.
	pub fn new() -> Self { 
		let mut r = VM { pc: 0, core: [0; CORE_SIZE] };

		for i in 0..eforth::EFORTH_CORE.len() {
			r.core[i] = eforth::EFORTH_CORE[i] as u16;
		}
		r
	}

	/// `reset` sets the VMs registers back to their defaults.
	pub fn reset(&mut self) {
		self.pc = 0;
	}

	pub fn run(&mut self, input: &mut dyn Read, output: &mut dyn Write) -> i32 {
		let mut pc = self.pc;
		let mut m = self.core;

        while pc & 0x8000 == 0 {
            let a = m[(pc + 0) as usize];
            let b = m[(pc + 1) as usize];
            let c = m[(pc + 2) as usize];
            pc = pc + 3;

            if a == 0xFFFF {
                m[b as usize] = fgetc(input);
            } else if b == 0xFFFF {
                fputc(output, m[a as usize] as u8);
            } else {
                let r = m[b as usize].wrapping_sub(m[a as usize]);
                if (0x8000 & r == 0x8000) || (r == 0x0000) {
                    pc = c;
                }
                m[b as usize] = r;
            }
		}
		self.pc = pc;
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run() {
		let mut vm = VM::new();
	}
}

