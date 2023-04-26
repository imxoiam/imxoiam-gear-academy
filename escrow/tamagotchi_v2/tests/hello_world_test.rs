use gtest::{Log, Program, System};
use tamagotchi_io::{TmgAction, TmgEvent};


#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send_bytes(2, String::from("CHERMANDER"));
    assert!(!res.main_failed());

}

#[test]
fn hello_test_2() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send_bytes(2, String::from("CHERMANDER"));
    assert!(!res.main_failed());
    let res = program.send(2, TmgAction::Name);
    let expected_log = Log::builder().dest(2)
        .payload(TmgEvent::Name(String::from("CHERMANDER")));
    assert!(res.contains(&expected_log));
    let res = program.send(2, TmgAction::Age);
    let expected_log = Log::builder().dest(2)
        .payload(TmgEvent::Age(0));
    assert!(res.contains(&expected_log));
    let res = program.send(2, TmgAction::Feed);
    let expected_log = Log::builder().dest(2)
        .payload(TmgEvent::Fed);
    assert!(res.contains(&expected_log));
    let res = program.send(2, TmgAction::Play);
    let expected_log = Log::builder().dest(2)
        .payload(TmgEvent::Entertained);
    assert!(res.contains(&expected_log));
    let res = program.send(2, TmgAction::Sleep);
    let expected_log = Log::builder().dest(2)
        .payload(TmgEvent::Slept);
    assert!(res.contains(&expected_log));
}