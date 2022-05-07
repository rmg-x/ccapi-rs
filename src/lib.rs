use anyhow::{bail, Context, Result};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use ureq::Error;

const DEFAULT_CCAPI_PORT: u16 = 6333;
const DEFAULT_RADIX: u32 = 16;

pub struct CCAPI {
    base_url: String,
    ip_address: IpAddr,
    port: u16,
}

#[derive(Debug)]
pub enum BuzzerType {
    Continuous,
    Single,
    Double,
    Triple,
}

impl BuzzerType {
    pub fn get_value(&self) -> i32 {
        match *self {
            BuzzerType::Continuous => 0,
            BuzzerType::Single => 1,
            BuzzerType::Double => 2,
            BuzzerType::Triple => 3,
        }
    }
}

impl FromStr for BuzzerType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(BuzzerType::Single),
            "double" => Ok(BuzzerType::Double),
            "triple" => Ok(BuzzerType::Triple),
            "continuous" => Ok(BuzzerType::Continuous),
            _ => bail!("Invalid buzzer type '{}' provided", &s),
        }
    }
}

#[derive(Debug)]
pub enum ShutdownMode {
    Shutdown,
    SoftReboot,
    HardReboot,
}

impl ShutdownMode {
    pub fn get_value(&self) -> i32 {
        match *self {
            ShutdownMode::Shutdown => 1,
            ShutdownMode::SoftReboot => 2,
            ShutdownMode::HardReboot => 3,
        }
    }
}

#[derive(Debug)]
pub enum NotifyIcon {
    Info,
    Caution,
    Friend,
    Slider,
    WrongWay,
    Dialog,
    DialogShadow,
    Text,
    Pointer,
    Grab,
    Hand,
    Pen,
    Finger,
    Arrow,
    ArrowRight,
    Progress,
    Trophy1,
    Trophy2,
    Trophy3,
    Trophy4,
}

impl NotifyIcon {
    fn get_value(&self) -> i32 {
        match *self {
            NotifyIcon::Info => 0,
            NotifyIcon::Caution => 1,
            NotifyIcon::Friend => 2,
            NotifyIcon::Slider => 3,
            NotifyIcon::WrongWay => 4,
            NotifyIcon::Dialog => 5,
            NotifyIcon::DialogShadow => 6,
            NotifyIcon::Text => 7,
            NotifyIcon::Pointer => 8,
            NotifyIcon::Grab => 9,
            NotifyIcon::Hand => 10,
            NotifyIcon::Pen => 11,
            NotifyIcon::Finger => 12,
            NotifyIcon::Arrow => 13,
            NotifyIcon::ArrowRight => 14,
            NotifyIcon::Progress => 15,
            NotifyIcon::Trophy1 => 16,
            NotifyIcon::Trophy2 => 17,
            NotifyIcon::Trophy3 => 18,
            NotifyIcon::Trophy4 => 19,
        }
    }
}

impl FromStr for NotifyIcon {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "info" => Ok(NotifyIcon::Info),
            "caution" => Ok(NotifyIcon::Caution),
            "friend" => Ok(NotifyIcon::Friend),
            "slider" => Ok(NotifyIcon::Slider),
            "wrongway" => Ok(NotifyIcon::WrongWay),
            "dialog" => Ok(NotifyIcon::Dialog),
            "dialogshadow" => Ok(NotifyIcon::DialogShadow),
            "text" => Ok(NotifyIcon::Text),
            "pointer" => Ok(NotifyIcon::Pointer),
            "grab" => Ok(NotifyIcon::Grab),
            "hand" => Ok(NotifyIcon::Hand),
            "pen" => Ok(NotifyIcon::Pen),
            "finger" => Ok(NotifyIcon::Finger),
            "arrow" => Ok(NotifyIcon::Arrow),
            "arrowright" => Ok(NotifyIcon::ArrowRight),
            "progress" => Ok(NotifyIcon::Progress),
            "trophy1" => Ok(NotifyIcon::Trophy1),
            "trophy2" => Ok(NotifyIcon::Trophy2),
            "trophy3" => Ok(NotifyIcon::Trophy3),
            "trophy4" => Ok(NotifyIcon::Trophy4),
            _ => bail!("Invalid notify icon '{}' provided", &s),
        }
    }
}

#[derive(Debug)]
pub enum ConsoleLed {
    LedRed,
    LedGreen,
}

impl ConsoleLed {
    pub fn get_value(&self) -> i32 {
        match *self {
            ConsoleLed::LedRed => 0,
            ConsoleLed::LedGreen => 1,
        }
    }
}

#[derive(Debug)]
pub enum LedStatus {
    LedOff,
    LedOn,
    LedBlink,
}

impl LedStatus {
    pub fn get_value(&self) -> i32 {
        match *self {
            LedStatus::LedOff => 0,
            LedStatus::LedOn => 1,
            LedStatus::LedBlink => 2,
        }
    }
}

#[derive(Debug)]
pub enum ConsoleType {
    Unknown,
    CEX,
    DEX,
    TOOL,
}

impl ConsoleType {
    pub fn get_value(&self) -> i32 {
        match *self {
            ConsoleType::Unknown => 0,
            ConsoleType::CEX => 1,
            ConsoleType::DEX => 2,
            ConsoleType::TOOL => 3,
        }
    }
}

impl From<i32> for ConsoleType {
    fn from(value: i32) -> Self {
        match value {
            1 => ConsoleType::CEX,
            2 => ConsoleType::DEX,
            3 => ConsoleType::TOOL,
            _ => ConsoleType::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct FirmwareInfo {
    pub firmware_version: u32,
    pub ccapi_version: u32,
    pub console_type: ConsoleType,
}

#[derive(Debug)]
pub struct TemperatureInfo {
    pub cell: i32,
    pub rsx: i32,
}

impl CCAPI {
    fn generate_base_url(socket_addr: &SocketAddr) -> String {
        format!("http://{}/ccapi/", socket_addr)
    }

    /// Returns a new instance of CCAPI
    ///
    /// ### Arguments
    ///
    /// * `console_ip` - The IPv4 address of the console to communicate with
    ///
    /// ### Examples
    ///
    /// ```
    /// use ccapi::CCAPI;
    /// use std::net::Ipv4Addr;
    ///
    /// // Typically, the IP will be in a private range (e.g. 192.168.x.x)
    /// let ip: Ipv4Addr = "127.0.0.1".parse().unwrap();
    /// let ccapi = CCAPI::new(ip);
    /// ```
    pub fn new(console_ip: Ipv4Addr) -> Self {
        let ip_address = IpAddr::V4(console_ip);
        let port = DEFAULT_CCAPI_PORT;

        let console_socket = SocketAddr::new(ip_address, port);
        let base_url = Self::generate_base_url(&console_socket);

        CCAPI {
            base_url,
            ip_address,
            port,
        }
    }

    /// Sets the IPv4 address of the console to communicate with
    ///
    /// ### Arguments
    ///
    /// * `console_ip` - The IPv4 address of the console to communicate with
    ///
    /// ### Examples
    ///
    /// ```
    /// use ccapi::CCAPI;
    /// use std::net::Ipv4Addr;
    ///
    /// let mut ccapi = CCAPI::new(Ipv4Addr::LOCALHOST);
    ///
    /// let new_ip: Ipv4Addr = "192.168.0.0".parse().unwrap();
    /// ccapi.set_console_ip(new_ip);
    /// ```
    pub fn set_console_ip(&mut self, console_ip: Ipv4Addr) {
        let ip_address = IpAddr::V4(console_ip);
        let console_socket = SocketAddr::new(ip_address, self.port);

        self.ip_address = ip_address;
        self.base_url = Self::generate_base_url(&console_socket);
    }

    /// Sets the port to communicate with
    ///
    /// ### Arguments
    ///
    /// * `port` - The port to communicate with
    ///
    /// ### Examples
    ///
    /// ```
    /// use ccapi::CCAPI;
    /// use std::net::Ipv4Addr;
    ///
    /// let mut ccapi = CCAPI::new(Ipv4Addr::LOCALHOST);
    ///
    /// let port: u16 = 6333;
    /// ccapi.set_console_port(port);
    /// ```
    pub fn set_console_port(&mut self, port: u16) {
        let console_socket = SocketAddr::new(self.ip_address, port);

        self.base_url = Self::generate_base_url(&console_socket);
    }

    /// Rings the console buzzer with the specified [BuzzerType](crate::BuzzerType)
    ///
    /// ### Arguments
    ///
    /// * `buzzer_type` - The buzzer type to use
    ///
    /// ### Examples
    ///
    /// ```
    /// use ccapi::{CCAPI, BuzzerType};
    /// use std::net::Ipv4Addr;
    ///
    /// let ccapi = CCAPI::new(Ipv4Addr::LOCALHOST);
    ///
    /// ccapi.ring_buzzer(BuzzerType::Single);
    /// ```
    pub fn ring_buzzer(&self, buzzer_type: BuzzerType) -> Result<()> {
        let buzzer_code = buzzer_type.get_value();

        let req_command = "ringbuzzer";
        let req_query = format!("{}?type={}", req_command, buzzer_code);
        let req_url = format!("{}{}", self.base_url, req_query);

        ureq::get(&req_url).call().with_context(|| {
            format!(
                "Failed to send command '{}', with buzzer type: {:?}",
                req_command, buzzer_type
            )
        })?;

        Ok(())
    }

    /// Shutdown/restart the console, depending on the [ShutdownMode](crate::ShutdownMode) given
    ///
    /// ### Arguments
    ///
    /// * `shutdown_mode` - The shutdown mode to use
    ///
    /// ### Examples
    ///
    /// ```
    /// use ccapi::{CCAPI, ShutdownMode};
    /// use std::net::Ipv4Addr;
    ///
    /// let ccapi = CCAPI::new(Ipv4Addr::LOCALHOST);
    ///
    /// ccapi.shutdown(ShutdownMode::Shutdown);
    /// ```
    pub fn shutdown(&self, shutdown_mode: ShutdownMode) -> Result<()> {
        let shutdown_code = shutdown_mode.get_value();

        let req_command = "shutdown";
        let req_query = format!("{}?mode={}", req_command, shutdown_code);
        let req_url = format!("{}{}", self.base_url, req_query);

        let response = ureq::get(&req_url).call();

        // After making the shutdown call, a transport
        // error occurs even though the shutdown is successful.
        // We're going to assume CCAPI is telling us
        // the shutdown succeeded.
        match response {
            Ok(_) | Err(Error::Transport(_)) => Ok(()),
            Err(e) => bail!("{}", e),
        }
    }

    /// Displays a notification message with an icon
    ///
    /// ### Arguments
    ///
    /// * `notify_icon` - Icon to display
    /// * `message` - Message to display
    ///
    /// ### Examples
    ///
    /// ```
    /// use ccapi::{CCAPI, NotifyIcon};
    /// use std::net::Ipv4Addr;
    ///
    /// let ccapi = CCAPI::new(Ipv4Addr::LOCALHOST);
    ///
    /// ccapi.notify(NotifyIcon::NotifyInfo, "hello world");
    /// ```
    pub fn notify(&self, notify_icon: NotifyIcon, message: &str) -> Result<()> {
        let notify_code = notify_icon.get_value();

        let req_command = "notify";
        let req_query = format!("{}?id={}&msg={}", req_command, notify_code, message);
        let req_url = format!("{}{}", self.base_url, req_query);

        ureq::get(&req_url).call().with_context(|| {
            format!(
                "Failed to send command '{}', with icon: {:?}, message: {}",
                req_command, notify_icon, &message
            )
        })?;

        Ok(())
    }

    /// Sets console LED color and status
    pub fn set_console_led(&self, color: ConsoleLed, status: LedStatus) -> Result<()> {
        let led_color_code = color.get_value();
        let led_status_code = status.get_value();

        let req_command = "setconsoleled";
        let req_query = format!(
            "{}?color={}&status={}",
            req_command, led_color_code, led_status_code
        );
        let req_url = format!("{}{}", self.base_url, req_query);

        ureq::get(&req_url).call().with_context(|| {
            format!(
                "Failed to send command '{}' with LED color: {:?}, LED status: {:?}",
                req_command, color, status
            )
        })?;

        Ok(())
    }

    /// Returns console firmware information
    pub fn get_firmware_info(&self) -> Result<FirmwareInfo> {
        let req_command = "getfirmwareinfo";
        let req_url = format!("{}{}", self.base_url, req_command);

        let response = ureq::get(&req_url)
            .call()
            .with_context(|| format!("Failed to send command '{}'", req_command))?;

        let body = response.into_string()?;

        let lines: Vec<&str> = body.split('\n').collect::<Vec<_>>();

        let raw_firmware_version = lines.get(1);
        let raw_ccapi_version = lines.get(2);
        let raw_console_type = lines.get(3);

        match (raw_firmware_version, raw_ccapi_version, raw_console_type) {
            (Some(fv), Some(cv), Some(ct)) => {
                let firmware_version: u32 = fv.parse()?;
                let ccapi_version = u32::from_str_radix(cv, DEFAULT_RADIX)?;
                let console_type_parsed: i32 = ct.parse()?;

                let firmware_info = FirmwareInfo {
                    firmware_version,
                    ccapi_version,
                    console_type: ConsoleType::from(console_type_parsed),
                };

                Ok(firmware_info)
            }
            _ => bail!("Could not retrieve firmware information"),
        }
    }

    /// Returns temperature information in celsius
    pub fn get_temperature_info(&self) -> Result<TemperatureInfo> {
        let req_command = "gettemperature";
        let req_url = format!("{}{}", self.base_url, req_command);

        let response = ureq::get(&req_url)
            .call()
            .with_context(|| format!("Failed to send command '{}'", req_command))?;

        let body = response.into_string()?;

        let lines: Vec<&str> = body.split('\n').collect::<Vec<_>>();

        let raw_cell_temp = lines.get(1);
        let raw_rsx_temp = lines.get(2);

        match (raw_cell_temp, raw_rsx_temp) {
            (Some(ct), Some(rt)) => {
                let cell_temp = i32::from_str_radix(ct, DEFAULT_RADIX)?;
                let rsx_temp = i32::from_str_radix(rt, DEFAULT_RADIX)?;

                let temp_info = TemperatureInfo {
                    cell: cell_temp,
                    rsx: rsx_temp,
                };

                Ok(temp_info)
            }
            _ => bail!("Could not retrieve temperature information"),
        }
    }

    /// Returns a list of process identifiers (pid)
    pub fn get_process_list(&self) -> Result<Vec<u32>> {
        let req_command = "getprocesslist";
        let req_url = format!("{}{}", self.base_url, req_command);

        let response = ureq::get(&req_url)
            .call()
            .with_context(|| format!("Failed to send command '{}'", req_command))?;

        let body = response.into_string()?;

        let lines: Vec<&str> = body.split('\n').collect::<Vec<_>>();
        let mut process_ids = Vec::new();

        // Skip first line which contains the "status" code
        for raw_pid in &lines[1..] {
            if let Ok(actual_pid) = u32::from_str(raw_pid) {
                process_ids.push(actual_pid);
            }
        }

        Ok(process_ids)
    }

    /// Returns a process name from its identifier (pid)
    pub fn get_process_name(&self, pid: u32) -> Result<String> {
        let req_command = "getprocessname";
        let req_query = format!("{}?pid={}", req_command, &pid);
        let req_url = format!("{}{}", self.base_url, req_query);

        let response = ureq::get(&req_url)
            .call()
            .with_context(|| format!("Failed to send command '{req_command}'"))?;

        let body = response.into_string()?;

        let lines: Vec<&str> = body.split('\n').collect::<Vec<_>>();

        let raw_status_code = lines.get(0);
        let raw_process_name = lines.get(1);

        match (raw_status_code, raw_process_name) {
            (Some(&"0"), Some(process_name)) => Ok(process_name.to_string()),
            _ => bail!("Could not retrieve process name for pid '{pid}'"),
        }
    }
}
