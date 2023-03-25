use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic, Context};

pub struct LuaRuntime {
    runtime: Lua,
}

impl LuaRuntime {
    pub fn new() -> Self {
        Self {
            runtime: Lua::new(),
        }
    }

    pub fn exec<F>(&self, lua_code: String, lambda: F) where F: FnOnce(Context) -> () {
        self.runtime.context(|ctx| {
            let globals = ctx.globals();
            let log = ctx.create_function(|_, msg: String| {
                eprintln!("{:#?}", msg);
                Ok(())
            }).unwrap();

            globals.set("log", log);

            lambda(ctx);
            ctx.load(&lua_code).exec();
        });
    }
}
