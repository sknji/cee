use core::fmt;
use std::fs;

pub struct Codegen {
    pub buf: String,
    pub depth: i64,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            depth: 0,
        }
    }

    pub fn ipush(&mut self) {
        self.icmd1ln("push", "%rax");
        self.depth += 1;
    }

    pub fn ipop(&mut self, arg: &str) {
        self.icmd1ln("pop", arg);
        self.depth -= 1;
    }

    fn write_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    pub fn iwrite(&mut self, s: &str) {
        self.write(format!("  {}", s).as_str())
    }

    pub fn write(&mut self, s: &str) {
        self.write_str(s);
    }

    pub fn iwriteln(&mut self, s: &str) {
        self.writeln(format!("  {}", s).as_str())
    }

    pub fn writeln(&mut self, s: &str) {
        self.write(s);
        self.line();
    }

    fn line(&mut self) {
        self.buf.push_str("\n");
    }

    pub fn flush(&mut self, filename: &str) {
        fs::write(filename, &self.buf).expect("Unable to write file");
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn icmd(&mut self, cmd: &str) {
        self.write(format!("  {}", cmd).as_str());
    }

    pub fn icmd1(&mut self, cmd: &str, arg1: &str) {
        self.write(format!("  {} {}", cmd, arg1).as_str());
    }

    pub fn icmd2(&mut self, cmd: &str, arg1: &str, arg2: &str) {
        self.write(format!("  {} {}, {}", cmd, arg1, arg2).as_str());
    }

    pub fn icmdln(&mut self, cmd: &str) {
        self.icmd(cmd);
        self.line();
    }

    pub fn icmd1ln(&mut self, cmd: &str, arg1: &str) {
        self.icmd1(cmd, arg1);
        self.line()
    }

    pub fn icmd2ln(&mut self, cmd: &str, arg1: &str, arg2: &str) {
        self.icmd2(cmd, arg1, arg2);
        self.line();
    }
}

impl fmt::Write for Codegen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}
