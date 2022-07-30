#![forbid(unsafe_code)]

mod errors;

use anyhow::{anyhow, bail, ensure, Error, Result};
use errors::ConsoleError;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

const CCAPI_OK: u32 = 0;
const DEFAULT_CCAPI_PORT: u16 = 6333;
const DEFAULT_RADIX: u32 = 16;

pub struct CCAPI {
    console_socket: SocketAddr,
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
            _ => bail!("invalid buzzer type '{s}' provided"),
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
            _ => bail!("invalid notify icon '{s}' provided"),
        }
    }
}

#[derive(Debug)]
pub enum ConsoleLed {
    Red,
    Green,
}

impl FromStr for ConsoleLed {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(ConsoleLed::Red),
            "green" => Ok(ConsoleLed::Green),
            _ => bail!("invalid LED color '{s}' provided"),
        }
    }
}

impl ConsoleLed {
    pub fn get_value(&self) -> i32 {
        match *self {
            ConsoleLed::Green => 1,
            ConsoleLed::Red => 2,
        }
    }
}

#[derive(Debug)]
pub enum LedStatus {
    Off,
    On,
    Blink,
}

impl FromStr for LedStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(LedStatus::On),
            "off" => Ok(LedStatus::Off),
            "blink" => Ok(LedStatus::Blink),
            _ => bail!("invalid LED status '{s}' provided"),
        }
    }
}

impl LedStatus {
    pub fn get_value(&self) -> i32 {
        match *self {
            LedStatus::Off => 0,
            LedStatus::On => 1,
            LedStatus::Blink => 2,
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

struct ConsoleRequest<'a> {
    socket: &'a SocketAddr,
    command: String,
    parameters: HashMap<String, String>,
}

struct ConsoleResponse {
    lines: Vec<String>,
}

impl<'a> ConsoleRequest<'a> {
    fn new(socket: &'a SocketAddr, command: &str) -> Self {
        ConsoleRequest {
            socket,
            command: command.to_string(),
            parameters: HashMap::new(),
        }
    }

    fn param(mut self, name: &str, value: &str) -> Self {
        self.parameters.insert(name.to_string(), value.to_string());
        self
    }

    fn send(&self) -> Result<ConsoleResponse> {
        let url = format!("http://{}/ccapi/{}", self.socket, self.command);
        let mut request = ureq::get(&url);

        for param in &self.parameters {
            request = request.query(&param.0, &param.1);
        }

        let response = request.call()?;

        let body = response.into_string()?;
        let lines: Vec<String> = body.split('\n').map(String::from).collect();

        let raw_status_code = lines.get(0).ok_or(anyhow!("Could not read status code"))?;
        let status_code = u32::from_str_radix(&raw_status_code, DEFAULT_RADIX)?;

        ensure!(
            status_code == CCAPI_OK,
            Error::new(ConsoleError::from(status_code)).context(format!(
                "invalid response code '{:#4x}', parameters: {:?}",
                status_code, self.parameters
            ))
        );

        Ok(ConsoleResponse { lines })
    }
}

impl CCAPI {
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
        let console_socket = SocketAddr::new(IpAddr::V4(console_ip), DEFAULT_CCAPI_PORT);

        CCAPI { console_socket }
    }

    /// Sets the IPv4 address of the console to communicate with
    pub fn set_console_ip(&mut self, console_ip: Ipv4Addr) {
        self.console_socket.set_ip(IpAddr::V4(console_ip));
    }

    /// Sets the port to communicate with
    pub fn set_console_port(&mut self, port: u16) {
        self.console_socket.set_port(port);
    }

    /// Rings the console buzzer with the specified [BuzzerType](crate::BuzzerType)
    ///
    /// ### Arguments
    ///
    /// * `buzzer_type` - The buzzer type to use
    pub fn ring_buzzer(&self, buzzer_type: BuzzerType) -> Result<()> {
        let buzzer_code = buzzer_type.get_value();

        ConsoleRequest::new(&self.console_socket, "ringbuzzer")
            .param("type", &buzzer_code.to_string())
            .send()?;

        Ok(())
    }

    /// **WARNING:** This function will return an error even if successful
    ///
    /// Shutdown/restart the console, depending on the [ShutdownMode](crate::ShutdownMode) given
    ///
    /// ### Arguments
    ///
    /// * `shutdown_mode` - The shutdown mode to use
    pub fn shutdown(&self, shutdown_mode: ShutdownMode) -> Result<()> {
        let shutdown_code = shutdown_mode.get_value();

        // FIXME: Explicitly ignore transport error for shutdown call
        let _ = ConsoleRequest::new(&self.console_socket, "shutdown")
            .param("mode", &shutdown_code.to_string())
            .send()?;

        Ok(())
    }

    /// Displays a notification message with an icon
    ///
    /// ### Arguments
    ///
    /// * `notify_icon` - Icon to display
    /// * `message` - Message to display
    pub fn notify(&self, notify_icon: NotifyIcon, message: &str) -> Result<()> {
        let notify_code = notify_icon.get_value();

        ConsoleRequest::new(&self.console_socket, "notify")
            .param("id", &notify_code.to_string())
            .param("msg", message)
            .send()?;

        Ok(())
    }

    /// Sets console LED color and status
    pub fn set_console_led(&self, color: ConsoleLed, status: LedStatus) -> Result<()> {
        let led_color_code = color.get_value();
        let led_status_code = status.get_value();

        ConsoleRequest::new(&self.console_socket, "setconsoleled")
            .param("color", &led_color_code.to_string())
            .param("status", &led_status_code.to_string())
            .send()?;

        Ok(())
    }

    /// Returns console firmware information
    pub fn get_firmware_info(&self) -> Result<FirmwareInfo> {
        let response = ConsoleRequest::new(&self.console_socket, "getfirmwareinfo").send()?;

        let raw_firmware_version = response.lines.get(1);
        let raw_ccapi_version = response.lines.get(2);
        let raw_console_type = response.lines.get(3);

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
        let response = ConsoleRequest::new(&self.console_socket, "gettemperature").send()?;

        let raw_cell_temp = response.lines.get(1);
        let raw_rsx_temp = response.lines.get(2);

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
        let response = ConsoleRequest::new(&self.console_socket, "getprocesslist").send()?;

        let mut process_ids = Vec::new();

        // Skip first line which contains the "status" code
        for raw_pid in &response.lines[1..] {
            if let Ok(pid) = u32::from_str(raw_pid) {
                process_ids.push(pid);
            }
        }

        Ok(process_ids)
    }

    /// Returns a process name from its identifier (pid)
    pub fn get_process_name(&self, pid: &u32) -> Result<String> {
        let response = ConsoleRequest::new(&self.console_socket, "getprocessname")
            .param("pid", &pid.to_string())
            .send()?;

        let raw_process_name = response.lines.get(1);

        match raw_process_name {
            Some(process_name) => Ok(process_name.to_string()),
            _ => bail!("Could not retrieve process name for pid '{pid}'"),
        }
    }

    /// Returns a map of process ids and their names
    pub fn get_process_map(&self) -> Result<HashMap<u32, String>> {
        let pids = self.get_process_list()?;
        let mut process_map = HashMap::new();

        for pid in pids {
            let process_name = self.get_process_name(&pid)?;
            process_map.insert(pid, process_name);
        }

        Ok(process_map)
    }

    /// **!! NOT IMPLEMENTED !!**
    ///
    /// Read process memory from the given address
    pub fn read_process_memory(&self, pid: &u32, address: &u64, size: &u32) -> Result<Vec<u8>> {
        let _response = ConsoleRequest::new(&self.console_socket, "getmemory")
            .param("pid", &pid.to_string())
            .param("addr", &format!("{address:#4x}"))
            .param("size", &size.to_string());

        unimplemented!("read_process_memory is not implemented")
    }
}
