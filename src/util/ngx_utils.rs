use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::process::Command;

use serde::{Deserialize, Serialize};

pub enum NgxCmd {
    // bin, conf
    START(String, String),
    RELOAD(String, String),
    CHECK(String, String),
    DumpConf(String, String),

    // bin
    STOP(String),
    QUIT(String),
    REOPEN(String),
    VERSION(String),
    CONFIGURE(String),

    // pid
    // nginx -s reload
    SIGHUP(String),
    // nginx -s stop
    SIGTERM(String),
    // nginx -s quit
    SIGQUIT(String),
    // nginx -s reopen
    SIGUSR1(String),
    // 平滑升级
    SIGUSR2(String),
    // 关闭工作进程
    SIGWINCH(String),
}

impl NgxCmd {
    pub fn cmd(&self) -> String {
        match self {
            NgxCmd::START(bin, conf) => format!("{} -c {}", bin, conf),
            NgxCmd::CHECK(bin, conf) => format!("{} -t -c {}", bin, conf),
            NgxCmd::RELOAD(bin, conf) => format!("{} -s reload -c {}", bin, conf),
            NgxCmd::DumpConf(bin, conf) => format!("{} -T -c {}", bin, conf),

            NgxCmd::STOP(bin) => format!("{} -s stop", bin),
            NgxCmd::QUIT(bin) => format!("{} -s quit ", bin),
            NgxCmd::REOPEN(bin) => format!("{} -s reopen ", bin),
            NgxCmd::VERSION(bin) => format!("{} -v", bin),
            NgxCmd::CONFIGURE(bin) => format!("{} -V", bin),

            NgxCmd::SIGHUP(pid) => format!("kill -HUP {} ", pid),
            NgxCmd::SIGTERM(pid) => format!("kill -TERM {} ", pid),
            NgxCmd::SIGQUIT(pid) => format!("kill -QUIT {} ", pid),
            NgxCmd::SIGUSR1(pid) => format!("kill -USR1 {} ", pid),
            NgxCmd::SIGUSR2(pid) => format!("kill -USR2 {} ", pid),
            NgxCmd::SIGWINCH(pid) => format!("kill -WINCH {} ", pid),
        }
    }

    pub fn exec(&self) -> ShellOutput {
        let output = Command::new("sh")
            .arg("-c")
            .arg(self.cmd())
            .output()
            .unwrap();

        ShellOutput {
            exit_code: output.status.code().unwrap(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ShellOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

// linux: newLine -> \n
fn read_lines_reverse(mut file: File, mut lines: u32) -> std::io::Result<Vec<String>> {
    let nl = b'\n';
    let metadata = file.metadata()?;
    let mut pos = metadata.len();
    let mut byte_buf: [u8; 1] = [0];
    let mut line_bytes = Vec::new();
    let mut str_lines = Vec::new();
    while pos > 0 && lines > 0 {
        pos -= 1;
        file.seek(SeekFrom::Start(pos))?;
        file.read(&mut byte_buf)?;
        if byte_buf[0] != nl {
            line_bytes.push(byte_buf[0]);
        } else {
            line_bytes.reverse();
            str_lines.push(String::from_utf8_lossy(&line_bytes).to_string());
            line_bytes.clear();
            lines -= 1;
        }
    }
    if !str_lines.is_empty() {
        line_bytes.reverse();
        str_lines.push(String::from_utf8_lossy(&line_bytes).to_string());
    }
    Ok(str_lines)
}