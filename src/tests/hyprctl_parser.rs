use crate::hyprland::structs::{Client, Monitor, MonitorDesc, MonitorId, MonitorName, WindowAddr, WindowClass, WindowTitle, Workspace, WorkspaceId, WorkspaceName};

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

#[test]
fn test_hyprctl_serde_parses_workspaces() {
    let mockup = include_str!("fixtures/hyprctl_workspaces.json");

    let workspaces = serde_json::from_str::<Vec<Workspace>>(mockup)
        .expect("should deserialize workspace JSON");

    // should have exactly 7 workspaces
    assert_eq!(workspaces.len(), 7);

    // test special workspace (magic)
    let special = &workspaces[0];
    assert_eq!(special.id, WorkspaceId::from(-98));
    assert_eq!(special.name, WorkspaceName::from("special:magic"));
    assert_eq!(special.monitor, MonitorName::from("DP-3"));
    assert_eq!(special.monitor_id, MonitorId::from(1));
    assert_eq!(special.windows, 1);
    assert!(!special.has_fullscreen);
    assert_eq!(special.last_window, "0x0123456789ab0");
    assert_eq!(special.last_window_title, "Special workspace");
    assert!(!special.is_persistent);

    // test workspace 4
    let ws4 = &workspaces[1];
    assert_eq!(ws4.id, WorkspaceId::from(4));
    assert_eq!(ws4.name, WorkspaceName::from("WorkspaceId=4"));
    assert_eq!(ws4.monitor, MonitorName::from("DP-3"));
    assert_eq!(ws4.monitor_id, MonitorId::from(1));
    assert_eq!(ws4.windows, 1);
    assert!(!ws4.has_fullscreen);
    assert_eq!(ws4.last_window, "0x0123456789ab1");
    assert_eq!(ws4.last_window_title, "App on WorkspaceId=4");

    // test workspace 1
    let ws1 = &workspaces[2];
    assert_eq!(ws1.id, WorkspaceId::from(1));
    assert_eq!(ws1.name, WorkspaceName::from("WorkspaceId=1"));
    assert_eq!(ws1.monitor, MonitorName::from("DP-3"));
    assert_eq!(ws1.monitor_id, MonitorId::from(1));

    // test workspace 5 (on DP-2, multiple windows)
    let ws5 = &workspaces[3];
    assert_eq!(ws5.id, WorkspaceId::from(5));
    assert_eq!(ws5.name, WorkspaceName::from("WorkspaceId=5"));
    assert_eq!(ws5.monitor, MonitorName::from("DP-2"));
    assert_eq!(ws5.monitor_id, MonitorId::from(0));
    assert_eq!(ws5.windows, 3, "workspace 5 should have 3 windows");
    assert_eq!(ws5.last_window, "0x0123456789ab3");

    // test workspace 3
    let ws3 = &workspaces[4];
    assert_eq!(ws3.id, WorkspaceId::from(3));
    assert_eq!(ws3.name, WorkspaceName::from("3"));
    assert_eq!(ws3.monitor, MonitorName::from("DP-3"));
    assert_eq!(ws3.windows, 1);

    // test workspace 2 (has fullscreen)
    let ws2 = &workspaces[5];
    assert_eq!(ws2.id, WorkspaceId::from(2));
    assert_eq!(ws2.name, WorkspaceName::from("2"));
    assert_eq!(ws2.monitor, MonitorName::from("DP-3"));
    assert_eq!(ws2.monitor_id, MonitorId::from(1));
    assert_eq!(ws2.windows, 3, "workspace 2 should have 3 windows");
    assert!(ws2.has_fullscreen, "workspace 2 should have fullscreen window");
    assert_eq!(ws2.last_window, "0x0123456789ab5");
    assert_eq!(ws2.last_window_title, "App on WorkspaceId=2");

    // test workspace 6 (on DP-2)
    let ws6 = &workspaces[6];
    assert_eq!(ws6.id, WorkspaceId::from(6));
    assert_eq!(ws6.name, WorkspaceName::from("6"));
    assert_eq!(ws6.monitor, MonitorName::from("DP-2"));
    assert_eq!(ws6.monitor_id, MonitorId::from(0));
    assert_eq!(ws6.windows, 1);
    assert!(!ws6.has_fullscreen);
    assert_eq!(ws6.last_window, "0x0123456789ab6");

    // verify all workspaces are not persistent
    assert!(workspaces.iter().all(|ws| !ws.is_persistent));
}

#[test]
fn test_hyprctl_serde_parses_clients() {
    let mockup = include_str!("fixtures/hyprctl_clients.json");

    let clients = serde_json::from_str::<Vec<Client>>(mockup)
        .expect("should deserialize clients JSON");

    // should have exactly 11 clients
    assert_eq!(clients.len(), 11);

    // test first client (workspace 2)
    let client0 = &clients[0];
    assert_eq!(client0.address, WindowAddr::from("0x0123456789a0"));
    assert!(client0.mapped);
    assert!(!client0.hidden);
    assert_eq!(client0.at, [56, 0]);
    assert_eq!(client0.size, [1252, 720]);
    assert_eq!(client0.workspace.id, WorkspaceId::from(2));
    assert_eq!(client0.workspace.name, "WorkspaceId=2");
    assert!(!client0.floating);
    assert!(!client0.pseudo);
    assert_eq!(client0.monitor, Some(1));
    assert_eq!(client0.class, WindowClass::from("test.client.class0"));
    assert_eq!(client0.title, WindowTitle::from("Test Client Title 0"));
    assert_eq!(client0.initial_class, "test.client.initialclass0");
    assert_eq!(client0.initial_title, "Test Client Initial Title 0");
    assert_eq!(client0.pid, 10000);
    assert!(!client0.xwayland);
    assert!(!client0.pinned);
    assert_eq!(client0.fullscreen, 0);
    assert_eq!(client0.fullscreen_client, 0);
    assert!(client0.grouped.is_empty());
    assert!(client0.tags.is_empty());
    assert_eq!(client0.swallowing, "0x0");
    assert_eq!(client0.focus_history_id, 3);
    assert!(!client0.inhibiting_idle);
    assert_eq!(client0.xdg_tag, "");
    assert_eq!(client0.xdg_description, "");

    // test special workspace client
    let client1 = &clients[1];
    assert_eq!(client1.address, WindowAddr::from("0x0123456789a1"));
    assert_eq!(client1.workspace.id, WorkspaceId::from(-98));
    assert_eq!(client1.workspace.name, "special:magic");
    assert_eq!(client1.size, [2504, 1440]);
    assert_eq!(client1.class, WindowClass::from("test.client.class1"));
    assert_eq!(client1.focus_history_id, 10);

    // test client on monitor 0 (DP-2)
    let client2 = &clients[2];
    assert_eq!(client2.address, WindowAddr::from("0x0123456789a2"));
    assert_eq!(client2.at, [2560, 0]);
    assert_eq!(client2.size, [2560, 1440]);
    assert_eq!(client2.workspace.id, WorkspaceId::from(6));
    assert_eq!(client2.monitor, Some(0));
    assert_eq!(client2.focus_history_id, 9);

    // test xwayland client
    let client3 = &clients[3];
    assert_eq!(client3.address, WindowAddr::from("0x0123456789a3"));
    assert!(client3.xwayland, "client 3 should be an xwayland window");
    assert_eq!(client3.workspace.id, WorkspaceId::from(4));
    assert_eq!(client3.class, WindowClass::from("test.client.class3"));

    // test workspace 5 clients (multiple on same workspace)
    let client4 = &clients[4];
    assert_eq!(client4.workspace.id, WorkspaceId::from(5));
    assert_eq!(client4.at, [2560, 0]);
    assert_eq!(client4.size, [1280, 720]);

    let client5 = &clients[5];
    assert_eq!(client5.workspace.id, WorkspaceId::from(5));
    assert_eq!(client5.at, [3840, 0]);
    assert_eq!(client5.size, [1280, 1440]);

    // test workspace 1 client
    let client6 = &clients[6];
    assert_eq!(client6.address, WindowAddr::from("0x0123456789a6"));
    assert_eq!(client6.workspace.id, WorkspaceId::from(1));
    assert_eq!(client6.focus_history_id, 2);

    // test another xwayland client
    let client7 = &clients[7];
    assert_eq!(client7.workspace.id, WorkspaceId::from(3));
    assert!(client7.xwayland, "client 7 should be an xwayland window");

    // test workspace 2 clients (has 3 total)
    let client8 = &clients[8];
    assert_eq!(client8.workspace.id, WorkspaceId::from(2));
    assert_eq!(client8.at, [1308, 0]);
    assert_eq!(client8.size, [1252, 720]);

    let client9 = &clients[9];
    assert_eq!(client9.workspace.id, WorkspaceId::from(2));
    assert_eq!(client9.at, [56, 720]);
    assert_eq!(client9.size, [2504, 720]);
    assert_eq!(client9.focus_history_id, 1);

    // test last client (lowest focus history)
    let client10 = &clients[10];
    assert_eq!(client10.address, WindowAddr::from("0x0123456789b0"));
    assert_eq!(client10.workspace.id, WorkspaceId::from(5));
    assert_eq!(client10.focus_history_id, 0, "should have lowest focus history ID");

    // verify no clients are floating, pinned, or fullscreen
    assert!(clients.iter().all(|c| !c.floating));
    assert!(clients.iter().all(|c| !c.pinned));
    assert!(clients.iter().all(|c| c.fullscreen == 0));
    assert!(clients.iter().all(|c| c.fullscreen_client == 0));

    // verify xwayland distribution (2 xwayland, 9 native)
    let xwayland_count = clients.iter().filter(|c| c.xwayland).count();
    assert_eq!(xwayland_count, 2, "should have exactly 2 xwayland windows");
}
