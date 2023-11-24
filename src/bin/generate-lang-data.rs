use repofetch::files;

fn main() -> Result<(), Box<dyn Error>> {
    files::write_langs_file()?;

    Ok(())
}
