use crate::entity::*;
use crate::game::*;

//================================================================

use mlua::prelude::*;

//================================================================

pub struct Script {
    pub lua: Lua,
    pub meta: Meta,
}

impl Script {
    pub fn new(game: &Game) -> Self {
        let lua = Lua::new_with(LuaStdLib::ALL_SAFE, LuaOptions::new()).unwrap();

        let global = lua.globals();
        let brushy = lua.create_table().unwrap();

        Self::system(&lua, &brushy);

        global.set("brushy", brushy).unwrap();

        lua.set_app_data(Meta::default());

        let package = global.get::<mlua::Table>("package").unwrap();
        let path = package.get::<mlua::String>("path").unwrap();
        package
            .set("path", format!("{path:?};{}/?.lua", game.path))
            .unwrap();

        lua.load("require \"main\"").exec().unwrap();

        let meta = lua.remove_app_data::<Meta>().unwrap();

        Self { lua, meta }
    }

    fn system(lua: &Lua, table: &mlua::Table) {
        table
            .set("entity", lua.create_function(Self::entity).unwrap())
            .unwrap();
    }

    fn entity(lua: &Lua, entity: LuaValue) -> mlua::Result<()> {
        let mut app = lua.app_data_mut::<Meta>().unwrap();

        app.entity.push(lua.from_value(entity)?);
        Ok(())
    }
}

//================================================================

#[derive(Default)]
pub struct Meta {
    pub entity: Vec<EntityMeta>,
}
