use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};

pub const URL: &str = "https://offsets.imtheo.lol";
const OFFSETS_URL: &str = "https://offsets.imtheo.lol/offsetshex.json";
const FFLAGS_URL: &str = "https://offsets.imtheo.lol/fflagshex.json";


#[derive(Debug)]
pub enum Error {
    Network(String),
    HttpStatus(u16),
    BodyTooLarge(usize),
    Json(String),
    MissingField(&'static str),
    InvalidHex { value: String },
    NoOffsets,
    CacheMissing,
    CacheRead(String),
    CacheParse(String),
    CacheEmpty,
    UnknownOffset(String),
    UnknownFlag(String),
    MissingVersion,
    VersionMismatch { running: String, expected: String },
    LockPoisoned,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(detail) => write!(f, "offset request failed: {detail}"),
            Self::HttpStatus(code) => write!(f, "offset server returned HTTP {code}"),
            Self::BodyTooLarge(limit) => {
                write!(f, "offset response exceeded {limit} bytes")
            }
            Self::Json(detail) => write!(f, "offset response was not valid JSON: {detail}"),
            Self::MissingField(field) => {
                write!(
                    f,
                    "offset response is missing {field}; the dump format may have changed"
                )
            }
            Self::InvalidHex { value } => write!(f, "invalid hexadecimal offset value: {value}"),
            Self::NoOffsets => write!(f, "offset response contained no offsets at all"),
            Self::CacheMissing => write!(f, "no cached offsets found on disk"),
            Self::CacheRead(detail) => write!(f, "could not read cached offsets: {detail}"),
            Self::CacheParse(detail) => write!(f, "cached offsets are corrupt: {detail}"),
            Self::CacheEmpty => write!(f, "cached offsets contain no usable entries"),
            Self::UnknownOffset(path) => {
                write!(f, "unknown offset '{path}' (source: {URL})")
            }
            Self::UnknownFlag(name) => write!(f, "unknown fast flag '{name}'"),
            Self::MissingVersion => {
                write!(f, "offset dump carries no Roblox version; cannot validate")
            }
            Self::VersionMismatch { running, expected } => write!(
                f,
                "Roblox version mismatch: running {running}, offsets built for {expected}; refresh offsets or wait for an updated dump"
            ),
            Self::LockPoisoned => write!(f, "offset cache lock poisoned"),
        }
    }
}

impl std::error::Error for Error {}

impl From<Error> for String {
    fn from(value: Error) -> String {
        value.to_string()
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
struct Snapshot {
    #[serde(default)]
    version: String,
    #[serde(default)]
    offsets: HashMap<String, u64>,
    #[serde(default)]
    fflags: HashMap<String, u64>,
}

impl Snapshot {
    fn offset(&self, path: &str) -> Result<u64> {
        self.offsets
            .get(&normalize_key(path))
            .copied()
            .ok_or_else(|| Error::UnknownOffset(path.to_string()))
    }

    fn flag(&self, name: &str) -> Result<u64> {
        let key = normalize_key(name);
        if let Some(value) = self.fflags.get(&key) {
            return Ok(*value);
        }
        let short = key.strip_prefix("fflags.").unwrap_or(&key);
        self.fflags
            .get(short)
            .copied()
            .ok_or_else(|| Error::UnknownFlag(name.to_string()))
    }

    fn offset_paths(&self) -> Vec<String> {
        let mut paths: Vec<String> = self.offsets.keys().cloned().collect();
        paths.sort();
        paths
    }
}

static STATE: OnceLock<RwLock<Snapshot>> = OnceLock::new();

fn state() -> &'static RwLock<Snapshot> {
    STATE.get_or_init(|| RwLock::new(Snapshot::default()))
}

fn normalize_key(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn parse_hex_value(raw: &str) -> Result<u64> {
    let trimmed = raw.trim();
    let digits = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);
    if digits.is_empty() {
        return Err(Error::InvalidHex {
            value: raw.to_string(),
        });
    }
    u64::from_str_radix(digits, 16).map_err(|_| Error::InvalidHex {
        value: raw.to_string(),
    })
}

fn cache_path() -> Option<PathBuf> {
    let base = std::env::var("APPDATA").ok()?;
    Some(
        PathBuf::from(base)
            .join("Rustine")
            .join("cache.json"),
    )
}

fn fetch_doc(url: &str) -> Result<serde_json::Value> {
    let body = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| Error::Network(e.to_string()))?
        .get(url)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| Error::Network(format!("{url}: {e}")))?
        .error_for_status()
        .map_err(|e| {
            e.status()
                .map(|s| Error::HttpStatus(s.as_u16()))
                .unwrap_or_else(|| Error::Network(e.to_string()))
        })?
        .text()
        .map_err(|e| Error::Network(e.to_string()))?;
    serde_json::from_str(&body).map_err(|e| Error::Json(e.to_string()))
}

fn parse_offsets_doc(doc: &serde_json::Value) -> Result<(String, HashMap<String, u64>)> {
    let version = doc
        .get("Roblox Version")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_string();
    let groups = doc
        .get("Offsets")
        .and_then(serde_json::Value::as_object)
        .ok_or(Error::MissingField("Offsets"))?;
    let mut offsets = HashMap::new();
    for (group, fields) in groups {
        let Some(entries) = fields.as_object() else {
            continue;
        };
        for (name, hex) in entries {
            let Some(text) = hex.as_str() else { continue };
            if let Ok(value) = parse_hex_value(text) {
                offsets.insert(normalize_key(&format!("{group}.{name}")), value);
            }
        }
    }
    if offsets.is_empty() {
        return Err(Error::NoOffsets);
    }
    Ok((version, offsets))
}

fn parse_fflags_doc(doc: &serde_json::Value) -> HashMap<String, u64> {
    let mut fflags = HashMap::new();
    let Some(root) = doc.get("FFlagOffsets") else {
        return fflags;
    };
    if let Some(flags) = root.get("FFlags").and_then(serde_json::Value::as_object) {
        for (name, hex) in flags {
            let Some(text) = hex.as_str() else { continue };
            if let Ok(value) = parse_hex_value(text) {
                fflags.insert(normalize_key(name), value);
            }
        }
    }
    if let Some(list) = root.get("FFlagList").and_then(serde_json::Value::as_object) {
        for (name, hex) in list {
            let Some(text) = hex.as_str() else { continue };
            if let Ok(value) = parse_hex_value(text) {
                fflags.insert(normalize_key(&format!("fflaglist.{name}")), value);
            }
        }
    }
    fflags
}

fn fetch_snapshot() -> Result<Snapshot> {
    let offsets_doc = fetch_doc(OFFSETS_URL)?;
    let (version, offsets) = parse_offsets_doc(&offsets_doc)?;
    let fflags = fetch_doc(FFLAGS_URL)
        .ok()
        .map(|doc| parse_fflags_doc(&doc))
        .unwrap_or_default();
    Ok(Snapshot {
        version,
        offsets,
        fflags,
    })
}

fn load_disk_snapshot() -> Result<Snapshot> {
    let path = cache_path().ok_or(Error::CacheMissing)?;
    let raw = std::fs::read_to_string(&path).map_err(|e| Error::CacheRead(e.to_string()))?;
    let disk: Snapshot =
        serde_json::from_str(&raw).map_err(|e| Error::CacheParse(e.to_string()))?;
    if disk.offsets.is_empty() {
        return Err(Error::CacheEmpty);
    }
    let mut snapshot = Snapshot {
        version: disk.version,
        ..Snapshot::default()
    };
    snapshot.offsets.reserve(disk.offsets.len());
    for (key, value) in disk.offsets {
        snapshot.offsets.insert(normalize_key(&key), value);
    }
    snapshot.fflags.reserve(disk.fflags.len());
    for (key, value) in disk.fflags {
        snapshot.fflags.insert(normalize_key(&key), value);
    }
    Ok(snapshot)
}

fn save_disk_snapshot(snapshot: &Snapshot) -> Result<()> {
    let path = cache_path().ok_or(Error::CacheMissing)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| Error::CacheRead(e.to_string()))?;
    }
    let raw = serde_json::to_string(snapshot).map_err(|e| Error::CacheRead(e.to_string()))?;
    std::fs::write(path, raw).map_err(|e| Error::CacheRead(e.to_string()))
}

pub fn ensure_loaded() -> Result<()> {
    let loaded = state()
        .read()
        .map_err(|_| Error::LockPoisoned)?
        .offsets
        .is_empty();
    if !loaded {
        return Ok(());
    }
    match fetch_snapshot() {
        Ok(fresh) => {
            let _ = save_disk_snapshot(&fresh);
            *state().write().map_err(|_| Error::LockPoisoned)? = fresh;
            Ok(())
        }
        Err(fetch_error) => {
            let disk = load_disk_snapshot().map_err(|_| fetch_error)?;
            *state().write().map_err(|_| Error::LockPoisoned)? = disk;
            Ok(())
        }
    }
}

pub fn refresh() -> Result<()> {
    let fresh = fetch_snapshot()?;
    let _ = save_disk_snapshot(&fresh);
    *state().write().map_err(|_| Error::LockPoisoned)? = fresh;
    Ok(())
}

pub fn is_loaded() -> bool {
    state()
        .read()
        .map(|snapshot| !snapshot.offsets.is_empty())
        .unwrap_or(false)
}

pub fn version() -> Option<String> {
    state().read().ok().and_then(|snapshot| {
        if snapshot.offsets.is_empty() || snapshot.version.is_empty() {
            None
        } else {
            Some(snapshot.version.clone())
        }
    })
}

pub fn offset_count() -> usize {
    state().read().map(|s| s.offsets.len()).unwrap_or(0)
}

pub fn fflag_count() -> usize {
    state().read().map(|s| s.fflags.len()).unwrap_or(0)
}

pub fn get(path: &str) -> Result<u64> {
    ensure_loaded()?;
    state()
        .read()
        .map_err(|_| Error::LockPoisoned)?
        .offset(path)
}

pub fn fflag(name: &str) -> Result<u64> {
    ensure_loaded()?;
    state().read().map_err(|_| Error::LockPoisoned)?.flag(name)
}

pub fn list_offsets() -> Result<Vec<String>> {
    ensure_loaded()?;
    Ok(state()
        .read()
        .map_err(|_| Error::LockPoisoned)?
        .offset_paths())
}

pub fn check_version(running: &str) -> Result<()> {
    ensure_loaded()?;
    let expected = state()
        .read()
        .map_err(|_| Error::LockPoisoned)?
        .version
        .clone();
    if expected.is_empty() {
        return Err(Error::MissingVersion);
    }
    if running == expected {
        Ok(())
    } else {
        Err(Error::VersionMismatch {
            running: running.to_string(),
            expected,
        })
    }
}

macro_rules! offset_group {
    ($module:ident, $($name:ident => $path:literal),* $(,)?) => {
        pub mod $module {
            $(
                pub fn $name() -> super::Result<u64> {
                    super::get($path)
                }
            )*
        }
    };
}

offset_group!(fake_datamodel,
    pointer => "FakeDataModel.Pointer",
    real_datamodel => "FakeDataModel.RealDataModel",
);

offset_group!(instance,
    name_container => "Instance.NameContainer",
    name => "Instance.Name",
    class_descriptor => "Instance.ClassDescriptor",
    class_name => "Instance.ClassName",
    parent => "Instance.Parent",
    children_start => "Instance.ChildrenStart",
    children_end => "Instance.ChildrenEnd",
);

offset_group!(datamodel,
    place_id => "DataModel.PlaceId",
    game_id => "DataModel.GameId",
    creator_id => "DataModel.CreatorId",
    game_loaded => "DataModel.GameLoaded",
    job_id => "DataModel.JobId",
    workspace => "DataModel.Workspace",
    script_context => "DataModel.ScriptContext",
    place_version => "DataModel.PlaceVersion",
);

offset_group!(workspace,
    current_camera => "Workspace.CurrentCamera",
);

offset_group!(players,
    local_player => "Player.LocalPlayer",
);

offset_group!(player,
    user_id => "Player.UserId",
    display_name => "Player.DisplayName",
    model_instance => "Player.ModelInstance",
    team => "Player.Team",
);

offset_group!(humanoid,
    health => "Humanoid.Health",
    max_health => "Humanoid.MaxHealth",
    walkspeed => "Humanoid.Walkspeed",
    jump_power => "Humanoid.JumpPower",
    hip_height => "Humanoid.HipHeight",
);

offset_group!(basepart,
    primitive => "BasePart.Primitive",
    transparency => "BasePart.Transparency",
);

offset_group!(localscript,
    bytecode => "LocalScript.ByteCode",
);

offset_group!(modulescript,
    bytecode => "ModuleScript.ByteCode",
);

offset_group!(bytecode,
    pointer => "ByteCode.Pointer",
    size => "ByteCode.Size",
);

pub mod string {
    pub fn length() -> super::Result<u64> {
        super::get("Misc.StringLength")
    }
}

pub mod fflags {
    pub fn get(name: &str) -> super::Result<u64> {
        super::fflag(name)
    }

    pub fn list_pointer() -> super::Result<u64> {
        super::fflag("FFlagList.Pointer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn hex_accepts_both_cases_and_bare_digits() {
        assert_eq!(parse_hex_value("0x8dc2258").unwrap(), 0x8DC2258);
        assert_eq!(parse_hex_value("0X1F8").unwrap(), 0x1F8);
        assert_eq!(parse_hex_value("1f8").unwrap(), 0x1F8);
        assert_eq!(parse_hex_value("  0x10  ").unwrap(), 0x10);
        assert_eq!(parse_hex_value("0x0").unwrap(), 0);
    }

    #[test]
    fn hex_rejects_empty_and_garbage() {
        assert!(matches!(parse_hex_value(""), Err(Error::InvalidHex { .. })));
        assert!(matches!(
            parse_hex_value("   "),
            Err(Error::InvalidHex { .. })
        ));
        assert!(matches!(
            parse_hex_value("0x"),
            Err(Error::InvalidHex { .. })
        ));
        assert!(matches!(
            parse_hex_value("0xZZZ"),
            Err(Error::InvalidHex { .. })
        ));
        assert!(matches!(
            parse_hex_value("not-hex"),
            Err(Error::InvalidHex { .. })
        ));
    }

    #[test]
    fn keys_normalize_case_and_whitespace() {
        assert_eq!(
            normalize_key("FakeDataModel.Pointer"),
            "fakedatamodel.pointer"
        );
        assert_eq!(normalize_key("  DATAMODEL.gameid "), "datamodel.gameid");
    }

    fn fixture_offsets() -> serde_json::Value {
        json!({
            "Roblox Version": "version-test",
            "Offsets": {
                "FakeDataModel": { "Pointer": "0x100", "RealDataModel": "0x1F8" },
                "DataModel": { "GameId": "0x188" }
            }
        })
    }

    #[test]
    fn parses_valid_offsets_doc() {
        let (version, map) = parse_offsets_doc(&fixture_offsets()).unwrap();
        assert_eq!(version, "version-test");
        assert_eq!(map["fakedatamodel.pointer"], 0x100);
        assert_eq!(map["datamodel.gameid"], 0x188);
    }

    #[test]
    fn rejects_doc_without_offsets_table() {
        let doc = json!({ "Roblox Version": "version-test" });
        assert!(matches!(
            parse_offsets_doc(&doc),
            Err(Error::MissingField("Offsets"))
        ));
    }

    #[test]
    fn rejects_doc_with_no_usable_offsets() {
        let doc = json!({ "Offsets": { "FakeDataModel": { "Pointer": "0xZZZ" } } });
        assert!(matches!(parse_offsets_doc(&doc), Err(Error::NoOffsets)));
    }

    #[test]
    fn skips_single_bad_entry_but_keeps_good_ones() {
        let doc = json!({ "Offsets": {
            "DataModel": { "GameId": "0x188", "Broken": "0xZZZ" }
        } });
        let (_, map) = parse_offsets_doc(&doc).unwrap();
        assert_eq!(map.len(), 1);
        assert_eq!(map["datamodel.gameid"], 0x188);
    }

    #[test]
    fn missing_fflags_doc_yields_empty_table() {
        assert!(parse_fflags_doc(&json!({})).is_empty());
        assert!(parse_fflags_doc(&serde_json::Value::Null).is_empty());
    }

    #[test]
    fn parses_fflags_and_list_entries() {
        let doc = json!({ "FFlagOffsets": {
            "FFlags": { "SomeFlag": "0x10" },
            "FFlagList": { "Pointer": "0x20" }
        } });
        let map = parse_fflags_doc(&doc);
        assert_eq!(map["someflag"], 0x10);
        assert_eq!(map["fflaglist.pointer"], 0x20);
    }

    #[test]
    fn snapshot_lookup_is_case_insensitive() {
        let snapshot = Snapshot {
            version: "version-test".into(),
            offsets: HashMap::from([("datamodel.gameid".into(), 0x188)]),
            fflags: HashMap::from([("someflag".into(), 0x10)]),
        };
        assert_eq!(snapshot.offset("DataModel.GameId").unwrap(), 0x188);
        assert_eq!(snapshot.offset("DATAMODEL.GAMEID").unwrap(), 0x188);
        assert_eq!(snapshot.flag("SomeFlag").unwrap(), 0x10);
        assert_eq!(snapshot.flag("fflags.SomeFlag").unwrap(), 0x10);
    }

    #[test]
    fn snapshot_reports_unknown_entries() {
        let snapshot = Snapshot::default();
        assert!(matches!(
            snapshot.offset("Nope.Missing"),
            Err(Error::UnknownOffset(_))
        ));
        assert!(matches!(snapshot.flag("Nope"), Err(Error::UnknownFlag(_))));
    }

    #[test]
    fn snapshot_survives_disk_round_trip() {
        let snapshot = Snapshot {
            version: "version-test".into(),
            offsets: HashMap::from([("a.b".into(), 1)]),
            fflags: HashMap::from([("c".into(), 2)]),
        };
        let raw = serde_json::to_string(&snapshot).unwrap();
        let back: Snapshot = serde_json::from_str(&raw).unwrap();
        assert_eq!(back.version, "version-test");
        assert_eq!(back.offsets["a.b"], 1);
        assert_eq!(back.fflags["c"], 2);
    }

    #[test]
    fn corrupt_cache_is_rejected() {
        let parsed: std::result::Result<Snapshot, _> = serde_json::from_str("{oops");
        assert!(parsed.is_err());
    }

    #[test]
    fn version_mismatch_message_is_professional() {
        let message = Error::VersionMismatch {
            running: "version-a".into(),
            expected: "version-b".into(),
        }
        .to_string();
        assert!(message.contains("version-a"));
        assert!(message.contains("version-b"));
        assert!(!message.contains("theo hasnt"));
        assert!(!message.contains("idk"));
    }
}
