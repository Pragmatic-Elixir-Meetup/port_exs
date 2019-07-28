#![allow(non_upper_case_globals)]

extern crate byteorder;
extern crate eetf;
extern crate rand;

use byteorder::{BigEndian, WriteBytesExt};
use eetf::{Binary, Term};
use rand::Rng;
use std::{ffi::CStr, io, io::Write, mem};

const PIPE_ACTIVE_BUF: &str = "1";

static mut pipe_fds: [i32; 2]  = [0 as libc::c_int; 2];
static mut rfid_result: [i8; 128] = [0 as libc::c_char; 128];

fn gen_rand_data() {
    let data = rand::thread_rng().gen_range(100_000_000, 1_000_000_000).to_string();

    unsafe {
        libc::memset(rfid_result.as_mut_ptr() as *mut core::ffi::c_void, 0, 128);
        libc::strncpy(rfid_result.as_mut_ptr(), data.as_ptr() as *const i8, data.len());

        libc::write(pipe_fds[1], PIPE_ACTIVE_BUF.as_ptr() as *const core::ffi::c_void, PIPE_ACTIVE_BUF.len());
    }
}

fn main() {
    unsafe {
        libc::pipe(pipe_fds.as_mut_ptr());

        let pipe_read_fd = pipe_fds[0];
        let pipe_write_fd = pipe_fds[1];

        let flags = libc::fcntl(pipe_write_fd, libc::F_GETFL);
        let result = libc::fcntl(pipe_write_fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
        if result < 0 {
            panic!("Failed to call `fcntl` - {}", result);
        }

        loop {
            let mut read_fds: libc::fd_set = mem::zeroed();

            libc::FD_ZERO(&mut read_fds);
            libc::FD_SET(pipe_read_fd, &mut read_fds);

            // Monitors the `close` flag (0 byte).
            libc::FD_SET(libc::STDOUT_FILENO, &mut read_fds);

            let mut timeout: libc::timeval = mem::zeroed();
            timeout.tv_sec = 1;
            timeout.tv_usec = 0;

            let select_err = libc::select(
                pipe_read_fd + 1,
                &mut read_fds,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut timeout
            );

            if select_err < 0 {
                let errno = *libc::__error();
                if errno == libc::EINTR {
                    continue;
                }

                println!("Returns an unexpected error from `select` - {}", errno);
                break;
            }

            if select_err == 0 {
                gen_rand_data();

                let _rfid_code = CStr::from_ptr(rfid_result.as_ptr()).to_owned();
                // println!("Random Rfid code is generated - {:?}", rfid_code);

                continue;
            }

            if libc::FD_ISSET(libc::STDOUT_FILENO, &mut read_fds) {
                // Receives the `close` flag.
                break;
            }

            if !libc::FD_ISSET(pipe_read_fd, &mut read_fds) {
                continue;
            }

            let mut buf = [0 as libc::c_char; 1];
            libc::read(pipe_read_fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len());

            let len = libc::strlen(&rfid_result as *const i8);
            let original_data = &*(&rfid_result[0..len] as *const [i8] as *const [u8]);

            let mut encoded_data = Vec::new();
            let term = Term::from(Binary::from(original_data));
            term.encode(&mut encoded_data).unwrap();

            let mut sender = io::stdout();
            sender.write_u16::<BigEndian>(encoded_data.len() as u16).expect("failed to write data size");
            sender.write_all(&encoded_data).expect("failed to write data");
            sender.flush().expect("failed to flush stdout");
        }

        libc::close(pipe_read_fd);
        libc::close(pipe_write_fd);
    }
}
