use arch_mirrors_rs::Mirror;

#[test]
fn deserializes_mirror_status_field_names_and_values() -> Result<(), serde_json::Error> {
    let mirror: Mirror = serde_json::from_str(
        r#"{
            "url": "https://mirror.example.com/archlinux/",
            "protocol": "https",
            "last_sync": null,
            "completion_pct": 1.0,
            "delay": -145,
            "duration_avg": 0.42,
            "duration_stddev": 0.1,
            "score": 0.5,
            "active": true,
            "country": "Australia",
            "country_code": "AU",
            "isos": true,
            "ipv4": true,
            "ipv6": true,
            "details": "https://archlinux.org/mirrors/example/"
        }"#,
    )?;

    assert_eq!(mirror.delay, Some(-145));
    assert_eq!(mirror.duration_average, Some(0.42));
    Ok(())
}
