use std::sync::atomic::{AtomicBool, Ordering};

static PARSE_CONDITIONALS: AtomicBool = AtomicBool::new(false);

pub fn get_parse_conditionals() -> bool {
    PARSE_CONDITIONALS.load(Ordering::Relaxed)
}

pub fn set_parse_conditionals(bool: bool) {
    PARSE_CONDITIONALS.store(bool, Ordering::Relaxed);
}

