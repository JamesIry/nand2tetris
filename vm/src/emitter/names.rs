pub fn make_ref(name: &str) -> String {
    let mut string = "@".to_string();
    string.push_str(name);
    string
}

pub fn make_label(name: &str) -> String {
    let mut string = "(".to_string();
    string.push_str(name);
    string.push(')');
    string
}

pub fn make_static_name(statics_base: &str, index: u16) -> String {
    let mut string = statics_base.to_string();
    string.push('.');
    string.push_str(&index.to_string());
    string
}

pub fn make_qualified_label_name(function_name: &str, label: &str) -> String {
    let mut string = function_name.to_string();
    string.push('$');
    string.push_str(label);
    string
}

pub fn make_numbered_label_name(function_name: &str, label: &str, number: usize) -> String {
    let mut string = make_qualified_label_name(function_name, label);
    string.push_str(&number.to_string());
    string
}
