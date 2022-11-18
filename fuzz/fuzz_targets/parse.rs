#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let mut parser = nsh::parser::ShellParser::new();
    _ = parser.parse(data);
});
