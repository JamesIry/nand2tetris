use crate::ast::*;

pub fn print_command(command: &Command) -> String {
    match command {
        Command::Push(segment, index) => print_push_pop("push", segment, *index),
        Command::Pop(segment, index) => print_push_pop("pop", segment, *index),
        Command::Add => "add".to_string(),
        Command::Sub => "sub".to_string(),
        Command::Neg => "neg".to_string(),
        Command::Eq => "eq".to_string(),
        Command::Gt => "gt".to_string(),
        Command::Lt => "lt".to_string(),
        Command::And => "and".to_string(),
        Command::Or => "or".to_string(),
        Command::Not => "not".to_string(),
        Command::Goto(label) => print_command2("goto", label),
        Command::IfGoto(label) => print_command2("if-goto", label),
        Command::Label(label) => print_command2("label", label),
        Command::Function(function, n_locals) => {
            print_command3("function", function, &n_locals.to_string())
        }
        Command::Call(function, n_args) => print_command3("call", function, &n_args.to_string()),
        Command::Return => "return".to_string(),
        Command::Comment(comment) => comment.clone(),
    }
}

fn print_command2(command: &str, label: &str) -> String {
    let mut result = command.to_string();
    result.push(' ');
    result.push_str(label);
    result
}

fn print_command3(command: &str, arg1: &str, arg2: &str) -> String {
    let mut result = command.to_string();
    result.push(' ');
    result.push_str(arg1);
    result.push(' ');
    result.push_str(arg2);
    result
}

fn print_push_pop(arg: &str, segment: &Segment, index: u16) -> String {
    let seg_s = print_segment(segment);
    let ind_s = print_index(index);
    let mut result = arg.to_string();
    result.push(' ');
    result.push_str(&seg_s);
    result.push(' ');
    result.push_str(&ind_s);
    result
}

fn print_segment(segment: &Segment) -> String {
    match segment {
        Segment::Argument => "argument".to_string(),
        Segment::Local => "local".to_string(),
        Segment::Static => "static".to_string(),
        Segment::Constant => "constant".to_string(),
        Segment::This => "this".to_string(),
        Segment::That => "that".to_string(),
        Segment::Pointer => "pointer".to_string(),
        Segment::Temp => "temp".to_string(),
    }
}

fn print_index(index: u16) -> String {
    index.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_print_index() {
        assert_eq!(print_index(123), "123".to_string());
    }

    #[test]
    fn test_print_segment() {
        assert_eq!(print_segment(&Segment::Argument), "argument");
        assert_eq!(print_segment(&Segment::Local), "local");
        assert_eq!(print_segment(&Segment::Static), "static");
        assert_eq!(print_segment(&Segment::Constant), "constant");
        assert_eq!(print_segment(&Segment::This), "this");
        assert_eq!(print_segment(&Segment::That), "that");
        assert_eq!(print_segment(&Segment::Pointer), "pointer");
        assert_eq!(print_segment(&Segment::Temp), "temp");
    }

    #[test]
    fn test_print_command() {
        assert_eq!(
            print_command(&Command::Push(Segment::Constant, 42)),
            "push constant 42"
        );
        assert_eq!(
            print_command(&Command::Pop(Segment::Static, 103)),
            "pop static 103"
        );
        assert_eq!(print_command(&Command::Add), "add");
        assert_eq!(print_command(&Command::Sub), "sub");
        assert_eq!(print_command(&Command::Neg), "neg");
        assert_eq!(print_command(&Command::Eq), "eq");
        assert_eq!(print_command(&Command::Gt), "gt");
        assert_eq!(print_command(&Command::Lt), "lt");
        assert_eq!(print_command(&Command::And), "and");
        assert_eq!(print_command(&Command::Or), "or");
        assert_eq!(print_command(&Command::Not), "not");
        assert_eq!(
            print_command(&Command::Goto("LOOP".to_string())),
            "goto LOOP"
        );
        assert_eq!(
            print_command(&Command::IfGoto("LOOP".to_string())),
            "if-goto LOOP"
        );

        assert_eq!(
            print_command(&Command::Label("LOOP".to_string())),
            "label LOOP"
        );
        assert_eq!(
            print_command(&Command::Comment("// foo".to_string())),
            "// foo"
        );
    }
}
