use benedict_engine::uci::Uci;

fn main() {
    // Initialize attack tables eagerly
    benedict_engine::tables::tables();

    let mut uci = Uci::new();
    uci.run();
}
