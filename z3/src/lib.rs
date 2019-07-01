#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::unreadable_literal)]

#[macro_use]
extern crate log;

extern crate z3_sys;

use std::ffi::CString;
use z3_sys::*;

pub mod ast;
mod config;
mod context;
mod datatype_builder;
mod func_decl;
mod model;
mod optimize;
mod solver;
mod sort;
mod symbol;

/// Configuration used to initialize [logical contexts].
///
/// [logical contexts]: struct.Context.html
pub struct Config {
    kvs: Vec<(CString, CString)>,
    z3_cfg: Z3_config,
}

/// Manager of all other Z3 objects, global configuration options, etc.
///
/// An application may use multiple Z3 contexts. Objects created in one context
/// cannot be used in another one. However, several objects may be "translated" from
/// one context to another. It is not safe to access Z3 objects from multiple threads.
/// The only exception is the method [`interrupt()`] that can be used to interrupt a long
/// computation.
///
/// # Examples:
///
/// Creating a context with the default configuration:
///
/// ```
/// use z3::{Config, Context};
/// let cfg = Config::new();
/// let ctx = Context::new(&cfg);
/// ```
///
/// [`interrupt()`]: #method.interrupt
pub struct Context {
    z3_ctx: Z3_context,
}

/// Symbols are used to name several term and type constructors.
pub enum Symbol {
    Int(u32),
    String(String),
}

/// Sorts represent the various 'types' of [`Ast`s](trait.Ast.html).
pub struct Sort<'ctx> {
    ctx: &'ctx Context,
    z3_sort: Z3_sort,
}

/// (Incremental) solver, possibly specialized by a particular tactic or logic.
pub struct Solver<'ctx> {
    ctx: &'ctx Context,
    z3_slv: Z3_solver,
}

/// Model for the constraints inserted into the logical context.
pub struct Model<'ctx> {
    ctx: &'ctx Context,
    z3_mdl: Z3_model,
}

/// Context for solving optimization queries.
pub struct Optimize<'ctx> {
    ctx: &'ctx Context,
    z3_opt: Z3_optimize,
}

/// Function declaration. Every constant and function have an associated declaration.
///
/// The declaration assigns a name, a sort (i.e., type), and for function
/// the sort (i.e., type) of each of its arguments. Note that, in Z3,
/// a constant is a function with 0 arguments.
pub struct FuncDecl<'ctx> {
    ctx: &'ctx Context,
    z3_func_decl: Z3_func_decl,
}

/// Build a datatype sort.
///
/// Example:
/// ```
/// # use z3::{ast::Int, Config, Context, DatatypeBuilder, Solver, Sort, ast::{Ast, Datatype}};
/// # let cfg = Config::new();
/// # let ctx = Context::new(&cfg);
/// # let solver = Solver::new(&ctx);
/// // Like Rust's Option<int> type
/// let option_int = DatatypeBuilder::new(&ctx)
///         .variant("None", &[])
///         .variant("Some", &[("value", &ctx.int_sort())])
///         .finish("OptionInt");
///
/// // Assert x.is_none()
/// let x = Datatype::new_const(&ctx, "x", &option_int.sort);
/// solver.assert(&option_int.variants[0].tester.apply(&[&x.into()]).as_bool().unwrap());
///
/// // Assert y == Some(3)
/// let y = Datatype::new_const(&ctx, "y", &option_int.sort);
/// let value = option_int.variants[1].constructor.apply(&[&Int::from_i64(&ctx, 3).into()]);
/// solver.assert(&y._eq(&value.as_datatype().unwrap()));
///
/// assert!(solver.check());
/// let model = solver.get_model();
///
/// // Get the value out of Some(3)
/// let ast = option_int.variants[1].accessors[0].apply(&[&y.into()]);
/// assert_eq!(3, model.eval(&ast.as_int().unwrap()).unwrap().as_i64().unwrap());
/// ```
pub struct DatatypeBuilder<'ctx> {
    ctx: &'ctx Context,
    // num_fields and constructor
    variants: Vec<(usize, Z3_constructor)>,
}

pub struct DatatypeVariant<'ctx> {
    pub constructor: FuncDecl<'ctx>,
    pub tester: FuncDecl<'ctx>,
    pub accessors: Vec<FuncDecl<'ctx>>,
}

pub struct DatatypeSort<'ctx> {
    ctx: &'ctx Context,
    pub sort: Sort<'ctx>,
    pub variants: Vec<DatatypeVariant<'ctx>>,
}
