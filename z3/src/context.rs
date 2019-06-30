use ast;
use z3_sys::*;
use Config;
use Context;
use FuncDecl;
use Sort;
use Symbol;

impl Context {
    pub fn new(cfg: &Config) -> Context {
        Context {
            z3_ctx: unsafe {
                let p = Z3_mk_context_rc(cfg.z3_cfg);
                debug!("new context {:p}", p);
                p
            },
        }
    }

    /// Interrupt a solver performing a satisfiability test, a tactic processing a goal, or simplify functions.
    ///
    /// This method can be invoked from a thread different from the one executing the
    /// interruptible procedure.
    pub fn interrupt(&self) {
        unsafe {
            Z3_interrupt(self.z3_ctx);
        }
    }

    // Helpers for common constructions

    pub fn bool_sort(&self) -> Sort {
        Sort::bool(self)
    }

    pub fn int_sort(&self) -> Sort {
        Sort::int(self)
    }

    pub fn real_sort(&self) -> Sort {
        Sort::real(self)
    }

    pub fn bitvector_sort(&self, sz: u32) -> Sort {
        Sort::bitvector(self, sz)
    }

    pub fn array_sort<'ctx>(&'ctx self, domain: &Sort<'ctx>, range: &Sort<'ctx>) -> Sort<'ctx> {
        Sort::array(self, domain, range)
    }

    pub fn set_sort<'ctx>(&'ctx self, elt: &Sort<'ctx>) -> Sort<'ctx> {
        Sort::set(self, elt)
    }

    /// Create an enumeration sort.
    ///
    /// Creates a Z3 enumeration sort with the given `name`.
    /// The enum variants will have the names in `enum_names`.
    /// Three things are returned:
    /// - the created `Sort`,
    /// - constants to create the variants,
    /// - and testers to check if a value is equal to a variant.
    ///
    /// # Examples
    /// ```
    /// # use z3::{Config, Context, Solver, Symbol};
    /// # let cfg = Config::new();
    /// # let ctx = Context::new(&cfg);
    /// # let solver = Solver::new(&ctx);
    /// let (colors, color_consts, color_testers) = ctx.enumeration_sort(
    ///     "Color".into(),
    ///     &[
    ///         "Red".into(),
    ///         "Green".into(),
    ///         "Blue".into(),
    ///     ],
    /// );
    ///
    /// let red_const = color_consts[0].apply(&[]);
    /// let red_tester = &color_testers[0];
    /// let eq = red_tester.apply(&[&red_const]);
    ///
    /// assert!(solver.check());
    /// let model = solver.get_model();
    ///
    /// assert!(model.eval(&eq).unwrap().as_bool().unwrap().as_bool().unwrap());
    /// ```
    pub fn enumeration_sort<'ctx>(
        &'ctx self,
        name: Symbol,
        enum_names: &[Symbol],
    ) -> (Sort<'ctx>, Vec<FuncDecl<'ctx>>, Vec<FuncDecl<'ctx>>) {
        Sort::enumeration(self, name, enum_names)
    }

    pub fn func_decl<'ctx, S: Into<Symbol>>(
        &'ctx self,
        name: S,
        domain: &[&Sort<'ctx>],
        range: &Sort<'ctx>,
    ) -> FuncDecl<'ctx> {
        FuncDecl::new(self, name, domain, range)
    }

    /// Create a forall quantifier.
    ///
    /// # Examples
    /// ```
    /// # use z3::{ast, Config, Context, Solver, Symbol};
    /// # use z3::ast::Ast;
    /// # use std::convert::TryInto;
    /// # let cfg = Config::new();
    /// # let ctx = Context::new(&cfg);
    /// # let solver = Solver::new(&ctx);
    /// let f = ctx.func_decl("f", &[&ctx.int_sort()], &ctx.int_sort());
    ///
    /// let x = ast::Int::new_const(&ctx, "x");
    /// let f_x: ast::Int = f.apply(&[&x.clone().into()]).try_into().unwrap();
    /// let forall: ast::Dynamic = ctx.forall_const(&[&x.clone().into()], &(x._eq(&f_x)).into());
    /// solver.assert(&forall.try_into().unwrap());
    ///
    /// assert!(solver.check());
    /// let model = solver.get_model();
    ///
    /// let f_f_3: ast::Int = f.apply(&[&f.apply(&[&ast::Int::from_u64(&ctx, 3).into()])]).try_into().unwrap();
    /// assert_eq!(3, model.eval(&f_f_3).unwrap().as_u64().unwrap());
    /// ```
    pub fn forall_const<'ctx>(
        &'ctx self,
        bounds: &[&ast::Dynamic<'ctx>],
        body: &ast::Dynamic<'ctx>,
    ) -> ast::Dynamic<'ctx> {
        ast::forall_const(self, bounds, body)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { Z3_del_context(self.z3_ctx) };
    }
}
