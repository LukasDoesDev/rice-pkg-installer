use argparse::{ArgumentParser, Store};
use std::fs;
mod datatypes;
mod select;
use std::process::Command;

fn main() {
    let mut filename = "./config.yaml".to_string();
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut filename).add_argument(
            "filename",
            Store,
            "Config file name (default config.yaml)",
        );
        ap.parse_args_or_exit();
    }
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let deser: datatypes::Root = serde_yaml::from_str(&contents).expect("Error while parsing YAML");
    //println!("{:#?}", deser);

    let item_names: Vec<String> = deser.packages.iter().map(|x| x.name.clone()).collect();
    let selected_item_names: Vec<String> = select::select_items(item_names);
    //println!("Selected items: {:#?}", selected_item_names);
    let selected_items: Vec<datatypes::Package> = deser
        .packages
        .into_iter()
        .filter(|x| selected_item_names.contains(&x.name))
        .collect();

    let mut errors: Vec<String> = vec![];

    {
        println!("Synchronizing databases");
        Command::new("sudo")
            .args(["pacman", "--noconfirm", "-Sy"])
            .output()
            .expect("Failed to synchronize pacman databases");
        if let Err(_x) = Command::new("sudo")
            .args(["pacman", "--noconfirm", "-Sy"])
            .output()
        {
            errors.push(String::from("Failed to synchronize pacman databases"))
        }
    }

    {
        for item in selected_items {
            for module in item.modules {
                if let Err(new_errors) = module.execute() {
                    errors.extend(new_errors);
                }
            }
        }
    }

    println!("Errors: {:?}", errors);

    println!("Bye!");

    ()
}
