use once_cell::sync::Lazy;

pub static TERA_ENGINE: Lazy<tera::Tera> =
    Lazy::new(|| match tera::Tera::new("templates/*.html") {
        Ok(engine) => {
            // let names: Vec<&str> = engine.get_template_names().collect();
            // println!("{:#?}", names);
            engine
        }
        Err(e) => {
            panic!("Error loading tera engine:\n{}", e)
        }
    });
