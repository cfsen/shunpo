use crate::hyprland::structs::{Monitor, MonitorDesc, MonitorId, MonitorName, Workspace, WorkspaceId, WorkspaceName};

#[test]
fn test_hyprctl_serde_parses_monitors() {
    let mockup = include_str!("fixtures/hyprctl_monitors.json");

    let monitors = serde_json::from_str::<Vec<Monitor>>(mockup)
        .expect("should deserialize monitor JSON");

    // should have exactly 2 monitors
    assert_eq!(monitors.len(), 2);

    // test first monitor (DP-3)
    let monitor1 = &monitors[0];
    assert_eq!(monitor1.id, MonitorId::from(1));
    assert_eq!(monitor1.name, MonitorName::from("DP-3"));
    assert_eq!(monitor1.description, MonitorDesc::from("Monitor 1 description"));
    assert_eq!(monitor1.make, "Monitor 1 make");
    assert_eq!(monitor1.model, "Monitor 1 model");
    assert_eq!(monitor1.serial, "Monitor 1 serial");
    assert_eq!(monitor1.width, 2560);
    assert_eq!(monitor1.height, 1440);
    assert_eq!(monitor1.physical_width, 600);
    assert_eq!(monitor1.physical_height, 340);
    assert!((monitor1.refresh_rate - 143.998).abs() < 0.001);
    assert_eq!(monitor1.x, 0);
    assert_eq!(monitor1.y, 0);
    assert_eq!(monitor1.active_workspace.id, WorkspaceId::from(2));
    assert_eq!(monitor1.active_workspace.name, "2");
    assert_eq!(monitor1.special_workspace.id, WorkspaceId::from(0));
    assert_eq!(monitor1.special_workspace.name, "");
    assert_eq!(monitor1.reserved, [56, 0, 0, 0]);
    assert!((monitor1.scale - 1.0).abs() < 0.001);
    assert_eq!(monitor1.transform, 0);
    assert!(!monitor1.focused);
    assert!(monitor1.dpms_status);
    assert!(!monitor1.vrr);
    assert_eq!(monitor1.solitary, "55c6e09a5a40");
    assert!(monitor1.solitary_blocked_by.is_none());
    assert!(!monitor1.actively_tearing);
    assert_eq!(monitor1.tearing_blocked_by, vec!["NOT_TORN", "USER", "WINDOW"]);
    assert_eq!(monitor1.direct_scanout_to, "0");
    assert_eq!(monitor1.direct_scanout_blocked_by, vec!["USER"]);
    assert!(!monitor1.disabled);
    assert_eq!(monitor1.current_format, "XRGB8888");
    assert_eq!(monitor1.mirror_of, "none");
    assert_eq!(monitor1.available_modes.len(), 9);
    assert!(monitor1.available_modes.contains(&"2560x1440@144.00Hz".to_string()));

    // test second monitor (DP-2)
    let monitor2 = &monitors[1];
    assert_eq!(monitor2.id, MonitorId::from(0));
    assert_eq!(monitor2.name, MonitorName::from("DP-2"));
    assert_eq!(monitor2.description, MonitorDesc::from("Monitor 2 description"));
    assert_eq!(monitor2.x, 2560);
    assert_eq!(monitor2.y, 0);
    assert_eq!(monitor2.active_workspace.id, WorkspaceId::from(5));
    assert_eq!(monitor2.active_workspace.name, "5");
    assert!(monitor2.focused, "DP-2 should be the focused monitor");
    assert_eq!(monitor2.reserved, [0, 0, 0, 0]);
    assert_eq!(monitor2.solitary, "0");
    assert_eq!(
        monitor2.solitary_blocked_by.as_ref().unwrap(),
        &vec!["WINDOWED", "CANDIDATE"]
    );
    assert_eq!(monitor2.tearing_blocked_by, vec!["NOT_TORN", "USER", "CANDIDATE"]);
    assert_eq!(monitor2.direct_scanout_blocked_by, vec!["USER", "CANDIDATE"]);
    assert_eq!(monitor2.available_modes.len(), 15);
    assert!((monitor2.refresh_rate - 59.951).abs() < 0.001);
}
