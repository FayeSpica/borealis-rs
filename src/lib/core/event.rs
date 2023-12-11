use std::sync::{Arc, Mutex};

pub struct Event<T> {
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(&T)>>>>,
}

pub struct Subscription<T> {
    index: usize,
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(&T)>>>>,
}

impl<T> Event<T> {
    pub fn new() -> Self {
        Event {
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe<F>(&mut self, callback: F) -> Subscription<T>
        where
            F: 'static + Fn(&T),
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(callback));

        Subscription {
            index: callbacks.len() - 1,
            callbacks: self.callbacks.clone(),
        }
    }

    pub fn unsubscribe(&mut self, subscription: Subscription<T>) {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.remove(subscription.index);
    }

    pub fn fire(&self, args: &T) -> bool {
        let callbacks = self.callbacks.lock().unwrap();
        for cb in &*callbacks {
            cb(args);
        }

        !callbacks.is_empty()
    }
}

impl<T> Clone for Subscription<T> {
    fn clone(&self) -> Self {
        Subscription {
            index: self.index,
            callbacks: self.callbacks.clone(),
        }
    }
}
