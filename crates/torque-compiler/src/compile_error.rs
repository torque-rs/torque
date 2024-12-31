use std::{error::Error, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
	#[error("io error")]
	IoError(#[from] std::io::Error),

	#[error("module not resolved (specifier: {specifier:?})")]
	ModuleNotResolved { specifier: Option<String> },

	#[error("module not found (specifier: {specifier:?}, path: {path:?})")]
	ModuleNotFound {
		specifier: Option<String>,
		path: PathBuf,
	},

	#[error("module not transformed (specifier: {specifier:?}, path: {path:?})")]
	ModuleNotTransformed {
		specifier: Option<String>,
		path: PathBuf,
		error: Box<dyn Error>,
	},

	#[error("module not compiled: (specifier: {specifier:?}, path: {path:?})")]
	ModuleNotCompiled {
		specifier: Option<String>,
		path: PathBuf,
	},

	#[error("module not instantiated: (specifier: {specifier:?}, path: {path:?})")]
	ModuleNotInstantiated {
		specifier: Option<String>,
		path: PathBuf,
	},

	#[error("module not evaluated: (specifier: {specifier:?}, path: {path:?})")]
	ModuleNotEvaluated {
		specifier: Option<String>,
		path: PathBuf,
	},
}
