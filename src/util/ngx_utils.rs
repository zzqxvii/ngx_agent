use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::process::{Command, Output};

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
            // .arg("echo '1234'")
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
pub fn read_reverse(path: &str, mut line: i32) -> io::Result<Vec<String>> {
    let mut list = Vec::new();
    let nl = b'\n';
    let mut file = File::open(path)?;
    let mut pos = file.metadata()?.len() - 1;
    let mut last_line = pos;
    let mut buf_byt = [0; 1024 * 10];
    let mut byt = [0; 1];
    while pos > 0 && line > 0 {
        file.seek(SeekFrom::Start(pos))?;
        let _ = file.read(&mut byt)?;
        if byt[0] == nl {
            let line_len = last_line - pos - 1;
            if line_len > 0 {
                let tmp = &mut buf_byt[0..line_len as usize];
                file.seek(SeekFrom::Start(pos + 1))?;
                let _ = file.read(tmp);
                list.push(String::from_utf8_lossy(tmp).to_string());
            } else {
                list.push(String::new());
            }
            last_line = pos;
            line -= 1;
        }
        pos -= 1;
    }
    Ok(list)
}
