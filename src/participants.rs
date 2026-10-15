use std::collections::HashMap;

pub fn unique_count(names: &[String]) -> usize {
    let mut students: HashMap<&String, usize> = HashMap::new();

    for name in names {
        if !students.contains_key(name) {
            students.insert(name, 0);
        }
        *students.get_mut(name).unwrap() += 1;
    }

    students.len()
}
