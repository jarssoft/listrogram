use chrono::{Local, Timelike};
use listagram::utils::progs::current_time;
use listagram::utils::progs::TimePolicy;

#[test]
fn summertime() {
    let result1 = current_time(&TimePolicy::Timezone(10800));
    let result2 = current_time(&TimePolicy::Naive());
    assert_eq!(result1.hour(), result2.hour());
}