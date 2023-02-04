fn main() {
    let mut client = client::new();
    let server = server::new_sub_app();
    client.add_sub_app("server", server, server::runner);
    client.run();
}
