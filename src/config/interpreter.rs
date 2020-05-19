use std::collections::HashMap;
///
///
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
///
use std::fs::File;
use std::io::prelude::*;

use log::info;

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};

pub struct PyConfRuntime<'a> {
    py: Python<'a>,
    pyconf_lib_path: &'a str,
}

impl<'a> PyConfRuntime<'a> {
    pub fn new(gil: &'a GILGuard, pyconf_lib_path: &'a str) -> Self {
        let py = gil.python();
        Self {
            py,
            pyconf_lib_path,
        }
    }

    pub fn exec_configs(&self) {
        let locals = PyDict::new(self.py);
        // let globals = PyDict::new(self.py);
        let globals = [("__builtins__", self.py.import("builtins").unwrap())].into_py_dict(self.py);

        let config_content = self.get_py_file_content();

        let setup = r#"
_aliases = {}
_exports = {}

def aliases(kwrags):
    global _aliases
    for k, v in kwrags.items():
        _aliases[k] = v

def alias(key, value):
    global _aliases
    _aliases[key] = value

def export(key, value):
    global _exports
    _exports[key] = value
            "#;

        let exec_script: Vec<&str> = vec![setup, config_content.as_str()];

        match self
            .py
            .run(exec_script.join("\n").as_str(), Some(globals), Some(locals))
        {
            Ok(_) => info!("Success"),
            Err(err) => info!("Error: {:?}", err.print(self.py)),
        }

        match locals.get_item("foo").unwrap().call0() {
            Ok(_) => info!("Success"),
            Err(err) => info!("Error: {:?}", err.print(self.py)),
        }
    }

    // TODO: Should be partly done by XDG
    fn get_py_file_content(&self) -> String {
        let mut paths = vec![self.pyconf_lib_path];
        paths.push("/");
        paths.push("civa.py");

        let path = paths.join("");

        info!("{}", path);
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        contents
    }

    // pub fn get_alias_map() -> HashMap<String, String> {

    // }
}
