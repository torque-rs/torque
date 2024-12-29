use std::{cell::RefCell, fmt::Debug, ops::Deref, path::PathBuf, rc::Rc, sync::Arc};

use fnv::FnvHashMap;
use log::trace;
use swc::config::{Config, JscConfig, Options, TransformConfig};
use swc_common::{errors::Handler, FilePathMapping, SourceMap, GLOBALS};
use swc_core::ecma::ast::EsVersion;
use swc_ecma_parser::{Syntax, TsSyntax};
use swc_ecma_transforms_react::Runtime;
use v8::script_compiler::compile_module;

#[derive(Clone)]
pub struct Compiler(Rc<Inner>);

impl Compiler {
	pub fn new() -> Self {
		Self(Rc::new(Inner::default()))
	}
}

impl Deref for Compiler {
	type Target = Rc<Inner>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Default for Compiler {
	fn default() -> Self {
		Self(Rc::new(Inner::default()))
	}
}

pub struct Inner {
	source_map: Arc<SourceMap>,
	compiler: swc::Compiler,
	options: Options,
	modules: RefCell<FnvHashMap<String, v8::Global<v8::Module>>>,
}

impl Debug for Inner {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Inner")
			.field("options", &self.options)
			.field("modules", &self.modules)
			.finish()
	}
}

impl Inner {
	pub fn new() -> Self {
		let source_map = Arc::new(SourceMap::new(FilePathMapping::empty()));

		Self {
			source_map: source_map.clone(),
			compiler: swc::Compiler::new(source_map),
			options: Options {
				config: Config {
					jsc: JscConfig {
						syntax: Some(Syntax::Typescript(TsSyntax {
							tsx: true,
							decorators: true,
							//dts: true,
							..Default::default()
						})),
						target: Some(EsVersion::Es2024),
						transform: Some(TransformConfig {
							react: swc_ecma_transforms_react::Options {
								runtime: Some(Runtime::Automatic),
								import_source: Some("@torque-rs".into()),
								throw_if_namespace: Some(false),
								..Default::default()
							},
							..Default::default()
						})
						.into(),
						..Default::default()
					},
					..Default::default()
				},
				..Default::default()
			},
			modules: RefCell::new(FnvHashMap::default()),
		}
	}

	pub fn add_module(self: &Rc<Self>, specifier: String, module: v8::Global<v8::Module>) {
		let mut modules = self.modules.borrow_mut();

		trace!("adding module");

		modules.insert(specifier, module);
	}

	pub fn get_module<'s>(
		self: &Rc<Self>,
		scope: &mut v8::HandleScope<'s, v8::Context>,
		specifier: &str,
	) -> Option<v8::Local<'s, v8::Module>> {
		let modules = self.modules.borrow();

		trace!("getting module {}", specifier);

		modules
			.get(specifier)
			.map(|v| v8::Local::new(scope, v))
			.inspect(|_| trace!("success"))
	}

	//#[instrument(skip(self, scope))]
	pub fn load_module<'s>(
		self: &Rc<Self>,
		scope: &mut v8::HandleScope<'s, v8::Context>,
		specifier: Option<String>,
		path: Option<PathBuf>,
	) -> Option<v8::Local<'s, v8::Module>> {
		// TODO: resolve specifier to source_path
		let (source_path, specifier) = match (specifier, path) {
			(None, None) => None?,
			(None, Some(path)) => (path, None),
			(Some(specifier), None) => (PathBuf::from(&specifier), Some(specifier)),
			(Some(specifier), Some(path)) => (path, Some(specifier)),
		};

		println!("loading: {}", source_path.to_string_lossy());

		let source_file = self.source_map.load_file(&source_path).unwrap();

		let output = GLOBALS
			.set(&Default::default(), || {
				self.compiler.process_js_file(
					source_file,
					&Handler::with_tty_emitter(
						swc_common::errors::ColorConfig::Auto,
						true,
						true,
						Some(self.source_map.clone()),
					),
					&self.options,
				)
			})
			.map_err(|error| {
				let message = v8::String::new(scope, &error.to_string()).unwrap();
				let exception = v8::Exception::error(scope, message);
				scope.throw_exception(exception);
			})
			.ok()?;

		let code = v8::String::new(scope, &output.code).unwrap();
		println!("javascript code: {}", code.to_rust_string_lossy(scope));

		let resource_name = v8::String::new(scope, &source_path.to_string_lossy())
			.unwrap()
			.into();
		let script_origin = v8::ScriptOrigin::new(
			scope,
			resource_name,
			0,
			0,
			false,
			0,
			None,
			false,
			false,
			true,
			None,
		);

		let module = compile_module(
			scope,
			&mut v8::script_compiler::Source::new(code, Some(&script_origin)),
		)?;

		module
			.instantiate_module(scope, resolve_callback)
			.and_then(|v| {
				v.then(|| {
					let scope = &mut v8::TryCatch::new(scope);

					if let Some(specifier) = specifier {
						self.add_module(specifier, v8::Global::new(scope, module));
					}

					let _ = module.evaluate(scope);

					if let Some(exception) = scope.exception() {
						panic!("{}", exception.to_rust_string_lossy(scope));
					}

					module
				})
			})
	}
}

impl Default for Inner {
	fn default() -> Self {
		Self::new()
	}
}

fn resolve_callback<'a>(
	context: v8::Local<'a, v8::Context>,
	specifier: v8::Local<v8::String>,
	_: v8::Local<v8::FixedArray>,
	_referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
	let scope = &mut unsafe { v8::CallbackScope::new(context) };
	let scope = &mut v8::EscapableHandleScope::new(scope);
	let scope = &mut v8::ContextScope::new(scope, context);
	let specifier = specifier.to_rust_string_lossy(scope);
	let compiler = context.get_slot::<Compiler>().expect("current context");

	compiler
		.get_module(scope, &specifier)
		.or_else(|| compiler.load_module(scope, Some(specifier), None))
		.map(|module| scope.escape(module))
}
