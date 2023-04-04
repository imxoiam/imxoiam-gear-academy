use gtest::{Log, Program, System};
use tamagotchi::{TmgAction, TmgEvent};


#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send_bytes(2, String::from("CHERMANDER"));
    let expected_log = Log::builder().dest(2).payload(String::from("SUCCESS"));
    assert!(res.contains(&expected_log));
    assert!(!res.main_failed());

}

#[test]
fn hello_test_2() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send_bytes(2, String::from("CHERMANDER"));
    let expected_log = Log::builder().dest(2).payload(String::from("SUCCESS"));
    assert!(res.contains(&expected_log));
    let res = program.send(2, TmgAction::Name);
    let expected_log = Log::builder().dest(2)
        .payload(TmgEvent::Name(String::from("CHERMANDER")));
    assert!(res.contains(&expected_log))

}