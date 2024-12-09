fn main() {
    let na_client_url = "https://lol.secure.dyn.riotcdn.net/channels/public/x/installer/current/live.na.exe";
    let response = reqwest::blocking::get(na_client_url)
        .expect("Failed to download installer");
    let client = response.bytes().expect("response invalid");
    let _ = std::fs::write("lolinstall.exe", &client);
}
