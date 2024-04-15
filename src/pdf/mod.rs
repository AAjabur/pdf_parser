mod page;
mod catalog;

struct Pdf {
    catalog: catalog::Catalog,
}

impl Pdf {
    fn from_file() -> Pdf {
        Pdf {
            catalog: catalog::Catalog {
                
            }
        }
    }
}