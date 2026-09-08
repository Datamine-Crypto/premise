use crate::charts::NamedLine;
use crate::money::none;
use crate::render::{filled, Named, Piece};
use patterns_macros::because;

pub type Reading = (u64, Vec<f64>);

pub fn readings_between(readings: Vec<Reading>, from: u64, to: u64) -> Vec<Reading> {
    readings.into_iter().filter(|(day, _)| *day >= from && *day <= to).collect()
}
because!(readings_between, "the readings inside a day range, both ends included, the range a reader picked in the date filter");

pub fn readings_paged(readings: Vec<Reading>, newest_first: bool, page: usize, per_page: usize) -> (Vec<Reading>, bool) {
    let ordered: Vec<Reading> = match newest_first {
        true => readings.into_iter().rev().collect(),
        false => readings,
    };
    let start = page.saturating_mul(per_page).min(ordered.len());
    let end = start.saturating_add(per_page).saturating_add(1).min(ordered.len());
    let slice = &ordered[start..end];
    let has_more = slice.len() > per_page;
    (slice.iter().take(per_page).cloned().collect(), has_more)
}
because!(readings_paged, "one page of readings in the direction asked for, with one more read than shown so the page knows whether another follows");

pub fn column_points(readings: &[Reading], column: usize) -> Vec<(u64, f64)> {
    readings
        .iter()
        .map(|(day, values)| (*day, values.get(column).copied().unwrap_or(none())))
        .collect()
}
because!(column_points, "one column of a table of readings as chart points, so a chart is drawn from the same numbers the table shows");

pub fn readings_keyed<K: Clone>(readings: Vec<Reading>, keys: &[K]) -> Vec<(u64, Vec<(K, f64)>)> {
    readings
        .into_iter()
        .map(|(day, values)| (day, keys.iter().cloned().zip(values).collect()))
        .collect()
}
because!(readings_keyed, "readings with each value paired to its column key in order, the shape a table row is answered in, with a value past the columns dropped and a column past the values absent");

pub trait Charted {
    type Col: PartialEq + Copy;
    type Ph: Named + 'static;
    type Reach: PartialEq + Copy;
    fn lines(&self) -> Vec<(&'static [Piece<Self::Ph>], Self::Col, Self::Reach)>;
}
because!(Charted, "what a chart definition tells the renderer: each line's name pieces, the column it draws and where it applies, so the renderer draws a spec's charts without naming them");

pub fn chart_series_of<C: Charted>(charts: &[C], readings: &[Reading], columns: &[C::Col], allowed: &[C::Reach], first: &str, second: &str) -> Vec<Vec<NamedLine>> {
    charts
        .iter()
        .map(|chart| {
            chart
                .lines()
                .into_iter()
                .filter(|(_, _, reach)| allowed.contains(reach))
                .filter_map(|(pieces, column, _)| {
                    let at = columns.iter().position(|c| *c == column)?;
                    Some((filled(pieces, first, second, "", ""), column_points(readings, at)))
                })
                .collect()
        })
        .collect()
}
because!(chart_series_of, "every chart's lines drawn from the table's readings, each line named from its pieces and read from the column it names, with the lines that do not apply on this deployment left out");
