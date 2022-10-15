use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Span<T: Debug> {
    inner: T,
    start: usize,
    line_start: usize,
    column_start: usize,
    end: usize,
    line_end: usize,
    column_end: usize,
}

impl<'a> Debug for Span<&'a str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "location:{}:{} - location:{}:{}\n{}",
            self.line_start,
            self.column_start,
            self.line_end,
            self.column_end,
            &self.inner[self.start..self.end]
        )
    }
}

impl<'a> Span<&'a str> {
    fn new(input: &'a str) -> Self {
        let last_line_pos = input
            .lines()
            .enumerate()
            .last()
            .map_or((0, 0), |(index, last_line)| (index, last_line.len()));
        Self {
            inner: input,
            start: 0,
            line_start: 0,
            column_start: 0,
            end: input.len(),
            line_end: last_line_pos.0,
            column_end: last_line_pos.1,
        }
    }
}

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// RUST_LOG=trace cargo watch -x 'test -- --nocapture test_parse_number'
#[test]
fn test_parse_number() {
    init();
    /*let span = Span::new(r#"
    (this is an (epic awesome great) "test" 5)
    "#);*/
    let _span = Span::new(r#"test fdsf"#);

    let _span = Span::new(r#"notest fdsf"#);
}
