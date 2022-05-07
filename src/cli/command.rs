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
        "process" => match first_free {
            Some(action) => match action.as_ref() {
                "list" => {
                    let process_list = ccapi.get_process_list()?;
                    println!("{process_list:?}");
                },
                "name" => match second_free {
                    Some(raw_pid) => {
                        let pid: u32 = raw_pid.parse()?;
                        println!("{}", &ccapi.get_process_name(pid)?);
                    },
                    _ => bail!("A valid process id must be specified")
                }
                _ => bail!("Invalid action '{action}' specified for processes")
            }
            _ => bail!("A valid action for processes must be specified")
        }
        _ => bail!("Command '{cmd}' not recognized"),
    }

    Ok(())
}
