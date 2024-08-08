use std::borrow::Cow;

use chrono::{Datelike, Local, NaiveDate};
use gpui::{
    prelude::FluentBuilder as _, relative, ClickEvent, ElementId, EventEmitter, FocusHandle,
    InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, ViewContext,
};
use rust_i18n::t;

use crate::{
    button::Button,
    h_flex,
    theme::{ActiveTheme, Colorize},
    v_flex, Clickable, Disableable, IconName, Selectable,
};

use super::utils::days_in_month;

pub enum CalendarEvent {
    /// The user selected a date.
    Selected(Date),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Single,
    Range,
}

/// The date of the calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Date {
    Single(Option<NaiveDate>),
    Range(Option<NaiveDate>, Option<NaiveDate>),
}

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Self::Single(Some(date))
    }
}

impl From<(NaiveDate, NaiveDate)> for Date {
    fn from((start, end): (NaiveDate, NaiveDate)) -> Self {
        Self::Range(Some(start), Some(end))
    }
}

impl Date {
    fn is_active(&self, v: &NaiveDate) -> bool {
        let v = *v;
        match self {
            Self::Single(d) => Some(v) == *d,
            Self::Range(start, end) => Some(v) == *start || Some(v) == *end,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            Self::Single(Some(_)) | Self::Range(Some(_), _) => true,
            _ => false,
        }
    }

    /// Return formatted date string.
    pub fn format(&self, format: &str) -> Option<SharedString> {
        match self {
            Self::Single(Some(date)) => Some(date.format(format).to_string().into()),
            Self::Range(Some(start), Some(end)) => {
                Some(format!("{} - {}", start.format(format), end.format(format)).into())
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ViewMode {
    Day,
    Month,
    Year,
}

impl ViewMode {
    fn is_day(&self) -> bool {
        matches!(self, Self::Day)
    }

    fn is_month(&self) -> bool {
        matches!(self, Self::Month)
    }

    fn is_year(&self) -> bool {
        matches!(self, Self::Year)
    }
}

pub struct Calendar {
    focus_handle: FocusHandle,
    date: Date,
    view_mode: ViewMode,
    mode: Mode,
    current_year: i32,
    current_month: u8,
    years: Vec<Vec<i32>>,
    year_page: i32,
}

impl Calendar {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let today = Local::now().naive_local().date();
        Self {
            focus_handle: cx.focus_handle(),
            mode: Mode::Single,
            view_mode: ViewMode::Day,
            date: Date::Single(None),
            current_month: today.month() as u8,
            current_year: today.year(),
            years: vec![],
            year_page: 0,
        }
        .year_range((today.year() - 50, today.year() + 50))
    }

    /// Set the mode of the calendar, default is `Mode::Single`.
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// Set the date of the calendar.
    ///
    /// When you set a range date, the mode will be automatically set to `Mode::Range`.
    pub fn set_date(&mut self, date: impl Into<Date>, cx: &mut ViewContext<Self>) {
        self.date = date.into();

        match self.date {
            Date::Single(Some(date)) => {
                self.mode = Mode::Single;
                self.current_month = date.month() as u8;
                self.current_year = date.year();
            }
            Date::Range(Some(start), _) => {
                self.mode = Mode::Range;
                self.current_month = start.month() as u8;
                self.current_year = start.year();
            }
            _ => {}
        }

        cx.notify()
    }

    /// Get the date of the calendar.
    pub fn date(&self) -> Date {
        self.date
    }

    /// Set the year range of the calendar, default is 50 years before and after the current year.
    ///
    /// Each year page contains 20 years, so the range will be divided into chunks of 20 years is better.
    pub fn year_range(mut self, range: (i32, i32)) -> Self {
        self.years = (range.0..range.1)
            .collect::<Vec<_>>()
            .chunks(20)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();
        self.year_page = self
            .years
            .iter()
            .position(|years| years.contains(&self.current_year))
            .unwrap_or(0) as i32;
        self
    }

    /// Returns the days of the month in a 2D vector to render on calendar.
    fn days(&self) -> Vec<Vec<NaiveDate>> {
        days_in_month(self.current_year, self.current_month as u32)
    }

    fn has_prev_year_page(&self) -> bool {
        self.year_page > 0
    }

    fn has_next_year_page(&self) -> bool {
        self.year_page < self.years.len() as i32 - 1
    }

    fn prev_year_page(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        if !self.has_prev_year_page() {
            return;
        }

        self.year_page -= 1;
        cx.notify()
    }

    fn next_year_page(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        if !self.has_next_year_page() {
            return;
        }

        self.year_page += 1;
        cx.notify()
    }

    fn prev_month(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.current_month = if self.current_month == 1 {
            12
        } else {
            self.current_month - 1
        };
        self.current_year = if self.current_month == 12 {
            self.current_year - 1
        } else {
            self.current_year
        };
        cx.notify()
    }

    fn next_month(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        self.current_month = if self.current_month == 12 {
            1
        } else {
            self.current_month + 1
        };
        self.current_year = if self.current_month == 1 {
            self.current_year + 1
        } else {
            self.current_year
        };
        cx.notify()
    }

    fn month_name(&self) -> SharedString {
        match self.current_month {
            1 => t!("Calendar.month.January"),
            2 => t!("Calendar.month.February"),
            3 => t!("Calendar.month.March"),
            4 => t!("Calendar.month.April"),
            5 => t!("Calendar.month.May"),
            6 => t!("Calendar.month.June"),
            7 => t!("Calendar.month.July"),
            8 => t!("Calendar.month.August"),
            9 => t!("Calendar.month.September"),
            10 => t!("Calendar.month.October"),
            11 => t!("Calendar.month.November"),
            12 => t!("Calendar.month.December"),
            _ => Cow::Borrowed(""),
        }
        .into()
    }

    fn render_week(
        &self,
        week: impl Into<SharedString>,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        h_flex()
            .w_9()
            .h_9()
            .rounded_md()
            .justify_center()
            .text_color(cx.theme().muted_foreground)
            .text_sm()
            .child(week.into())
    }

    fn item_button(
        &self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        active: bool,
        muted: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement + Styled + StatefulInteractiveElement {
        h_flex()
            .id(id.into())
            .w_9()
            .h_9()
            .rounded_lg()
            .justify_center()
            .cursor_pointer()
            .when(muted, |this| {
                this.text_color(cx.theme().muted_foreground.opacity(0.3))
            })
            .when(!active, |this| {
                this.hover(|this| {
                    this.bg(cx.theme().accent)
                        .text_color(cx.theme().accent_foreground)
                })
            })
            .when(active, |this| {
                this.bg(cx.theme().primary)
                    .text_color(cx.theme().primary_foreground)
            })
            .child(label.into())
    }

    fn render_item(
        &self,
        ix: usize,
        d: &NaiveDate,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let day = d.day();
        let is_current_month = d.month() == self.current_month as u32;
        let is_active = self.date.is_active(d);

        let date = *d;

        self.item_button(ix, day.to_string(), is_active, !is_current_month, cx)
            .on_click(
                cx.listener(move |view, _: &ClickEvent, cx| match view.mode {
                    Mode::Single => {
                        view.set_date(date, cx);
                        cx.emit(CalendarEvent::Selected(view.date()));
                    }
                    Mode::Range => {
                        match view.date {
                            Date::Range(Some(start), None) if start < date => {
                                view.set_date((start, date), cx);
                            }
                            _ => {}
                        }

                        cx.emit(CalendarEvent::Selected(view.date()));
                    }
                }),
            )
    }

    fn set_view_mode(&mut self, mode: ViewMode, cx: &mut ViewContext<Self>) {
        self.view_mode = mode;
        cx.notify();
    }

    fn months(&self) -> Vec<SharedString> {
        [
            t!("Calendar.month.January"),
            t!("Calendar.month.February"),
            t!("Calendar.month.March"),
            t!("Calendar.month.April"),
            t!("Calendar.month.May"),
            t!("Calendar.month.June"),
            t!("Calendar.month.July"),
            t!("Calendar.month.August"),
            t!("Calendar.month.September"),
            t!("Calendar.month.October"),
            t!("Calendar.month.November"),
            t!("Calendar.month.December"),
        ]
        .iter()
        .map(|s| s.clone().into())
        .collect()
    }

    fn render_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let current_year = self.current_year;
        let disabled = self.view_mode.is_month();

        h_flex()
            .gap_0p5()
            .justify_between()
            .items_center()
            .child(
                Button::new("prev", cx)
                    .icon(IconName::ArrowLeft)
                    .ghost()
                    .disabled(disabled)
                    .when(self.view_mode.is_day(), |this| {
                        this.on_click(cx.listener(Self::prev_month))
                    })
                    .when(self.view_mode.is_year(), |this| {
                        this.when(!self.has_prev_year_page(), |this| this.disabled(true))
                            .on_click(cx.listener(Self::prev_year_page))
                    }),
            )
            .child(
                h_flex()
                    .justify_center()
                    .gap_3()
                    .child(
                        Button::new("month", cx)
                            .ghost()
                            .label(self.month_name())
                            .selected(self.view_mode.is_month())
                            .compact()
                            .on_click(cx.listener(|view, _, cx| {
                                if view.view_mode.is_month() {
                                    view.set_view_mode(ViewMode::Day, cx);
                                } else {
                                    view.set_view_mode(ViewMode::Month, cx);
                                }
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("year", cx)
                            .ghost()
                            .label(current_year.to_string())
                            .compact()
                            .selected(self.view_mode.is_year())
                            .on_click(cx.listener(|view, _, cx| {
                                if view.view_mode.is_year() {
                                    view.set_view_mode(ViewMode::Day, cx);
                                } else {
                                    view.set_view_mode(ViewMode::Year, cx);
                                }
                                cx.notify();
                            })),
                    ),
            )
            .child(
                Button::new("next", cx)
                    .icon(IconName::ArrowRight)
                    .ghost()
                    .disabled(disabled)
                    .when(self.view_mode.is_day(), |this| {
                        this.on_click(cx.listener(Self::next_month))
                    })
                    .when(self.view_mode.is_year(), |this| {
                        this.when(!self.has_next_year_page(), |this| this.disabled(true))
                            .on_click(cx.listener(Self::next_year_page))
                    }),
            )
    }

    fn render_days(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let weeks = [
            t!("Calendar.week.0"),
            t!("Calendar.week.1"),
            t!("Calendar.week.2"),
            t!("Calendar.week.3"),
            t!("Calendar.week.4"),
            t!("Calendar.week.5"),
            t!("Calendar.week.6"),
        ];

        v_flex()
            .child(
                h_flex()
                    .gap_0p5()
                    .justify_between()
                    .children(weeks.iter().map(|week| self.render_week(week.clone(), cx))),
            )
            .children(self.days().iter().map(|week| {
                h_flex().gap_0p5().justify_between().children(
                    week.iter()
                        .enumerate()
                        .map(|(ix, d)| self.render_item(ix, d, cx)),
                )
            }))
    }

    fn render_months(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let months = self.months();

        h_flex()
            .mt_3()
            .gap_0p5()
            .gap_y_3()
            .justify_between()
            .flex_wrap()
            .children(
                months
                    .iter()
                    .enumerate()
                    .map(|(ix, month)| {
                        let active = (ix + 1) as u8 == self.current_month;

                        self.item_button(ix, month.to_string(), active, false, cx)
                            .w(relative(0.3))
                            .on_click(cx.listener(move |view, _, cx| {
                                view.current_month = (ix + 1) as u8;
                                view.set_view_mode(ViewMode::Day, cx);
                                cx.notify();
                            }))
                    })
                    .collect::<Vec<_>>(),
            )
    }

    fn render_years(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let current_page_years = &self.years[self.year_page as usize];

        h_flex()
            .id("years")
            .mt_3()
            .gap_0p5()
            .gap_y_3()
            .justify_between()
            .flex_wrap()
            .children(
                current_page_years
                    .iter()
                    .enumerate()
                    .map(|(ix, year)| {
                        let year = *year;
                        let active = year == self.current_year;

                        self.item_button(ix, year.to_string(), active, false, cx)
                            .w(relative(0.2))
                            .on_click(cx.listener(move |view, _, cx| {
                                view.current_year = year;
                                view.set_view_mode(ViewMode::Day, cx);
                                cx.notify();
                            }))
                    })
                    .collect::<Vec<_>>(),
            )
    }
}

impl EventEmitter<CalendarEvent> for Calendar {}

impl Render for Calendar {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        v_flex()
            .track_focus(&self.focus_handle)
            .gap_0p5()
            .text_sm()
            .child(self.render_header(cx))
            .when(self.view_mode.is_day(), |this| {
                this.child(self.render_days(cx))
            })
            .when(self.view_mode.is_month(), |this| {
                this.child(self.render_months(cx))
            })
            .when(self.view_mode.is_year(), |this| {
                this.child(self.render_years(cx))
            })
    }
}
