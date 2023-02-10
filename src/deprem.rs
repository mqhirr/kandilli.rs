use serde::{Serialize, Deserialize};

///
/// [Deprem]: Kandilli Rasathanesi'nden gelen verileri işleyen bir struct'dır.
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct Deprem {
    pub il: String,
    pub ilce: String,
    pub tarih: String,

    /// Nanosaniye olarak kaydedilir.
    pub saat: i64,

    pub derinlik: f32,
    pub buyukluk: f32
}

impl Deprem {
    ///
    /// Kandilli Rasathanesi'nden en son yaşanan depremin verisini çeker. </br></br>
    /// Örnek kullanım:
    /// ```
    /// use kandilli::deprem::Deprem;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let deprem = Deprem::en_son_olan()?;
    ///     println!("deprem detayları: {:#?}", deprem);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn en_son_olan() -> Result<Self, Box<dyn std::error::Error>> {
        let req = Some(ureq::get("http://www.koeri.boun.edu.tr/scripts/lst0.asp").call()?.into_string()?);
        
        if req.is_some() {
            let resp = req.unwrap();
            let dom = tl::parse(resp.as_str(), tl::ParserOptions::default())?;
            let parser = dom.parser();

            let element = dom
                .query_selector("pre")
                .unwrap()
                .next()
                .unwrap()
                .get(parser)
                .unwrap();
            let ihtml = element.inner_html(parser);
            let split = &ihtml
                .as_ref()
                .clone()
                .split("\n")
                .collect::<Vec<&str>>()[7..][0]
                .split(" ")
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>();
    
            return Ok(Self {
                il: split[9].to_owned().replace("(", "").replace(")", ""),
                ilce: split[8].to_owned(),
                saat: dateparser::parse(split[1])?.timestamp_nanos() / 1000000000,
                derinlik: split[4].parse()?,
                buyukluk: split[6].parse()?,
                tarih: split[0].to_owned()
            });
        } else {
            return Err("Hata: veri alınamadı.".into());
        }
    }

    /// Kandilli Rasathanesi'nden en son yaşanan `sayi` kadar depremin verisini çeker. </br></br>
    /// Örnek kullanım:
    /// ```
    /// use kandilli::deprem::Deprem;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let deprem = Deprem::en_son_olanlar(5)?;
    ///     println!("en son yaşanan 5 depremin detayları: {:#?}", deprem);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn en_son_olanlar(sayi: usize) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let req = Some(ureq::get("http://www.koeri.boun.edu.tr/scripts/lst0.asp").call()?.into_string()?);
        
        if req.is_some() {
            let mut depremler: Vec<Self> = Vec::new();
            let resp = req.unwrap();
        
            let dom = tl::parse(resp.as_str(), tl::ParserOptions::default()).unwrap();
        
            let parser = dom.parser();
            let element = dom.query_selector("pre").unwrap().next().unwrap().get(parser).unwrap();
    
            let ihtml = element.inner_html(parser);
            let split = &ihtml.as_ref().clone().split("\n").collect::<Vec<&str>>()[7..];
        
            for i in 0..sayi {
                let dstr = split[i].split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
    
                depremler.push(
                    Self { 
                        il: dstr[9].to_owned().replace("(", "").replace(")", ""), 
                        ilce: dstr[8].to_owned(), 
                        saat: dateparser::parse(dstr[1])?.timestamp_nanos() / 1000000000,
                        derinlik: dstr[4].parse()?, 
                        buyukluk: dstr[6].parse()?,
                        tarih: dstr[0].to_owned()
                    }
                );
            }
    
            Ok(depremler)
        } else {
            return Err("Hata: veri alınamadı.".into());
        }
    }
}