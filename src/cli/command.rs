use std::str::FromStr;

use anyhow::{bail, Result};
use ccapi::{BuzzerType, CCAPI, NotifyIcon, ShutdownMode};
use getopts::Matches;

pub fn run(ccapi: &CCAPI, matches: &Matches) -> Result<()> {
    let cmd = matches.opt_str("command").unwrap();

    let first_free = matches.free.get(0);
    let second_free = matches.free.get(1);

    match cmd.as_ref() {
        "ringbuzzer" => match first_free {
            Some(raw_buzzer_type) => {
                let buzzer_type = BuzzerType::from_str(&raw_buzzer_type)?;
                ccapi.ring_buzzer(buzzer_type)?;
            }
            _ => bail!("A buzzer type must be provided"),
        },
        "shutdown" => ccapi.shutdown(ShutdownMode::Shutdown)?,
        "restart" => match first_free {
            Some(raw_restart_mode) => {
                let shutdown_mode = match raw_restart_mode.as_ref() {
                    "soft" => ShutdownMode::SoftReboot,
                    "hard" => ShutdownMode::HardReboot,
                    _ => bail!("Invalid restart mode '{}' provided", raw_restart_mode),
                };
                ccapi.shutdown(shutdown_mode)?;
            }
            _ => ccapi.shutdown(ShutdownMode::HardReboot)?,
        },
        "notify" => match (first_free, second_free) {
            (Some(raw_notify_icon), Some(raw_message)) => {
                let notify_icon = NotifyIcon::from_str(raw_notify_icon)?;
                ccapi.notify(notify_icon, &raw_message)?;
            },
            _ => bail!("A valid icon and message must be provided")
        },
        "firmware" => {
            let firmware_info = ccapi.get_firmware_info()?;
            println!("{firmware_info:?}");
        },
        "temperature" | "temp" => {
            let temperature_info = ccapi.get_temperature_info()?;
            println!("{temperature_info:?}");
        },
        _ => bail!("Command '{}' not recognized", cmd),
    }

    Ok(())
}
