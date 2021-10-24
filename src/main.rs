extern crate osascript;

#[allow(unused_must_use)]
fn main() {
    osascript::JavaScript::new("
        var App = Application('Spotify');
        App.includeStandardAdditions = true;
        App.playpause();
        return;
    ").execute::<()>();
}
