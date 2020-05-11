///
/// What we try to achieve is that all config can be
/// written in python files.
///
/// The interpreter has therefore be able to:
///     1. Parse python config files
///     2. Extract all relevant information
///         2.1 Use Case 1: get all declared aliases
///         2.2 Use Case 2: Collect functions and execute, when in use
///     3. Hot Reload configs?
///
///
/// How can we share or communicate with the rust code?
///
/// Method 1: Shared intermediate representations in file format (IRF)
///
///             1.1 Wrap the python configs in python code, which is
///                 then writes the information out into the IRF
///             1.1 Retrieve all informations over the given interface
///                 of rustpython and let rust write the IRF
///
/// Method 2: Direct messaging between the python interpreter and rust
///
///             2.1 XMLRPC?
///
///
///
///
use rustpython_compiler as compiler;
use rustpython_vm as vm;

use log::{error, info};

///
///
/// How the configs should be evaluated and handled
///
/// ~/.config/civa/civa.alias.py
///
/// ```python
///
/// import civa
///
/// civa.init()
///
/// civa.alias("lsa", "exa -la")
/// civa.alias("..", "cd ..")
///
///
/// civa.exec()
///
/// ```
///
///
///
///
pub fn run_interpreter(config_dir: String) -> vm::pyobject::PyResult<()> {
    let vm = vm::VirtualMachine::new(vm::PySettings::default());
    let scope = vm.new_scope_with_builtins();

    init_civa_lib(&vm, &scope, config_dir);

    let code_obj = vm
        .compile(
            r#"import civa; civa.alias("Hello", "world")"#,
            compiler::compile::Mode::Exec,
            "<embedded>".to_owned(),
        )
        .map_err(|err| vm.new_syntax_error(&err))?;

    vm.run_code_obj(code_obj, scope)?;

    Ok(())
}

fn init_civa_lib(vm: &vm::VirtualMachine, scope: &vm::scope::Scope, config_dir: String) {
    info!("Init PyConfig in dir: {}", config_dir);

    let header = format!(
        r#"
import civa

civa.config_dir = "{}"

"#,
        config_dir
    );

    match vm
        .compile(
            header.as_str(),
            compiler::compile::Mode::Exec,
            "<embedded>".to_owned(),
        )
        .map_err(|err| vm.new_syntax_error(&err))
    {
        Ok(code_obj) => match vm.run_code_obj(code_obj, scope.to_owned()) {
            Ok(_) => {}
            Err(err) => error!("Py Error: {:?}", err),
        },
        Err(err) => error!("Py Error: {:?}", err),
    }
}
