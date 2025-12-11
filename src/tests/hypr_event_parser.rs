use crate::hyprland::{
    event_parser::HyprlandEvent,
    structs::FullscreenEvent
};

#[test]
fn fullscreen_events_parse() {
    assert!(FullscreenEvent::parse_raw("0").is_ok());
    assert!(FullscreenEvent::parse_raw("1").is_ok());
    assert!(FullscreenEvent::parse_raw("any").is_err());
}

//
// HyprlandEvent
//
macro_rules! test_parser_ok {
    ($name:ident, $input:expr) => {
        #[test]
        fn $name() {
            assert!(HyprlandEvent::parse_event($input).is_ok());
        }
    };
}

macro_rules! test_parser_err {
    ($name:ident, $input:expr) => {
        #[test]
        fn $name() {
            assert!(HyprlandEvent::parse_event($input).is_err());
        }
    };
}
// invalid input
test_parser_err!(hyprlandevent_rejects_invalid_event, "not an event");

// by event:

// test_parser_ok!(hyprlandevent_parses_, ">>");
// test_parser_err!(hyprlandevent_rejects_, ">>");

// workspace
test_parser_ok!(hyprlandevent_parses_workspace, "workspace>>1");
test_parser_err!(hyprlandevent_rejects_workspace_not_int, "workspace>>only accepts number");

// workspacev2
test_parser_ok!(hyprlandevent_parses_workspacev2, "workspacev2>>1,DP-1");
test_parser_err!(hyprlandevent_rejects_workspacev2_bad_order, "workspacev2>>DP-1,1");
test_parser_err!(hyprlandevent_rejects_workspacev2_no_delimiter, "workspacev2>>no delimiter");

// focusedmon
test_parser_ok!(hyprlandevent_parses_focusedmon, "focusedmon>>1,workspace_name");
test_parser_err!(hyprlandevent_rejects_focusedmon_no_delimiter, "focusedmon>>no delimiter");

// fullscreen
test_parser_ok!(hyprlandevent_parses_fullscreen_exit, "fullscreen>>0");
test_parser_ok!(hyprlandevent_parses_fullscreen_enter, "fullscreen>>1");
test_parser_err!(hyprlandevent_rejects_fullscreen_not_int, "fullscreen>>only accepts 0 or 1");
test_parser_err!(hyprlandevent_rejects_fullscreen_int_out_of_range, "fullscreen>>10");

