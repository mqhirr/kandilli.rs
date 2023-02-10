use kandilli::deprem::Deprem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deprem = Deprem::en_son_olan()?;
    println!("deprem detayları: {:#?}", deprem);
    
    Ok(())
}