use patterns::{
    accumulated, after_prefix, at_or_before, axis_ceiling, bar_height, booked, camel_of, civil_day, count_of, count_text,
    csv_cell, csv_text, day_from_iso, day_of, day_start, day_text, days_between, dedup_sorted, each_with, earliest, far_future,
    fixed, grouped, grouped_places, hex_color, highest_of, highest_value, interleaved, iso_day_text, joined_texts, keep_in,
    latest, latest_time, looked_up, lowest_of, merged, month_start, negated, negative_count, percent_of, percent_text,
    picked_names, pie_slices, places_of, point_on, points_of_lines, priced, ratio_of, recorded, restored, rgba_text, rounded,
    series_days, signed_of, sum_seconds, summed_signed, unit_fraction, unsigned_count, unsigned_of, usd_text, whole_tokens,
    wide_summed, widened_usize, window_from, DayTotal, Days, Field, Record, Totals,
};
use std::collections::BTreeMap;

const DAY: u64 = 86_400;
const AUGUST_TWENTY_FIRST: u64 = 1_787_270_400;

#[test]
fn days_are_floored_counted_and_named() {
    assert_eq!(day_of(90_000, DAY), DAY);
    assert_eq!(days_between(0, 3 * DAY, DAY), 3);
    assert_eq!(civil_day(AUGUST_TWENTY_FIRST, DAY), (2026, 8, 21));
    assert_eq!(day_start(2026, 8, 21, DAY), AUGUST_TWENTY_FIRST);
    assert_eq!(month_start(2026, 8, DAY), day_start(2026, 8, 1, DAY));
    assert_eq!(day_text(AUGUST_TWENTY_FIRST), "2026/08/21");
    assert_eq!(iso_day_text(AUGUST_TWENTY_FIRST), "2026-08-21");
    assert_eq!(day_from_iso("2026-08-21"), Some(AUGUST_TWENTY_FIRST));
    assert_eq!(day_from_iso("yesterday"), None);
}

#[test]
fn figures_read_the_way_a_reader_expects() {
    assert_eq!(usd_text(2452.0, 2), "$ 2,452.00");
    assert_eq!(usd_text(0.05579, 4), "$ 0.0558");
    assert_eq!(grouped_places(5_722_162.0571, 2, 2), "5,722,162.06");
    assert_eq!(count_text(926_622), "926,622");
    assert_eq!(percent_text(46.26, 2), "46.26%");
    assert_eq!(fixed(3.14159, 2), "3.14");
    assert_eq!(grouped(1_234_567), "1,234,567");
    assert_eq!(hex_color(0xff713a), "#ff713a");
    assert_eq!(rgba_text(0xff713a, 0.5), "rgba(255, 113, 58, 0.5)");
}

#[test]
fn csv_quotes_only_what_needs_quoting() {
    assert_eq!(csv_cell("plain"), "plain");
    assert_eq!(csv_cell("a,b"), "\"a,b\"");
    assert_eq!(csv_cell("say \"hi\""), "\"say \"\"hi\"\"\"");
    let text = csv_text(&[String::from("day"), String::from("value")], &[vec![String::from("2026/08/21"), String::from("1,5")]]);
    assert_eq!(text, "day,value\n2026/08/21,\"1,5\"");
}

#[test]
fn money_truncates_rounds_and_shares() {
    assert_eq!(whole_tokens(1_500_000_000_000_000_000, 1_000_000_000_000, 1_000_000.0), 1.5);
    assert_eq!(rounded(2.345, 2), 2.35);
    assert_eq!(rounded(2.344, 2), 2.34);
    assert_eq!(percent_of(1.0, 4.0, 100.0), Some(25.0));
    assert_eq!(percent_of(1.0, 0.0, 100.0), None);
    assert_eq!(ratio_of(1.0, 2.0), Some(0.5));
    assert_eq!(priced(2.0, 3.0), 6.0);
}

#[derive(Record, Clone, Debug, PartialEq)]
struct Sample {
    day_timestamp_unix_sec: u64,
    label: String,
    price_usd: Option<f64>,
    hash: [u8; 4],
}

#[test]
fn a_record_round_trips_through_its_fields() {
    let sample = Sample { day_timestamp_unix_sec: AUGUST_TWENTY_FIRST, label: String::from("FLUX"), price_usd: None, hash: [10, 11, 12, 13] };
    let field = recorded(&sample);
    match &field {
        Field::Table(rows) => {
            assert_eq!(rows[0].0, "day_timestamp_unix_sec");
            assert_eq!(rows[2].1, Field::Nothing);
            assert_eq!(rows[3].1, Field::Text(String::from("0x0a0b0c0d")));
        }
        other => panic!("a struct records as a table, not {:?}", other),
    }
    assert_eq!(restored::<Sample>(&field), Some(sample));
    assert_eq!(camel_of("day_timestamp_unix_sec"), "dayTimestampUnixSec");
}

fn plus(a: &u64, with: &u64) -> u64 {
    a + with
}

fn same(item: &u64) -> u64 {
    *item
}

fn is_odd(item: &u64) -> bool {
    item % 2 == 1
}

#[test]
fn lists_are_shaped_without_closures() {
    assert_eq!(dedup_sorted(&[3, 1, 3, 2]), vec![1, 2, 3]);
    assert_eq!(after_prefix("/addresses/0xabc", "/addresses/"), Some(String::from("0xabc")));
    assert_eq!(after_prefix("/other", "/addresses/"), None);
    assert_eq!(joined_texts(&[String::from("a"), String::from("b")]), "ab");
    assert_eq!(interleaved(&[1, 2, 3], &0), vec![1, 0, 2, 0, 3]);
    assert_eq!(accumulated(&[vec![1, 2], vec![2, 3]], same), vec![1, 2, 3]);
    assert_eq!(highest_value(&[1.0, 3.0, 2.0]), 3.0);
    assert_eq!(places_of(4), 4);
    assert_eq!(each_with(&[1, 2], &10, plus), vec![11, 12]);
    assert_eq!(keep_in(&[1, 2, 3, 4], is_odd, &[true]), vec![1, 3]);
    assert_eq!(looked_up(&[1, 9], &[(1, "one")], "none"), vec!["one", "none"]);
    assert_eq!(picked_names(&[true, false], "yes", "no"), vec!["yes", "no"]);
    assert_eq!(sum_seconds(&[((), 1), ((), 2)]), 3);
    assert_eq!(widened_usize(3), 3);
    assert_eq!(far_future(), u64::MAX);
}

#[test]
fn tables_answer_by_key_order() {
    let mut map = BTreeMap::new();
    map.insert(10u64, "ten");
    map.insert(20u64, "twenty");
    assert_eq!(latest(&map), Some((20, "twenty")));
    assert_eq!(earliest(&map), Some((10, "ten")));
    assert_eq!(at_or_before(&map, 15), Some("ten"));
    assert_eq!(at_or_before(&map, 5), None);
    let mut more = BTreeMap::new();
    more.insert(30u64, "thirty");
    assert_eq!(merged(map, more).len(), 3);
}

#[test]
fn a_series_books_its_days_and_totals() {
    let (days, totals): (Days<char>, Totals<char>) = booked(Days::new(), Totals::new(), 'a', DAY, 5, 1);
    let (days, totals) = booked(days, totals, 'a', DAY, 2, 1);
    assert_eq!(patterns::amount_of(&totals, &'a'), 7);
    assert_eq!(count_of(&totals, &'a'), 2);
    assert_eq!(series_days(&days, 'a'), vec![(DAY, DayTotal { amount: 7, count: 2, ending_amount: 7, ending_count: 2 })]);
}

#[test]
fn signs_saturate_rather_than_wrap() {
    assert_eq!(signed_of(5), 5);
    assert_eq!(negated(5), -5);
    assert_eq!(unsigned_of(-3), 0);
    assert_eq!(unsigned_of(3), 3);
    assert_eq!(highest_of(2, 3), 3);
    assert_eq!(lowest_of(2, 3), 2);
    assert_eq!(wide_summed(u128::MAX, 1), u128::MAX);
    assert_eq!(negative_count(3), -3);
    assert_eq!(summed_signed(5, -7), -2);
    assert_eq!(unsigned_count(-1), 0);
}

#[test]
fn chart_conventions_round_the_axis_and_slice_the_pie() {
    assert_eq!(pie_slices(&[1.0, 3.0]), vec![(0.25, 0.0, 0.25), (0.75, 0.25, 1.0)]);
    assert_eq!(pie_slices(&[0.0, -1.0]), vec![]);
    assert_eq!(axis_ceiling(87.0, 1.1, &[1.0, 2.0, 5.0]), 100.0);
    assert_eq!(axis_ceiling(3.0, 1.0, &[1.0, 2.0, 5.0]), 5.0);
    assert_eq!(bar_height(50.0, 100.0, 200.0, 2.0), 100.0);
    assert_eq!(bar_height(0.0, 100.0, 200.0, 2.0), 2.0);
    assert_eq!(unit_fraction(1.0, 4.0), 0.25);
    assert_eq!(unit_fraction(1.0, 0.0), 0.0);
    let lines = vec![(String::from("a"), vec![(100, 1.0), (300, 2.0)]), (String::from("b"), vec![(200, 3.0)])];
    let points = points_of_lines(&lines);
    assert_eq!(latest_time(&points), Some(300));
    assert_eq!(window_from(Some(1000), 2, 100), Some(800));
    assert_eq!(window_from(None, 2, 100), None);
    assert_eq!(point_on(&points[0], 250), 1.0);
    assert_eq!(point_on(&points[0], 50), 0.0);
}
