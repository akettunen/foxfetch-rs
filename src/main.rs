use std::io::{self, Write};
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

const O: &str = "\x1b[38;2;255;165;0m";
const R: &str = "\x1b[0m";

const LOGO: [&str; 4] = [
    r"    |\__/|",
    r"   /     \",
    r"  /_.~ ~,_\",
    r"     \@/",
];

fn main() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut info: Vec<String> = Vec::with_capacity(6);

    info.push(format!("{O}os{R}: windows"));

    if let Ok(cv) = hklm.open_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion") {
        let build: String = cv.get_value("CurrentBuildNumber").unwrap_or_default();
        let ubr: u32 = cv.get_value("UBR").unwrap_or(0);
        info.push(format!("{O}kernel{R}: 10.0.{build}.{ubr}"));
    }

    if let Ok(class) = hklm
        .open_subkey(r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}")
    {
        for name in class.enum_keys().flatten() {
            if let Ok(k) = class.open_subkey(&name) {
                if let Ok(desc) = k.get_value::<String, _>("DriverDesc") {
                    info.push(format!("{O}gpu{R}: {desc}"));
                }
            }
        }
    }

    let rows = LOGO.len().max(info.len());
    let mut out = String::with_capacity(512);
    for i in 0..rows {
        let left = LOGO.get(i).copied().unwrap_or("");
        let right = info.get(i).map(String::as_str).unwrap_or("");
        out.push_str(&format!("{O}{left:<11}{R}   {right}\n"));
    }

    let _ = io::stdout().lock().write_all(out.as_bytes());
}