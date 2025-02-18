use std::{cell::Cell, rc::Rc, time::Duration};

use leptos::{leptos_dom::helpers::TimeoutHandle, *};

use crate::instant::Instant;

pub(crate) fn use_timeout(
    func: impl Fn() -> Option<TimeoutHandle> + 'static,
) -> impl Fn() {
    // Saves last interval to be cleared on cleanup.
    let timeout: Rc<Cell<Option<TimeoutHandle>>> = Rc::new(Cell::new(None));
    let clean_up = {
        let interval = timeout.clone();
        move || {
            if let Some(handle) = interval.take() {
                handle.clear();
            }
        }
    };

    on_cleanup(clean_up.clone());

    create_effect(move |maybe_handle: Option<Option<TimeoutHandle>>| {
        let maybe_handle = maybe_handle.flatten().or_else(|| timeout.take());
        if let Some(handle) = maybe_handle {
            handle.clear();
        }

        let result = func();
        timeout.set(result);

        result
    });

    clean_up
}

pub(crate) fn time_until_stale(updated_at: Instant, stale_time: Duration) -> Duration {
    let updated_at = updated_at.0.as_millis() as i64;
    let now = Instant::now().0.as_millis() as i64;
    let stale_time = stale_time.as_millis() as i64;
    let result = (updated_at + stale_time) - now;
    let ensure_non_negative = result.max(0);
    Duration::from_millis(ensure_non_negative as u64)
}

pub(crate) fn maybe_time_until_stale(
    updated_at: Option<Instant>,
    stale_time: Option<Duration>,
) -> Option<Duration> {
    match (updated_at, stale_time) {
        (Some(last_updated), Some(stale_time)) => Some(time_until_stale(last_updated, stale_time)),
        _ => None,
    }
}
