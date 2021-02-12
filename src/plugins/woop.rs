use super::Plugin;
use matrix_sdk::{
    Client, RoomState, async_trait
};

pub struct WoopPlugin { }

#[async_trait]
impl Plugin for WoopPlugin {
    async fn room_message(&self, client: &Client, room: &RoomState, msg_body: &str) {
        if msg_body.starts_with(".woop") {
            let response = match extract_num(msg_body) {
                Some(n) => woop(n),
                None => woop(10.0)
            };
            self.send_message(client, room, &response).await;
        }
    }
}

// extract e.g. 2.5 out of a string like ".woop 2.5"
fn extract_num(woopstr: &str) -> Option<f32> {
    woopstr
        .strip_prefix(".woop")
        .and_then(|n| n.trim().parse().ok())
}

// convert e.g. 2.5 into "WOOP WOOP WO"
fn woop(n: f32) -> String {
    let whole_portion = n.trunc();
    if whole_portion > 100.0 {
        return woop(100.0);
    }
    let fractional_portion = ((n - whole_portion) * 4.0).round();
    let woop = "WOOP ".repeat(whole_portion as usize) + &"WOOP"[..fractional_portion as usize];
    woop.trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn woop_integer() {
        assert_eq!(woop(0.0), "");
        assert_eq!(woop(1.0), "WOOP");
        assert_eq!(woop(2.0), "WOOP WOOP");
    }

    #[test]
    fn woop_fractional() {
        assert_eq!(woop(0.0), "");
        assert_eq!(woop(0.1), "");
        assert_eq!(woop(0.2), "W");
        assert_eq!(woop(0.25),"W");
        assert_eq!(woop(0.3), "W");
        assert_eq!(woop(0.4), "WO");
        assert_eq!(woop(0.5), "WO");
        assert_eq!(woop(0.6), "WO");
        assert_eq!(woop(0.7), "WOO");
        assert_eq!(woop(0.8), "WOO");
        assert_eq!(woop(0.9), "WOOP");
        assert_eq!(woop(1.0), "WOOP");
        assert_eq!(woop(1.25), "WOOP W");
    }

    #[test]
    fn woop_clamped() {
        assert_eq!(woop(10_000.7), woop(100.0));
    }

    #[test]
    fn extract_num_woop() {
        assert_eq!(extract_num(".woop"), None);
        assert_eq!(extract_num(".woop woop"), None);
        assert_eq!(extract_num(".woop 🎉"), None);
        assert_eq!(extract_num(".woop 10 10"), None);

        assert_eq!(extract_num(".woop 10"), Some(10.0));
        assert_eq!(extract_num(".woop 2.5"), Some(2.5));
        assert_eq!(extract_num(".woop 999999999999999999999999999999999"), Some(999999999999999999999999999999999.0));
    }
}