use chrono::{Local, Timelike};
use listagram::utils::progs::current_time;
use listagram::handlers::TimeFormat;

#[test]
fn summertime() {
    let result1 = current_time(&TimeFormat::Timezone(10800));
    let result2 = current_time(&TimeFormat::Local());
    assert_eq!(result1.hour(), result2.hour());
}