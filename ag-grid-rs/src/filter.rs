//! Types pertaining to grid filtering.

use ag_grid_core::imports::ObjectExt;
use chrono::NaiveDateTime;
use wasm_bindgen::JsCast;

#[derive(Debug)]
pub enum Comparator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    InRange,
    Blank,
    NotBlank,
    ChooseOne,
}

impl From<String> for Comparator {
    fn from(v: String) -> Self {
        match v.as_str() {
            "equals" => Self::Equals,
            "notEqual" => Self::NotEquals,
            "contains" => Self::Contains,
            "notContains" => Self::NotContains,
            "startsWith" => Self::StartsWith,
            "endsWith" => Self::EndsWith,
            "lessThan" => Self::LessThan,
            "lessThanOrEqual" => Self::LessThanOrEqual,
            "greaterThan" => Self::GreaterThan,
            "greaterThanOrEqual" => Self::GreaterThanOrEqual,
            "inRange" => Self::InRange,
            "blank" => Self::Blank,
            "notBlank" => Self::NotBlank,
            "empty" => Self::ChooseOne,
            // This is the full set of options from the AG Grid library.
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum FilterModelType {
    Single(FilterModel),
    Combined(CombinedFilterModel),
}

#[derive(Debug)]
pub enum FilterModel {
    Text(TextFilter),
    Number(NumberFilter),
    Date(DateFilter),
}

impl FilterModel {
    pub fn from_object(obj: &ObjectExt) -> Self {
        let filter_type = obj.get_string_unchecked("filterType");
        match filter_type.as_str() {
            "text" => FilterModel::Text(TextFilter::from_object(obj)),
            "number" => FilterModel::Number(NumberFilter::from_object(obj)),
            "date" => FilterModel::Date(DateFilter::from_object(obj)),
            _ => unreachable!(),
        }
    }
}

impl CombinedFilterModel {
    pub fn from_object(obj: &ObjectExt) -> Self {
        let filter_type = obj.get_string_unchecked("filterType");
        let operator = obj.get_string_unchecked("operator").into();
        let c1 = obj.get("condition1").unchecked_into::<ObjectExt>();
        let c2 = obj.get("condition2").unchecked_into::<ObjectExt>();
        match filter_type.as_str() {
            "text" => CombinedFilterModel::Text(CombinedTextFilter {
                operator,
                condition_1: TextFilter::from_object(&c1),
                condition_2: TextFilter::from_object(&c2),
            }),
            "number" => CombinedFilterModel::Number(CombinedNumberFilter {
                operator,
                condition_1: NumberFilter::from_object(&c1),
                condition_2: NumberFilter::from_object(&c2),
            }),
            "date" => CombinedFilterModel::Date(CombinedDateFilter {
                operator,
                condition_1: DateFilter::from_object(&c1),
                condition_2: DateFilter::from_object(&c2),
            }),

            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum CombinedFilterModel {
    Text(CombinedTextFilter),
    Number(CombinedNumberFilter),
    Date(CombinedDateFilter),
}

/// Describe how to handle multiple conditions.
#[derive(Debug)]
pub enum JoinOperator {
    /// Combine two given conditions using *and* semantics.
    And,
    /// Combine two given conditions using *or* semantics.
    Or,
}

impl From<String> for JoinOperator {
    fn from(v: String) -> Self {
        match v.as_str() {
            "AND" => Self::And,
            "OR" => Self::Or,
            // This is the complete list of operators as per AG Grid.
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct TextFilter {
    pub filter: Option<String>,
    pub filter_to: Option<String>,
    pub comparator: Option<Comparator>,
}

impl TextFilter {
    pub fn from_object(obj: &ObjectExt) -> Self {
        let comparator = obj.get_string("type").map(Comparator::from);
        Self {
            filter: obj.get_string("filter"),
            filter_to: obj.get_string("filterTo"),
            comparator,
        }
    }
}

#[derive(Debug)]
pub struct NumberFilter {
    pub filter: Option<f64>,
    pub filter_to: Option<f64>,
    pub comparator: Option<Comparator>,
}

impl NumberFilter {
    pub fn from_object(obj: &ObjectExt) -> Self {
        let comparator = obj.get_string("type").map(Comparator::from);
        Self {
            filter: obj.get_f64("filter"),
            filter_to: obj.get_f64("filterTo"),
            comparator,
        }
    }
}

#[derive(Debug)]
pub struct DateFilter {
    pub filter: Option<NaiveDateTime>,
    pub filter_to: Option<NaiveDateTime>,
    pub comparator: Option<Comparator>,
}

impl DateFilter {
    pub fn from_object(obj: &ObjectExt) -> Self {
        let comparator = obj.get_string("type").map(Comparator::from);
        Self {
            filter: obj.get_string("dateFrom").map(|dt| {
                NaiveDateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S")
                    .expect("Ag Grid should always pass a date in format 'YYYY-MM-DD hh:mm:ss'")
            }),
            filter_to: obj.get_string("dateFrom").map(|dt| {
                NaiveDateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S")
                    .expect("Ag Grid should always pass a date in format 'YYYY-MM-DD hh:mm:ss'")
            }),
            comparator,
        }
    }
}

#[derive(Debug)]
pub struct CombinedTextFilter {
    pub condition_1: TextFilter,
    pub condition_2: TextFilter,
    pub operator: JoinOperator,
}

#[derive(Debug)]
pub struct CombinedNumberFilter {
    pub condition_1: NumberFilter,
    pub condition_2: NumberFilter,
    pub operator: JoinOperator,
}

#[derive(Debug)]
pub struct CombinedDateFilter {
    pub condition_1: DateFilter,
    pub condition_2: DateFilter,
    pub operator: JoinOperator,
}
