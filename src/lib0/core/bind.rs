use std::cell::RefCell;
use std::rc::Rc;
use log::error;
use crate::lib::core::activity::Activity;
use crate::lib::core::view::View;

struct BoundView {
    id: String,
    view: Rc<RefCell<Option<Box<dyn View>>>>,
    owner_view: Rc<RefCell<Option<Box<dyn View>>>>,
    owner_activity: Rc<RefCell<Option<Box<dyn Activity>>>>,
}

impl BoundView {
    fn new_view(id: String, owner_view: Rc<RefCell<Option<Box<dyn View>>>>,) -> Box<Self> {
        Box::new(BoundView {
            id,
            view: Rc::new(RefCell::new(None)),
            owner_view,
            owner_activity: Rc::new(RefCell::new(None)),
        })
    }

    fn new_activity(id: String, owner_activity: Rc<RefCell<Option<Box<dyn Activity>>>>,) -> Box<Self> {
        Box::new(BoundView {
            id,
            view: Rc::new(RefCell::new(None)),
            owner_view: Rc::new(RefCell::new(None)),
            owner_activity,
        })
    }

    fn get_view(&mut self) -> Rc<RefCell<Option<Box<dyn View>>>> {
        self.resolve();
        Rc::clone(&self.view)
    }

    fn resolve(&mut self) {
        if self.view.borrow().is_some() {
            return;
        }

        // Resolve by owner view first
        // if let Some(owner_view) = &self.owner_view.get_mut() {
        //     self.view = owner_view.get_view(&self.id);
        //
        //     if self.view.borrow().is_none() {
        //         error!("Cannot find view with ID")
        //     }
        // }
        // // Then resolve by owner activity
        // else if let Some(owner_activity) = &self.owner_activity.get_mut() {
        //     self.view = owner_activity.get_view(&self.id);
        //
        //     if self.view.borrow().is_none() {
        //         error!("Cannot find view with ID")
        //     }
        // } else {
        //     error!("Cannot find view with ID")
        // }
    }
}