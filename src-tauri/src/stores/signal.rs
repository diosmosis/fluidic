use leptos::reactive::signal::{signal, ReadSignal, WriteSignal};

pub struct Signal<T: Send + Sync + 'static> {
    pub read: ReadSignal<T>,
    pub write: WriteSignal<T>,
}

impl<T: Send + Sync + 'static> Signal<T> {
    pub fn new(value: T) -> Self {
        let (read, write) = signal(value);
        Self { read, write }
    }
}
