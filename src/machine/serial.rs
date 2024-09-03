use super::Device;
use std::{ collections::VecDeque, io, io::{ Read, Write }, sync::{ Arc, Mutex, atomic::AtomicBool }, thread };

pub const SERIAL_CONTROL_PIPE1_ENABLE: u8 = 1;
pub const SERIAL_CONTROL_PIPE2_ENABLE: u8 = 2;
pub const SERIAL_STATUS_PIPE1_RX: u8 = 1;
pub const SERIAL_STATUS_PIPE2_RX: u8 = 2;
pub const SERIAL_REG_STATUS: u64 = 1;
pub const SERIAL_REG_CONTROL: u64 = 2;
pub const SERIAL_REG_PIPE1: u64 = 3; // stdin & stdout
pub const SERIAL_REG_PIPE2: u64 = 4; // real serial port

pub struct SerialDevice {
    running: AtomicBool,
    control: Mutex<u8>,
    rx1: Mutex<VecDeque<u8>>,
    tx1: Mutex<VecDeque<u8>>,
    rx2: Mutex<VecDeque<u8>>,
    tx2: Mutex<VecDeque<u8>>
}

impl SerialDevice {
    pub fn new() -> Self {
        SerialDevice {
            running: AtomicBool::new(true),
            control: Mutex::new(0),
            rx1: Mutex::new(VecDeque::with_capacity(64)),
            tx1: Mutex::new(VecDeque::with_capacity(128)),
            rx2: Mutex::new(VecDeque::with_capacity(512)),
            tx2: Mutex::new(VecDeque::with_capacity(512))
        }
    }

    pub fn start(self) -> RunningSerialDevice {
        let serial_device = Arc::new(self);

        RunningSerialDevice {
            _join_handle: {
                let serial_device = serial_device.clone();
                thread::spawn(move || serial_device.run())
            },
            serial_device
        }
    }

    pub fn run(&self) {
        while self.running.load(std::sync::atomic::Ordering::Relaxed) {
            // Run locks in a sub-scope to release them before sleeping
            {
                let control = {
                    let lock = self.control.lock().expect("Control is poisoned");
                    lock.clone()
                };

                if control & SERIAL_CONTROL_PIPE1_ENABLE != 0 {
                    if let Ok(mut tx1) = self.tx1.lock() {
                        if !tx1.is_empty() {
                            io::stdout().write_all(&tx1.drain(..).collect::<Vec<_>>()).expect("Failed to write TX1 to stdout");
                        }
                    }

                    let stdin: Vec<u8> = io::stdin().bytes().map(|b| b.unwrap()).collect();

                    if stdin.len() > 0 {
                        if let Ok(mut rx1) = self.rx1.lock() {
                            rx1.write_all(&stdin).expect("Failed to write stdin to RX1");
                        }
                    }
                }

                if control & SERIAL_CONTROL_PIPE2_ENABLE != 0 {
                    if let Ok(tx2) = self.tx2.lock() {
                        if !tx2.is_empty() {
                            // TODO
                        }
                    }
                }
            }

            thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn status(&self) -> Result<u8, &'static str> {
        let mut status = 0;

        if !self.rx1.lock().or(Err("RX1 is poisoned"))?.is_empty() {
            status |= SERIAL_STATUS_PIPE1_RX;
        }

        if !self.rx2.lock().or(Err("RX2 is poisoned"))?.is_empty() {
            status |= SERIAL_STATUS_PIPE2_RX;
        }

        Ok(status)
    }
}

pub struct RunningSerialDevice {
    _join_handle: thread::JoinHandle<()>,
    serial_device: Arc<SerialDevice>
}

impl Drop for RunningSerialDevice {
    fn drop(&mut self) {
        self.serial_device.running.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}

impl Device for RunningSerialDevice {
    fn read_byte(&self, address: u64) -> Result<u8, &'static str> {
        match address {
            SERIAL_REG_STATUS => self.serial_device.status(),
            SERIAL_REG_PIPE1 => self.serial_device.rx1.lock().or(Err("RX1 is poisoned"))?
                .pop_front().ok_or("No data in RX buffer for pipe 1"),
            SERIAL_REG_PIPE2 => self.serial_device.rx2.lock().or(Err("RX2 is poisoned"))?
                .pop_front().ok_or("No data in RX buffer for pipe 2"),
            _ => Err("Invalid address")
        }
    }

    fn write_byte(&mut self, address: u64, data: u8) -> Result<(), &'static str> {
        match address {
            SERIAL_REG_STATUS => Ok(()),
            SERIAL_REG_CONTROL => {
                *self.serial_device.control.lock().or(Err("Control is poisoned"))? = data;
                Ok(())
            },
            SERIAL_REG_PIPE1 => {
                self.serial_device.tx1.lock().or(Err("TX1 is poisoned"))?.push_back(data);
                Ok(())
            },
            SERIAL_REG_PIPE2 => {
                self.serial_device.tx2.lock().or(Err("TX2 is poisoned"))?.push_back(data);
                Ok(())
            },
            _ => Err("Invalid address")
        }
    }

    fn read_stream(&self, address: u64, stream: &mut [u8]) -> Result<(), &'static str> {
        if address > SERIAL_REG_PIPE2 ||
            stream.len() < SERIAL_REG_PIPE2 as usize ||
            stream.len() > SERIAL_REG_PIPE2 as usize
        {
            Err("Invalid address")
        } else {
            stream.copy_from_slice(&[
                self.serial_device.status()?,
                self.serial_device.rx1.lock().or(Err("RX1 is poisoned"))?.pop_front().unwrap_or(0),
                self.serial_device.rx2.lock().or(Err("RX2 is poisoned"))?.pop_front().unwrap_or(0)
            ]);

            Ok(())
        }
    }

    fn write_stream(&mut self, address: u64, stream: &[u8]) -> Result<(), &'static str> {
        if address > SERIAL_REG_PIPE2 ||
            stream.len() < SERIAL_REG_PIPE2 as usize ||
            stream.len() > SERIAL_REG_PIPE2 as usize
        {
            Err("Invalid address")
        } else {
            self.serial_device.tx1.lock().or(Err("TX1 is poisoned"))?.push_back(stream[1]);
            self.serial_device.tx2.lock().or(Err("TX2 is poisoned"))?.push_back(stream[2]);

            Ok(())
        }
    }
}
