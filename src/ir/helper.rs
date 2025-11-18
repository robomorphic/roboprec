use once_cell::sync::Lazy;
use std::{collections::HashSet, sync::Mutex};

/// Global counter for generating unique names.
static TEMP_VAR_COUNTER: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

/// list of all the names generated
static TEMP_VAR_NAMES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn clear_all_names() {
    let mut names = TEMP_VAR_NAMES.lock().unwrap();
    names.clear();
    *TEMP_VAR_COUNTER.lock().unwrap() = 0;
}

/// Generates a unique variable name given a prefix.
fn generate_name_if_needed(prefix: &str, names: &HashSet<String>) -> String {
    // remove all preceding '_' characters
    let prefix = prefix.trim_start_matches('_');
    // if the prefix is too long, cut it down to a reasonable length
    let max_length = 40;
    let mut counter = TEMP_VAR_COUNTER.lock().unwrap();

    let prefix = if prefix.len() > max_length {
        *counter += 1;
        format!("var_{}", *counter)
    } else {
        prefix.to_string()
    };

    if !names.contains(&prefix) {
        return prefix;
    }
    
    *counter += 1;
    if prefix.is_empty() {
        format!("r_{}", *counter)
    } else {
        format!("r_{}_{}", *counter, prefix)
    }
}

/// This function handles if the name already exists or not.
pub fn add_name(name: &str) -> String {
    let mut names = TEMP_VAR_NAMES.lock().unwrap();

    let new_name = generate_name_if_needed(name, &names);
    names.insert(new_name.clone());
    new_name
}
