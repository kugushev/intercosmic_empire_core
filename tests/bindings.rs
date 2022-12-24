use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    let config = Config {
        dll_name: "intercosmic_empire".to_string(),
        namespace_mappings: NamespaceMappings::new("AK.Scripts.Core.Native")
            .add("UnityEngine", "UnityEngine"),
        ..Config::default()
    };

    let inventory = intercosmic_empire::my_inventory();

    Generator::new(config, inventory)
        .write_file("bindings/csharp/Interop.cs")?;

    Ok(())
}