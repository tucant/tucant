use std::rc::Rc;

use tucant_types::Tucan;


pub struct RcTucanType<TucanType: Tucan + 'static>(pub Rc<TucanType>);

impl<TucanType: Tucan + 'static> Clone for RcTucanType<TucanType> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<TucanType: Tucan + 'static> PartialEq for RcTucanType<TucanType> {
    fn eq(&self, other: &RcTucanType<TucanType>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
