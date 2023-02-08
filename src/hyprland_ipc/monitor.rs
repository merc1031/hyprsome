use serde::{Deserialize, Serialize};

const MONITORS: &str = "monitors";
const DISPATCH: &str = "dispatch";
const FOCUSMONITOR: &str = "focusmonitor";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub width: u64,
    pub height: u64,
    pub x: u64,
    pub y: u64,
    pub active_workspace: ActiveWorkspace,
    pub reserved: [u64; 4],
    pub scale: f64,
    pub transform: u64,
    pub focused: bool,
    pub dpms_status: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveWorkspace {
    pub id: u64,
    pub name: String,
}

impl Monitor {
    pub fn real_width(&self) -> u64 {
        return match self.transform {
            0 | 2 | 4 | 6 => self.width as f64 / self.scale,
            1 | 3 | 5 => self.height as f64 / self.scale,
            _ => self.width as f64,
        } as u64;
    }
    pub fn real_height(&self) -> u64 {
        return match self.transform {
            0 | 2 | 4 | 6 => self.height as f64 / self.scale,
            1 | 3 | 5 => self.width as f64 / self.scale,
            _ => self.height as f64,
        } as u64;
    }
}

pub fn get_by_id(id: u64) -> Monitor {
    let response = super::send_message(MONITORS, vec![]);

    let monitors: Vec<Monitor> = serde_json::from_str(&response).unwrap();
    let monitor = monitors.into_iter().find(|m| m.id == id).unwrap();

    return monitor;
}

pub fn get() -> Vec<Monitor> {
    let response = super::send_message(MONITORS, vec![]);
    let monitors: Vec<Monitor> = serde_json::from_str(&response).unwrap();

    return monitors;
}

pub fn focus_left() {
    let _ = super::send_message(DISPATCH, vec![FOCUSMONITOR, "l"]);
}

pub fn focus_right() {
    let _ = super::send_message(DISPATCH, vec![FOCUSMONITOR, "r"]);
}

pub fn focus_up() {
    let _ = super::send_message(DISPATCH, vec![FOCUSMONITOR, "u"]);
}

pub fn focus_down() {
    let _ = super::send_message(DISPATCH, vec![FOCUSMONITOR, "d"]);
}

pub fn focus_by_id(id: &str) {
    let _ = super::send_message(DISPATCH, vec![FOCUSMONITOR, id]);
}
