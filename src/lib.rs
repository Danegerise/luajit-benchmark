use mlua::prelude::*;

#[inline(always)]
pub fn exec_inner(s: &str) {
    let lua = unsafe { Lua::unsafe_new() };
    lua.load(s).exec().unwrap();
}

#[macro_export]
macro_rules! exec {
    ($file:expr) => {
        luajit::exec_inner(include_str!($file))
    };
}
