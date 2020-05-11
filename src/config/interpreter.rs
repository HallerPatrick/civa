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
use std::process::Command;

use log::info;

use pyo3::prelude::*;

pub struct PyConfRuntime<'a> {
    py: Python<'a>,
}

impl<'a> PyConfRuntime<'a> {
    pub fn new(gil: &'a GILGuard) -> Self {
        let py = gil.python();

        let sys = py.import("sys").unwrap();
        let version: String = sys.get("version").unwrap().extract().unwrap();

        info!("Using Python Version: {}", version);

        Self { py }
    }

    fn import_civa_lib_from_path(&self, path: &str) {
        // let sys = self.py.import("sys").unwrap();
        // sys.get("path").unwrap()
        //     .self.py.import("civa").unwrap();
    }

    ///
    /// Check if civa is found in site-packages
    ///
    pub fn import_civa_lib(&self) -> bool {
        match self.py.import("civa") {
            Ok(_) => true,
            _ => false,
        }
    }
}

pub fn exec_pyconf() {
    let gil = Python::acquire_gil();
    PyConfRuntime::new(&gil);
}
