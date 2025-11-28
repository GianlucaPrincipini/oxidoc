use server::network::listener::start_listener;

const PORT: u16 = 7878;
const ADDRESS: &str = "127.0.0.1";

fn main() -> std::io::Result<()> {
    start_listener(ADDRESS, PORT)
}
