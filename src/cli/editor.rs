use std::borrow::Cow::{self, Borrowed, Owned};
use std::cell::RefCell;

use crate::config::ContextManager;
use log::info;
use rcalc::{Calculator, RuntimeItem, Value};
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::OutputStreamType;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::Helper;
use rustyline::{Cmd, CompletionType, Config, Context, EditMode, Editor, KeyPress};
use termion::color::{Fg, Green};

pub struct MyHelper {
    pub completer: FilenameCompleter,
    pub highlighter: MatchingBracketHighlighter,
    pub validator: MatchingBracketValidator,
    pub hinter: HistoryHinter,
    pub colored_prompt: String,
    pub calculator: RefCell<Calculator>,
}

impl Helper for MyHelper {}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for MyHelper {
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        if self.calculator.borrow().is_arithmetic_expression(line) {
            let curr_str = line;

            let mut curr_strr = String::from(curr_str);

            // Remove leading $
            curr_strr.remove(0);

            let expr = curr_strr;
            let expr_str = expr.trim();

            info!("Arithmetics operator: {}", line);

            match self.calculator.borrow_mut().calc(expr_str) {
                Ok(item) => {
                    if let RuntimeItem::Value(ref v) = item {
                        match *v {
                            Value::Integer(n) => {
                                return Some(format!("{} : {}", Fg(Green), n));
                            }
                            _ => return self.hinter.hint(line, pos, ctx),
                        }
                    } else {
                        return self.hinter.hint(line, pos, ctx);
                    };
                }
                Err(_) => self.hinter.hint(line, pos, ctx),
            }
        } else {
            self.hinter.hint(line, pos, ctx)
        }
    }
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for MyHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}
pub fn built_editor(ctx: &ContextManager) -> Editor<MyHelper> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build();
    let h = MyHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
        calculator: RefCell::new(Calculator::new()),
    };
    let mut rl = Editor::with_config(config);
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyPress::Meta('N'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyPress::Meta('P'), Cmd::HistorySearchBackward);

    info!("{}", ctx.retrieve_history_cache().as_str());
    if rl
        .load_history(ctx.retrieve_history_cache().as_str())
        .is_err()
    {
        info!("No previous history.");
    } else {
        info!("Load history file succesfully");
    }
    rl
}
