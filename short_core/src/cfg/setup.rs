use std::cell::RefCell;
use std::rc::Rc;

pub trait SetupsCfg {
    type Setup;

    fn add_setup(&mut self, setup: Self::Setup);

    fn remove_by_name_setup(&mut self, name: String);

    fn get_setup(&self, name: String) -> Option<Rc<RefCell<Self::Setup>>>;
}

pub trait Setup {
    fn name(&self) -> String;
}
