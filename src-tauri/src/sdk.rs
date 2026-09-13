#![allow(dead_code)]

use crate::memory::Process;
use std::sync::Arc;

#[derive(Clone)]
pub struct Roblox {
    mem: Arc<Process>,
    datamodel: DataModel,
}

impl Roblox {
    pub fn new() -> Result<Self, String> {
        crate::offsets::ensure_loaded()?;
        let process = Process::open_roblox()?;
        crate::offsets::check_version(process.version())?;
        let mem = Arc::new(process);

        let fake = mem
            .read_u64(mem.base() + crate::offsets::fake_datamodel::pointer()?)
            .map_err(|e| format!("failed to read FakeDataModel: {e}"))?;
        if !Process::is_valid_address(fake) {
            return Err("FakeDataModel is null - join a game first".into());
        }
        let real = mem
            .read_u64(fake + crate::offsets::fake_datamodel::real_datamodel()?)
            .map_err(|e| format!("failed to read RealDataModel: {e}"))?;
        if !Process::is_valid_address(real) {
            return Err("RealDataModel is null - join a game first".into());
        }

        let datamodel = DataModel(Instance::new(mem.clone(), real));
        if datamodel.get_class_name().as_deref() != Some("DataModel") {
            return Err("DataModel validation failed - offsets may be outdated".into());
        }

        Ok(Self { mem, datamodel })
    }

    pub fn process(&self) -> Arc<Process> {
        self.mem.clone()
    }

    pub fn datamodel(&self) -> DataModel {
        self.datamodel.clone()
    }

    pub fn datamodel_address(&self) -> u64 {
        self.datamodel.address()
    }
}

#[derive(Clone)]
pub struct Instance {
    mem: Arc<Process>,
    address: u64,
}

impl Instance {
    pub fn new(mem: Arc<Process>, address: u64) -> Self {
        Self { mem, address }
    }

    pub fn address(&self) -> u64 {
        self.address
    }

    pub fn is_valid(&self) -> bool {
        Process::is_valid_address(self.address)
    }

    pub fn get_name(&self) -> Option<String> {
        if !self.is_valid() {
            return None;
        }
        let container = self
            .mem
            .read_u64(self.address + crate::offsets::instance::name_container().ok()?)
            .ok()?;
        if !Process::is_valid_address(container) {
            return None;
        }
        let name = self
            .mem
            .read_std_string(container + crate::offsets::instance::name().ok()?)
            .ok()?;
        if name.is_empty() || name.len() >= 200 {
            return None;
        }
        Some(name)
    }

    pub fn get_class_name(&self) -> Option<String> {
        if !self.is_valid() {
            return None;
        }
        let descriptor = self
            .mem
            .read_u64(self.address + crate::offsets::instance::class_descriptor().ok()?)
            .ok()?;
        if !Process::is_valid_address(descriptor) {
            return None;
        }
        let name_ptr = self
            .mem
            .read_u64(descriptor + crate::offsets::instance::class_name().ok()?)
            .ok()?;
        if !Process::is_valid_address(name_ptr) {
            return None;
        }
        self.mem
            .read_std_string(name_ptr)
            .ok()
            .filter(|s| !s.is_empty())
    }

    pub fn get_parent(&self) -> Option<Instance> {
        if !self.is_valid() {
            return None;
        }
        let parent = self
            .mem
            .read_u64(self.address + crate::offsets::instance::parent().ok()?)
            .ok()?;
        if !Process::is_valid_address(parent) {
            return None;
        }
        Some(Instance::new(self.mem.clone(), parent))
    }

    pub fn get_children(&self) -> Vec<Instance> {
        let mut out = Vec::new();
        if !self.is_valid() {
            return out;
        }
        let Ok(children_start) = crate::offsets::instance::children_start() else {
            return out;
        };
        let Ok(children_end_off) = crate::offsets::instance::children_end() else {
            return out;
        };
        let start = match self.mem.read_u64(self.address + children_start) {
            Ok(v) => v,
            Err(_) => return out,
        };
        if !Process::is_valid_address(start) {
            return out;
        }
        let end = match self.mem.read_u64(start + children_end_off) {
            Ok(v) => v,
            Err(_) => return out,
        };
        let list = match self.mem.read_u64(start) {
            Ok(v) => v,
            Err(_) => return out,
        };
        if !Process::is_valid_address(list) || end <= list {
            return out;
        }
        if end - list > 0x1000000 {
            return out;
        }
        let mut ptr = list;
        while ptr < end {
            if let Ok(child) = self.mem.read_u64(ptr) {
                if Process::is_valid_address(child) {
                    out.push(Instance::new(self.mem.clone(), child));
                }
            }
            ptr = ptr.wrapping_add(16);
            if out.len() > 10_000 {
                break;
            }
        }
        out
    }

    pub fn find_first_child(&self, name: &str) -> Option<Instance> {
        if name.is_empty() {
            return None;
        }
        for child in self.get_children() {
            if child.get_name().as_deref() == Some(name) {
                return Some(child);
            }
        }
        None
    }

    pub fn find(&self, path: &[&str]) -> Option<Instance> {
        let mut current = self.clone();
        for name in path {
            current = current.find_first_child(name)?;
        }
        Some(current)
    }

    pub fn get_data_model(&self) -> Option<DataModel> {
        let mut current = self.clone();
        for _ in 0..64 {
            if current.get_class_name().as_deref() == Some("DataModel") {
                return Some(DataModel(current));
            }
            current = current.get_parent()?;
        }
        None
    }
}

#[derive(Clone)]
pub struct DataModel(pub Instance);

impl DataModel {
    pub fn address(&self) -> u64 {
        self.0.address()
    }

    pub fn get_name(&self) -> Option<String> {
        self.0.get_name()
    }

    pub fn get_class_name(&self) -> Option<String> {
        self.0.get_class_name()
    }

    pub fn get_children(&self) -> Vec<Instance> {
        self.0.get_children()
    }

    pub fn find_first_child(&self, name: &str) -> Option<Instance> {
        self.0.find_first_child(name)
    }

    pub fn get_place_id(&self) -> Result<u64, String> {
        self.0
            .mem
            .read::<u64>(self.0.address() + crate::offsets::datamodel::place_id()?)
    }

    pub fn get_game_id(&self) -> Result<u64, String> {
        self.0
            .mem
            .read::<u64>(self.0.address() + crate::offsets::datamodel::game_id()?)
    }

    pub fn get_creator_id(&self) -> Result<u64, String> {
        self.0
            .mem
            .read::<u64>(self.0.address() + crate::offsets::datamodel::creator_id()?)
    }

    pub fn get_workspace(&self) -> Result<Workspace, String> {
        let addr = self
            .0
            .mem
            .read::<u64>(self.0.address() + crate::offsets::datamodel::workspace()?)?;
        if !Process::is_valid_address(addr) {
            return Err("Workspace is null".into());
        }
        Ok(Workspace(Instance::new(self.0.mem.clone(), addr)))
    }

    pub fn get_script_context(&self) -> Result<ScriptContext, String> {
        let addr = self.0.mem.read::<u64>(
            self.0.address() + crate::offsets::datamodel::script_context()?,
        )?;
        if !Process::is_valid_address(addr) {
            return Err("ScriptContext is null".into());
        }
        Ok(ScriptContext(Instance::new(self.0.mem.clone(), addr)))
    }

    pub fn is_game_loaded(&self) -> Result<bool, String> {
        let raw = self
            .0
            .mem
            .read::<u8>(self.0.address() + crate::offsets::datamodel::game_loaded()?)?;
        Ok(raw != 0)
    }
}

#[derive(Clone)]
pub struct Workspace(pub Instance);

impl Workspace {
    pub fn address(&self) -> u64 {
        self.0.address()
    }

    pub fn get_name(&self) -> Option<String> {
        self.0.get_name()
    }

    pub fn get_children(&self) -> Vec<Instance> {
        self.0.get_children()
    }

    pub fn get_data_model(&self) -> Option<DataModel> {
        self.0.get_data_model()
    }
}

#[derive(Clone)]
pub struct Players(pub Instance);

impl Players {
    pub fn get_players(&self) -> Vec<Player> {
        self.0
            .get_children()
            .into_iter()
            .filter(|c| c.get_class_name().as_deref() == Some("Player"))
            .map(Player)
            .collect()
    }

    pub fn find_player(&self, name: &str) -> Option<Player> {
        self.0.find_first_child(name).map(Player)
    }
}

#[derive(Clone)]
pub struct Player(pub Instance);

impl Player {
    pub fn get_name(&self) -> Option<String> {
        self.0.get_name()
    }

    pub fn get_user_id(&self) -> Result<u64, String> {
        self.0
            .mem
            .read::<u64>(self.0.address() + crate::offsets::player::user_id()?)
    }

    pub fn get_display_name(&self) -> Result<String, String> {
        self.0
            .mem
            .read_std_string(self.0.address() + crate::offsets::player::display_name()?)
    }
}

#[derive(Clone)]
pub struct Humanoid(pub Instance);

impl Humanoid {
    pub fn get_health(&self) -> Result<f32, String> {
        self.0
            .mem
            .read::<f32>(self.0.address() + crate::offsets::humanoid::health()?)
    }

    pub fn get_max_health(&self) -> Result<f32, String> {
        self.0
            .mem
            .read::<f32>(self.0.address() + crate::offsets::humanoid::max_health()?)
    }

    pub fn get_walkspeed(&self) -> Result<f32, String> {
        self.0
            .mem
            .read::<f32>(self.0.address() + crate::offsets::humanoid::walkspeed()?)
    }
}

#[derive(Clone)]
pub struct BasePart(pub Instance);

impl BasePart {
    pub fn get_name(&self) -> Option<String> {
        self.0.get_name()
    }

    pub fn get_transparency(&self) -> Result<f32, String> {
        self.0.mem.read::<f32>(
            self.0.address() + crate::offsets::basepart::transparency()?,
        )
    }
}

#[derive(Clone)]
pub struct ScriptContext(pub Instance);

impl ScriptContext {
    pub fn address(&self) -> u64 {
        self.0.address()
    }
}

#[derive(Clone)]
pub struct LocalScript(pub Instance);

impl LocalScript {
    pub fn address(&self) -> u64 {
        self.0.address()
    }

    pub fn get_name(&self) -> Option<String> {
        self.0.get_name()
    }
}

#[derive(Clone)]
pub struct ModuleScript(pub Instance);

impl ModuleScript {
    pub fn address(&self) -> u64 {
        self.0.address()
    }

    pub fn get_name(&self) -> Option<String> {
        self.0.get_name()
    }
}
