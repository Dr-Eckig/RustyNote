use std::str::FromStr;

use leptos::{prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::storage::use_local_storage;

/// Creates a reactive signal that persists its value in `localStorage`.
///
/// ```rust,ignore
/// use markdown::api::local_storage::use_persistent_signal;
///
/// let signal = use_persistent_signal::<String>("draft".into());
/// signal.set("Hello".into());
/// ```
pub fn use_persistent_signal<T>(key: String) -> RwSignal<T>
where
    T: Clone + Default + PartialEq + ToString + FromStr + Send + Sync + 'static,
{
    let (stored, set_stored, _) = use_local_storage::<T, FromToStringCodec>(key);

    let signal = RwSignal::new(stored.get());

    Effect::new(move || {
        set_stored.set(signal.get())
    });

    signal
}
