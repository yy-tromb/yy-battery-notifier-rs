fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.get(0) {
        Some(arg1) => {
            if (arg1 == "-h") || (arg1 == "--help") {
                print_help()
            } else {
                hogehoge(&arg1)
            }
        }
        None => print_help(),
    }
}

fn print_help() -> anyhow::Result<()> {
    //hoge
    anyhow::Ok(())
}

fn hogehoge(toml_path: &str) -> anyhow::Result<()> {
    //hogehoge
    anyhow::Ok(())
}
