//! TODO
//!
//! # Motivation
//!
//! TODO
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! fn system(mut commands: Commands) {
//!   helper_function(&mut commands);
//! }
//!
//! fn helper_function(commands: &mut Commands) {
//!   // ...
//! }
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! fn system(mut commands: Commands) {
//!   helper_function(commands.reborrow());
//! }
//!
//! fn helper_function(mut commands: Commands) {
//!   // ...
//! }
//! ```

use crate::declare_bevy_lint;
use clippy_utils::diagnostics::{span_lint, span_lint_and_then};
use clippy_utils::ty::match_type;
use rustc_ast::{BindingMode, ByRef};
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, FnDecl, PatKind, TyKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::inherent::SliceLike;
use rustc_middle::ty::{Interner, Ty};
use rustc_session::declare_lint_pass;
use rustc_span::def_id::LocalDefId;
use rustc_span::Span;

declare_bevy_lint! {
    pub BORROW_OF_COMMANDS,
    RESTRICTION,
    "parameter takes a reference to `Commands` instead of a re-borrowed `Commands`",
}

declare_bevy_lint! {
    pub BORROW_OF_QUERY,
    RESTRICTION,
    "parameter takes a reference to `Query` instead of a re-borrowed `Query`",
}

declare_lint_pass! {
    BorrowOfReborrowable => [BORROW_OF_COMMANDS.lint, BORROW_OF_QUERY.lint]
}

impl<'tcx> LateLintPass<'tcx> for BorrowOfReborrowable {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        _: &'tcx Body<'tcx>,
        _: Span,
        _: LocalDefId,
    ) {
        for ty in decl.inputs {
            // FIXME: This is failing
            cx.typeck_results().node_type(ty.hir_id);
            span_lint(cx, BORROW_OF_COMMANDS.lint, ty.span, format!("{:#?}", ty));

            // FIXME: This is returning the type of the parent for some reason:
            let ty_ty = cx.tcx.type_of(ty.hir_id.owner).instantiate_identity();

            span_lint(cx, BORROW_OF_COMMANDS.lint, ty.span, format!("{:?}", ty_ty));

            // TODO: Handle the various reborrowable types
            // let Some(reborrowable) = Reborrowable::try_from_ty(cx, ty_ty) else {
            //     continue;
            // };
        }
    }
}

enum Reborrowable {
    Commands,
    Query,
}

impl Reborrowable {
    fn try_from_ty<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<Self> {
        if match_type(cx, ty, &crate::paths::COMMANDS) {
            Some(Self::Commands)
        } else if match_type(cx, ty, &crate::paths::QUERY) {
            Some(Self::Query)
        } else {
            None
        }
    }
}
