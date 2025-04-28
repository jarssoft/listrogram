use chrono::Timelike;
use listagram::utils::progs::current_datetime;
use listagram::utils::progs::TimePolicy;

#[test]
fn summertime() {
    let result1 = current_datetime(&TimePolicy::Timezone(10800));
    let result2 = current_datetime(&TimePolicy::Naive());
    assert_eq!(result1.hour(), result2.hour());
}