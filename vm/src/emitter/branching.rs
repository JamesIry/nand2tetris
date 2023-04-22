use super::names::*;
use super::push_pop::*;

pub fn emit_goto(function: &str, label: &str) -> Vec<String> {
    let mut results = Vec::new();
    results.push(make_ref(&make_qualified_label_name(function, label)));
    results.push("0;JMP".to_string());
    results
}

pub fn emit_if_goto(function: &str, label: &str) -> Vec<String> {
    let mut results = pop_d();
    results.push(make_ref(&make_qualified_label_name(function, label)));
    results.push("D;JNE".to_string());
    results
}

pub fn emit_label(function: &str, label: &str) -> Vec<String> {
    vec![make_label(&make_qualified_label_name(function, label))]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_emit_goto() {
        assert_eq!(emit_goto("bar", "FOO"), vec!["@bar$FOO", "0;JMP"]);
    }

    #[test]
    fn test_emit_if_goto() {
        assert_eq!(
            emit_if_goto("bar", "FOO"),
            vec!["@SP", "M=M-1", "A=M", "D=M", "@bar$FOO", "D;JNE"]
        );
    }

    #[test]
    fn test_emit_label() {
        assert_eq!(emit_label("bar", "FOO"), vec!["(bar$FOO)"]);
    }
}
