use scraper::Html;
use parse::Parse;
use std::io;
use serde::Serialize;

pub struct XmlSelector<T, F>
where
    F: FnOnce(Html) -> Result<T, io::Error> + 'static,
{
    convert: F,
}

impl<T, F> Parse<T, io::Error, Html, F> for XmlSelector<T, F>
where
    T: Serialize,
    F: FnOnce(Html) -> Result<T, io::Error> + 'static,
{
    fn new(convert: F) -> Self {
        XmlSelector { convert }
    }

    fn parse<S: io::Read>(self, mut buf: S) -> Result<T, io::Error> {
        let mut s = String::new();
        buf.read_to_string(&mut s)?;
        let document = Html::parse_document(&s);
        (self.convert)(document)
    }
}

#[cfg(test)]
mod tests {
    use parse::xml::XmlSelector;
    use parse::Parse;
    use scraper::Selector;
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

        // awesome error handling; see xml.rs for simplier approach
        let product = XmlSelector::new(|document| {
            let title_selector = Selector::parse("#title").unwrap();
            let title = document
                .select(&title_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .to_owned();
            let image_selector = Selector::parse("#img").unwrap();
            let image = document
                .select(&image_selector)
                .next()
                .unwrap()
                .value()
                .attr("src")
                .unwrap()
                .to_owned();
            let price_selector = Selector::parse(".price").unwrap();
            let price = Currency::from_str(
                document
                    .select(&price_selector)
                    .next()
                    .unwrap()
                    .text()
                    .next()
                    .unwrap(),
            ).unwrap()
                .value()
                .to_f64()
                .unwrap() / 100.0;

            Ok(Product {
                title,
                image,
                price,
            })
        }).parse(Cursor::new(xml_data))
            .unwrap();

        assert_eq!(product, {
            Product {
                title: "Title".to_string(),
                image: "image.png".to_string(),
                price: 9.99,
            }
        });
    }
}
