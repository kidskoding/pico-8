use rlua::{Lua, RluaCompat};

fn run_lua_script() {
    let lua = Lua::new();
    lua.context(|ctx| {
        ctx.load(r#"
            function _update()
                print("Hello, World!")
            end
        "#).exec().unwrap();
    })
}