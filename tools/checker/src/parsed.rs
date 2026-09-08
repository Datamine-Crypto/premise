use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Held = Result<Rc<syn::File>, String>;

thread_local! {
    static SEEN: RefCell<HashMap<String, Held>> = RefCell::new(HashMap::new());
}

pub fn of(text: &str) -> Held {
    SEEN.with(|held| {
        if let Some(hit) = held.borrow().get(text) {
            return hit.clone();
        }
        let made = syn::parse_file(text).map(Rc::new).map_err(|e| e.to_string());
        held.borrow_mut().insert(text.to_string(), made.clone());
        made
    })
}
