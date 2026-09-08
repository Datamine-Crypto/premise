use crate::levels::{ending_on, signed_whole, Prices, Rows, Units};
use crate::money::{none, rounded, unit};
use crate::table::{at_or_before, union_keys};
use patterns_macros::{because, source};

pub type Point = (u64, f64);

pub type NamedLine = (String, Vec<Point>);

pub fn named_lines<T, L: Copy>(items: &[(String, T)], line: L, read: fn(&T, L) -> Vec<Point>) -> Vec<NamedLine> {
    items
        .iter()
        .map(|(name, item)| (name.clone(), read(item, line)))
        .filter(|(_, points)| !points.is_empty())
        .collect()
}
because!(named_lines, "the same line read off every named thing in a list, with anything that has no points left out, which is one chart; the reader is a parameter because what a line means differs per subject while gathering them into a chart never does");
because!(NamedLine, "one line of a chart as a name and its points, the shape every chart in this library is built from and handed on in, named once so a signature says what it carries rather than spelling the pair out again");

pub fn point_on(points: &[Point], day: u64) -> f64 {
    points
        .iter()
        .rev()
        .find(|(d, _)| *d <= day)
        .map(|(_, v)| *v)
        .unwrap_or(none())
}
because!(point_on, "a charted series read at a day, carried forward from its newest point at or before it, zero before it starts, which is how a report reads a chart at a month end; read from the newest backwards so it stops at the point that answers rather than walking the whole series to keep the last one that did");

pub fn pie_slices(amounts: &[f64]) -> Vec<(f64, f64, f64)> {
    let total: f64 = amounts.iter().map(|a| a.max(none())).sum();
    if total <= none() {
        return vec![];
    }
    let mut start = none();
    amounts
        .iter()
        .map(|amount| {
            let fraction = amount.max(none()) / total;
            let end = start + fraction;
            let out = (fraction, start, end);
            start = end;
            out
        })
        .collect()
}
because!(pie_slices, "each amount's share of the whole and where its slice starts and ends around the circle, negatives counted as nothing, or no slices when there is nothing to share");

pub fn axis_ceiling(max: f64, headroom: f64, steps: &[f64]) -> f64 {
    let raised = max.max(unit()) * headroom;
    let magnitude = DECIMAL_BASE_POWER.powf(raised.log10().floor());
    steps
        .iter()
        .map(|step| magnitude * step)
        .find(|candidate| *candidate >= raised)
        .unwrap_or(magnitude * DECIMAL_BASE_POWER)
}
because!(axis_ceiling, "the round figure above the tallest bar the axis ends at, the smallest round multiple of a power of ten that clears the bar with its headroom");

const DECIMAL_BASE_POWER: f64 = 10.0;
because!(DECIMAL_BASE_POWER, ChartConventions, "the base an axis ceiling is rounded in, ten, since readers count in tens");

pub struct ChartConventions;
source!(ChartConventions, "the drawing rules the retired client used for its bar chart, the one-two-five axis and the pixel floor under a bar");

pub fn bar_height(value: f64, ceiling: f64, height: f64, floor: f64) -> f64 {
    (value / ceiling * height).max(floor)
}
because!(bar_height, "a bar's height in pixels against the axis ceiling, never under the floor that keeps a small bar visible");

pub fn unit_fraction(value: f64, whole: f64) -> f64 {
    match whole > none() {
        true => value / whole,
        false => none(),
    }
}
because!(unit_fraction, "a part over a whole as a plain fraction, or nothing over an empty whole");

pub fn latest_time(lines: &[Vec<Point>]) -> Option<u64> {
    lines.iter().flat_map(|line| line.iter().map(|(day, _)| *day)).max()
}
because!(latest_time, "the newest moment across every line of a chart, the right edge the chart opens on");

pub fn window_from(latest: Option<u64>, days: u64, day_seconds: u64) -> Option<u64> {
    latest.map(|end| end.saturating_sub(days.saturating_mul(day_seconds)))
}
because!(window_from, "the left edge a chart opens on, a count of days back from its newest moment, or nothing when the chart has no points");

pub fn points_of_lines(lines: &[(String, Vec<Point>)]) -> Vec<Vec<Point>> {
    lines.iter().map(|(_, points)| points.clone()).collect()
}
because!(points_of_lines, "the points of every named line without their names, for a reading that spans the lines");

pub fn time_ordered(lines: &[(String, Vec<Point>)]) -> Vec<(String, Vec<Point>)> {
    lines
        .iter()
        .map(|(name, points)| {
            let mut ordered = points.clone();
            ordered.sort_by_key(|(day, _)| *day);
            (name.clone(), ordered)
        })
        .collect()
}
because!(time_ordered, "every named line with its points oldest first, which a chart insists on, while a report reads its month rows newest first");

pub fn ending_points(rows: &Rows, units: Units, decimals: u32) -> Vec<Point> {
    rows.iter()
        .map(|(day, row)| (*day, rounded(signed_whole(row.ending_amount, units), decimals)))
        .collect()
}
because!(ending_points, "a series' level at the end of each of its days as tokens, the line a level chart draws");

pub fn remaining_points(minted: &Rows, burned: &Rows, units: Units, decimals: u32) -> Vec<Point> {
    union_keys(&[minted, burned])
        .into_iter()
        .map(|day| (day, rounded(ending_on(minted, day, units) - ending_on(burned, day, units), decimals)))
        .collect()
}
because!(remaining_points, "what remains of a supply on each day either series moved, minted less burned with both carried forward");

pub fn walked_usd_points(rows: &Rows, prices: &Prices, min_day: u64, units: Units, decimals: u32) -> Vec<Point> {
    let mut out = Vec::new();
    for (day, row) in rows {
        if *day <= min_day {
            continue;
        }
        let price = match at_or_before(prices, *day) {
            Some(p) => p,
            None => continue,
        };
        out.push((*day, rounded(signed_whole(row.amount, units) * price, decimals)));
    }
    out
}
because!(walked_usd_points, "a walked series revalued in dollars on each of its days at that day's carried price, from the first day after a floor and only where a price exists");

pub fn after_day(points: Vec<Point>, min_day: u64) -> Vec<Point> {
    points.into_iter().filter(|(day, _)| *day > min_day).collect()
}
because!(after_day, "a series cut to the days after a floor, for a token whose early days the dashboard does not chart");

pub fn count_points(rows: &Rows) -> Vec<Point> {
    rows.iter().map(|(day, row)| (*day, row.ending_count as f64)).collect()
}
because!(count_points, "a series' ending count on each of its days, the line the validator charts draw");

pub fn share_of_supply_points(part: &Rows, minted: &Rows, burned: &Rows, units: Units, percent: f64, decimals: u32) -> Vec<Point> {
    let mut out = Vec::new();
    for (day, row) in part {
        let supply = ending_on(minted, *day, units) - ending_on(burned, *day, units);
        if supply <= none() {
            continue;
        }
        out.push((*day, rounded((signed_whole(row.ending_amount, units) / supply) * percent, decimals)));
    }
    out
}
because!(share_of_supply_points, "a series as a share of the supply on each of its days, minted less burned carried to the day, skipping days with no supply");

pub fn fixed_share_points(locked: &Rows, supply: f64, min_day: u64, units: Units, percent: f64, decimals: u32) -> Vec<Point> {
    if supply <= none() {
        return vec![];
    }
    locked
        .iter()
        .filter(|(day, _)| **day > min_day)
        .map(|(day, row)| (*day, rounded((signed_whole(row.ending_amount, units) / supply) * percent, decimals)))
        .collect()
}
because!(fixed_share_points, "a series on each day after a floor as a share of one fixed figure rather than of a moving one, so the line tells a story about the series and not about the figure");
