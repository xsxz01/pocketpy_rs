use std::ffi::CString;

use pocketpy_rs::{
    py_CompileMode_EXEC_MODE, py_Ref, py_exec, py_finalize, py_initialize, py_newmodule, PK_VERSION,
};

fn main() {
    // 读取文件 scripts/main.py
    let main_script_raw = include_str!("../scripts/main.py");
    unsafe {
        py_initialize();
        println!("pocketpy bind success!");
        println!("pocketpy version: {}", String::from_utf8_lossy(PK_VERSION));
        let option_ptr: py_Ref = py_newmodule(CString::new(".").unwrap().as_ptr());
        let main_script = CString::new(main_script_raw);
        let is_ok = py_exec(
            main_script.unwrap().as_ptr(),
            CString::new("stdout.log").unwrap().as_ptr(),
            py_CompileMode_EXEC_MODE,
            option_ptr,
        );
        if !is_ok {
            print!("exec failed")
        }
        py_finalize();
    }
}
