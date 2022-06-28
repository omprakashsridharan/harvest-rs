use harvest::settings::SETTINGS;

fn main() {
    let database = &SETTINGS.read().unwrap().database;
    println!("Database settings {:?}", database);
}
