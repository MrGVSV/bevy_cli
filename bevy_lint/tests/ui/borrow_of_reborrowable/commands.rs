//! This tests the `panicking_query_methods` lint, specifically when triggered on the `Query` type.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::borrow_of_commands)]

use bevy::prelude::*;

//~v ERROR: foo
fn helper_function(commands: &mut Commands) {
    commands.spawn_empty();
}

fn main() {}
