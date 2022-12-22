use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};

    let config = Config {
        dll_name: "intercosmic_empire_core".to_string(),
        namespace_mappings: NamespaceMappings::new("AK.Scripts.Core.Native"),
        ..Config::default()
    };

    let inventory = IntercosmicEmpireCore::my_inventory();

    Generator::new(config, inventory)
        .write_file("bindings/csharp/Interop.cs")?;

    Ok(())
}