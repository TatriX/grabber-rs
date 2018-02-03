use scraper::{Html, Selector};
use scraper::element_ref::ElementRef;
use parse::Parse;
use std::io;
use serde::Serialize;

pub struct Xml<T, F>
where
    F: FnOnce(SimpleHtml) -> Result<T, io::Error> + 'static,
{
    convert: F,
}

impl<T, F> Parse<T, io::Error, SimpleHtml, F> for Xml<T, F>
where
    T: Serialize,
    F: FnOnce(SimpleHtml) -> Result<T, io::Error> + 'static,
{
    fn new(convert: F) -> Self {
        Xml { convert }
    }

    fn parse<S: io::Read>(self, mut buf: S) -> Result<T, io::Error> {
        let mut s = String::new();
        buf.read_to_string(&mut s)?;
        let document = Html::parse_document(&s);
        (self.convert)(SimpleHtml { document })
    }
}

pub struct SimpleHtml {
    document: Html,
}

impl SimpleHtml {
    pub fn select(&self, css_selector: &str) -> SimpleElement {
        let selector = Selector::parse(css_selector).unwrap();
        SimpleElement {
            elem: self.document.select(&selector).next(),
        }
    }
}

pub struct SimpleElement<'a> {
    elem: Option<ElementRef<'a>>,
}

impl<'a> SimpleElement<'a> {
    pub fn text(&self) -> Option<String> {
        self.elem
            .and_then(|elem| elem.text().next())
            .map(String::from)
    }

    pub fn attr(&self, attr_name: &str) -> Option<String> {
        self.elem
            .and_then(|elem| elem.value().attr(attr_name))
            .map(String::from)
    }
}

#[cfg(test)]
mod tests {
    use parse::xml::Xml;
    use parse::Parse;
    use currency::Currency;
    use num_traits::cast::ToPrimitive;
    use std::io::Cursor;

    #[test]
    fn parse() {
        let xml_data = r#"
            <html>
             <body>
              <div id="title">Title</div>
              <img id="img" src="image.png">
              <span class="price">$9.99</span>
             </body>
            </html>
        "#;

        #[derive(Debug, PartialEq, Serialize)]
        struct Product {
            title: String,
            image: String,
            price: f64,
        }

        let parser = Xml::new(|document| {
            let price_string = document.select(".price").text().unwrap();
            let price = Currency::from_str(&price_string)
                .unwrap()
                .value()
                .to_f64()
                .unwrap() / 100.0;

            Ok(Product {
                title: document.select("#title").text().unwrap(),
                image: document.select("#img").attr("src").unwrap(),
                price,
            })
        });
        let product = parser.parse(Cursor::new(xml_data)).unwrap();

        assert_eq!(product, {
            Product {
                title: "Title".to_string(),
                image: "image.png".to_string(),
                price: 9.99,
            }
        });
    }
}
