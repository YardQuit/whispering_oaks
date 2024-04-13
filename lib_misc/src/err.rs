use std::io::{self, BufWriter, Write};

#[derive(Debug)]
pub struct WoErr<T, U, V> {
    err_msg: T,
    cust_msg: U,
    payload: V,
    exit_code: i32,
}

impl<T, U, V> WoErr<T, U, V> {
    pub fn new(err_msg: T, cust_msg: U, payload: V, exit_code: i32) -> Self {
        Self {
            err_msg,
            cust_msg,
            payload,
            exit_code,
        }
    }
}

pub fn error_handling<T, U, V>(wo_err: &WoErr<T, U, V>)
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
    V: std::fmt::Debug,
{
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    if !format!("{:?}", wo_err.err_msg).is_empty() {
        writeln!(handle, "\nerror:\t{:?}", wo_err.err_msg).unwrap();
    }

    if !format!("{:?}", wo_err.cust_msg).is_empty() {
        writeln!(handle, "\t{:?}", wo_err.cust_msg).unwrap();
    }

    if !format!("{:#?}", wo_err.payload).is_empty() {
        writeln!(handle, "\t{:#?}", wo_err.payload).unwrap();
    }

    writeln!(handle, "\t(exit_code: {})", wo_err.exit_code).unwrap();

    handle.flush().unwrap();
    std::process::exit(wo_err.exit_code);
}
