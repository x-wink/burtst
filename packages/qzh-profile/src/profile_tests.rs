use super::*;
use crate::key_id::{KeyId, MouseButton};
use serde_json::json;

fn make_profile(rules: Vec<BurstRule>) -> Profile {
    Profile {
        schema_version: CURRENT_SCHEMA_VERSION,
        meta: ProfileMeta {
            name: "t".into(),
            created_at: 0,
            updated_at: 0,
            app_version: "0".into(),
        },
        rules,
        hotkeys: Hotkeys::default(),
        advanced: Advanced::default(),
    }
}

fn rule(id: &str, mode: BurstMode, trigger: KeyId, target: KeyId, interval: u32) -> BurstRule {
    BurstRule {
        id: id.into(),
        enabled: true,
        trigger_key: trigger,
        target_key: target,
        mode,
        stop_key: None,
        interval_ms: interval,
        group: None,
    }
}

fn kbd(vk: u32) -> KeyId {
    KeyId::Keyboard(vk)
}

#[test]
fn validate_accepts_empty_profile() {
    assert!(make_profile(vec![]).validate().is_ok());
}

#[test]
fn validate_accepts_interval_at_lower_bound() {
    let p = make_profile(vec![rule(
        "r",
        BurstMode::Hold,
        kbd(0x41),
        kbd(0x42),
        MIN_INTERVAL_MS,
    )]);
    assert!(p.validate().is_ok());
}

#[test]
fn validate_accepts_interval_at_upper_bound() {
    let p = make_profile(vec![rule(
        "r",
        BurstMode::Hold,
        kbd(0x41),
        kbd(0x42),
        MAX_INTERVAL_MS,
    )]);
    assert!(p.validate().is_ok());
}

#[test]
fn validate_rejects_interval_below_minimum() {
    let p = make_profile(vec![rule("r", BurstMode::Hold, kbd(0x41), kbd(0x42), 0)]);
    assert!(matches!(
        p.validate(),
        Err(ProfileError::InvalidInterval(0))
    ));
}

#[test]
fn validate_rejects_interval_above_maximum() {
    let p = make_profile(vec![rule(
        "r",
        BurstMode::Hold,
        kbd(0x41),
        kbd(0x42),
        MAX_INTERVAL_MS + 1,
    )]);
    assert!(matches!(
        p.validate(),
        Err(ProfileError::InvalidInterval(i)) if i == MAX_INTERVAL_MS + 1
    ));
}

#[test]
fn validate_rejects_too_many_rules() {
    let rules = (0..=MAX_RULES)
        .map(|i| rule(&format!("r{i}"), BurstMode::Hold, kbd(0x41), kbd(0x42), 10))
        .collect();
    assert!(matches!(
        make_profile(rules).validate(),
        Err(ProfileError::TooManyRules)
    ));
}

#[test]
fn validate_accepts_max_rules() {
    let rules = (0..MAX_RULES)
        .map(|i| rule(&format!("r{i}"), BurstMode::Hold, kbd(0x41), kbd(0x42), 10))
        .collect();
    assert!(make_profile(rules).validate().is_ok());
}

#[test]
fn validate_default_mode_allows_toggle_target_equals_trigger() {
    // 非 DD 模式（distinct_target = false）允许 toggle target == trigger
    let p = make_profile(vec![rule("t", BurstMode::Toggle, kbd(0x46), kbd(0x46), 10)]);
    assert!(p.validate().is_ok());
}

#[test]
fn burst_rule_serializes_keyid_shape() {
    let r = rule(
        "x",
        BurstMode::Hold,
        kbd(0x51),
        KeyId::Mouse(MouseButton::Left),
        10,
    );
    let v = serde_json::to_value(&r).unwrap();
    assert_eq!(v["trigger_key"], json!({"kind":"keyboard","code":81}));
    assert_eq!(v["target_key"], json!({"kind":"mouse","code":"left"}));
}
