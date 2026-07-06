pub struct AppState {
    appmode: AppMode,
    editorstate: EditorState,
    gamestate: GameState,
    options: AppOptions,
    ///holds the path of the currently selected level
    selected_level: String,
}

pub enum AppMode {
    Menu(MenuMode),
    Editor,
    Game,
}
pub enum MenuMode {
    MainMenu,
    LevelSelect,
    Options,
}

///this will later have stuff like resolution and whatever
pub struct AppOptions {
    //empty for now teehee
}
impl AppOptions {
    pub fn new() -> AppOptions {
        AppOptions {}
    }
}
