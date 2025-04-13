use chrono::Timelike;
use listagram::utils::progs::current_time;

#[test]
fn summertime() {
    let result1 = current_time(Some(10800));
    let result2 = current_time(None);
    assert_eq!(result1.hour(), result2.hour());
}