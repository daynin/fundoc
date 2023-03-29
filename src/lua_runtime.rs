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

            match globals.set("log", log) {
                Err(err) => eprintln!("{:#?}", err),
                _ => {}
            };

            match ctx.load(&lua_code).exec() {
                Err(err) => eprintln!("{:#?}", err),
                _ => {}
            };
        });
    }

    pub fn call_transform(&self, text: String) -> Result<String> {
        self.runtime.context(|ctx| {
            let transform: Function = ctx.globals().get("transform")?;

            match transform.call::<_, ()>(text) {
                Err(err) => eprintln!("{:#?}", err),
                _ => {}
            }

            ctx.globals().get::<_, String>("result")
        })
    }
}
