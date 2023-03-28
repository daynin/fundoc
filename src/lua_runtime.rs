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

    pub fn exec(&self, lua_code: String) {
        self.runtime.context(|ctx| {
            let globals = ctx.globals();
            let log = ctx.create_function(|_, msg: String| {
                eprintln!("{:#?}", msg);
                Ok(())
            }).unwrap();

            globals.set("log", log);

            ctx.load(&lua_code).exec();
        });
    }

    pub fn call_transform(&self, text: String) -> Result<String> {
        self.runtime.context(|ctx| {
            let transform: Function = ctx.globals().get("transform")?;
            transform.call::<_, ()>(text);

            ctx.globals().get::<_, String>("result")
        })
    }
}
