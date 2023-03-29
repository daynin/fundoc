use rlua::{Function, Lua, Result};

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
            let log = ctx
                .create_function(|_, msg: String| {
                    eprintln!("{:#?}", msg);
                    Ok(())
                })
                .unwrap();
            if let Err(err) = globals.set("log", log) {
                eprintln!("{:#?}", err)
            }

            if let Err(err) = ctx.load(&lua_code).exec() {
                eprintln!("{:#?}", err)
            }
        });
    }

    pub fn call_transform(&self, text: String) -> Result<String> {
        self.runtime.context(|ctx| {
            let transform: Function = ctx.globals().get("transform")?;

            if let Err(err) = transform.call::<_, ()>(text) {
                eprintln!("{:#?}", err)
            }

            ctx.globals().get::<_, String>("result")
        })
    }
}
