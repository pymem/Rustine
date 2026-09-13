use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
    MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

pub struct Process {
    handle: HANDLE,
    pid: u32,
    base: u64,
    version: String,
}

unsafe impl Send for Process {}
unsafe impl Sync for Process {}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

fn find_pid(exe_name: &str) -> Option<u32> {
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
        let mut entry = PROCESSENTRY32::default();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        if Process32First(snap, &mut entry).is_err() {
            let _ = CloseHandle(snap);
            return None;
        }
        loop {
            let raw =
                std::slice::from_raw_parts(entry.szExeFile.as_ptr() as *const u8, 260);
            let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
            let name = String::from_utf8_lossy(&raw[..end]);
            if name.eq_ignore_ascii_case(exe_name) {
                let pid = entry.th32ProcessID;
                let _ = CloseHandle(snap);
                return if pid != 0 { Some(pid) } else { None };
            }
            if Process32Next(snap, &mut entry).is_err() {
                break;
            }
        }
        let _ = CloseHandle(snap);
        None
    }
}

fn module_info(pid: u32, module_name: &str) -> Option<(u64, String)> {
    unsafe {
        let snap =
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid).ok()?;
        let mut entry = MODULEENTRY32::default();
        entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
        if Module32First(snap, &mut entry).is_err() {
            let _ = CloseHandle(snap);
            return None;
        }
        loop {
            let raw =
                std::slice::from_raw_parts(entry.szModule.as_ptr() as *const u8, 256);
            let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
            let name = String::from_utf8_lossy(&raw[..end]);
            if name.eq_ignore_ascii_case(module_name) {
                let base = entry.modBaseAddr as u64;
                let praw =
                    std::slice::from_raw_parts(entry.szExePath.as_ptr() as *const u8, 260);
                let pend = praw.iter().position(|&b| b == 0).unwrap_or(praw.len());
                let path = String::from_utf8_lossy(&praw[..pend]).to_string();
                let _ = CloseHandle(snap);
                return if base != 0 { Some((base, path)) } else { None };
            }
            if Module32Next(snap, &mut entry).is_err() {
                break;
            }
        }
        let _ = CloseHandle(snap);
        None
    }
}

impl Process {
    pub fn open_roblox() -> Result<Self, String> {
        let pid = find_pid("RobloxPlayerBeta.exe")
            .ok_or_else(|| "Roblox not found - launch Roblox and join a game first".to_string())?;
        let handle = unsafe {
            OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, false, pid)
                .map_err(|e| format!("OpenProcess failed: {e} - try running as administrator"))?
        };
        if handle.is_invalid() {
            return Err("invalid process handle".into());
        }

        let (base, path) = module_info(pid, "RobloxPlayerBeta.exe")
            .ok_or_else(|| "cannot find Roblox base - is Roblox running?".to_string())?;
        if base == 0 {
            return Err("invalid base address".into());
        }

        let parts: Vec<&str> = path.split('\\').collect();
        let version = if parts.len() >= 2 {
            parts[parts.len() - 2].to_string()
        } else {
            String::new()
        };
        if version.is_empty() || !version.starts_with("version-") {
            return Err(format!("cannot determine Roblox version from path: {path}"));
        }

        Ok(Self {
            handle,
            pid,
            base,
            version,
        })
    }

    #[allow(dead_code)]
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn base(&self) -> u64 {
        self.base
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn is_valid_address(addr: u64) -> bool {
        addr != 0 && addr > 0x10000
    }

    pub fn read<T: Copy>(&self, addr: u64) -> Result<T, String> {
        if !Self::is_valid_address(addr) {
            return Err(format!("read of invalid address {addr:#x}"));
        }
        if self.handle.is_invalid() {
            return Err("invalid process handle".into());
        }
        let mut out = std::mem::MaybeUninit::<T>::uninit();
        let mut read = 0usize;
        unsafe {
            ReadProcessMemory(
                self.handle,
                addr as *const _,
                out.as_mut_ptr() as *mut _,
                std::mem::size_of::<T>(),
                Some(&mut read),
            )
            .map_err(|e| format!("ReadProcessMemory {addr:#x} failed: {e}"))?;
        }
        if read != std::mem::size_of::<T>() {
            return Err(format!("incomplete read at {addr:#x}"));
        }
        Ok(unsafe { out.assume_init() })
    }

    pub fn read_u64(&self, addr: u64) -> Result<u64, String> {
        self.read::<u64>(addr)
    }

    pub fn read_bytes(&self, addr: u64, len: usize) -> Result<Vec<u8>, String> {
        if !Self::is_valid_address(addr) {
            return Err(format!("read of invalid address {addr:#x}"));
        }
        if len == 0 {
            return Ok(Vec::new());
        }
        if len > 4096 {
            return Err("read bytes length out of range".into());
        }
        let mut buf = vec![0u8; len];
        let mut read = 0usize;
        unsafe {
            ReadProcessMemory(
                self.handle,
                addr as *const _,
                buf.as_mut_ptr() as *mut _,
                len,
                Some(&mut read),
            )
            .map_err(|e| format!("ReadProcessMemory {addr:#x} len {len} failed: {e}"))?;
        }
        if read == 0 {
            return Err("read bytes returned zero bytes".into());
        }
        buf.truncate(read);
        Ok(buf)
    }

    #[allow(dead_code)]
    pub fn read_string(&self, addr: u64) -> Result<String, String> {
        let bytes = self.read_bytes(addr, 256)?;
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        String::from_utf8(bytes[..end].to_vec()).map_err(|e| format!("invalid string: {e}"))
    }

    pub fn read_std_string(&self, addr: u64) -> Result<String, String> {
        if !Self::is_valid_address(addr) {
            return Err(format!("read of invalid address {addr:#x}"));
        }
        let len = self.read::<u64>(addr + crate::offsets::string::length()?)? as usize;
        if len > 512 {
            return Err("std::string length out of range".into());
        }
        if len == 0 {
            return Ok(String::new());
        }
        let data = if len >= 16 {
            self.read::<u64>(addr)?
        } else {
            addr
        };
        let bytes = self.read_bytes(data, len)?;
        String::from_utf8(bytes).map_err(|e| e.to_string())
    }

    pub fn is_alive(&self) -> bool {
        if self.handle.is_invalid() {
            return false;
        }
        unsafe {
            use windows::Win32::System::Threading::GetExitCodeProcess;
            let mut code = 0u32;
            if GetExitCodeProcess(self.handle, &mut code).is_ok() {
                const STILL_ACTIVE: u32 = 259;
                if code != STILL_ACTIVE {
                    return false;
                }
            }
            find_pid("RobloxPlayerBeta.exe").is_some_and(|pid| pid == self.pid)
        }
    }
}
