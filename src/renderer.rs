use headless_chrome::{types::PrintToPdfOptions, Browser, LaunchOptionsBuilder};

trait EditableTab {
    fn set_content<T: AsRef<str>>(&self, content: T) -> Result<(), anyhow::Error>;
}

impl EditableTab for headless_chrome::Tab {
    fn set_content<T: AsRef<str>>(&self, content: T) -> Result<(), anyhow::Error> {
        // From: https://github.com/rust-headless-chrome/rust-headless-chrome/issues/336#issuecomment-1285209798
        self.evaluate(
            &format!(
                r#"(function() {{
                    let html = `{}`;

                    document.open();
                    document.write(html);
                    document.close();

                    }})()"#,
                content.as_ref()
            ),
            false,
        )?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Renderer<'a> {
    options: LaunchOptionsBuilder<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(options: LaunchOptionsBuilder<'a>) -> Self {
        Self { options }
    }
}

impl Renderer<'_> {
    fn instance(&self) -> Result<Browser, anyhow::Error> {
        Browser::new(self.options.build()?)
    }

    pub fn html_to_bytes(
        &self,
        html: &str,
        options: Option<PrintToPdfOptions>,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let tab = self.instance()?.new_tab()?;
        tab.set_content(html)?;

        let bytes = tab.print_to_pdf(options)?;
        Ok(bytes)
    }

    pub fn url_to_bytes(
        &self,
        url: &str,
        options: Option<PrintToPdfOptions>,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let tab = self.instance()?.new_tab()?;
        tab.navigate_to(url)?.wait_until_navigated()?;

        let bytes = tab.print_to_pdf(options)?;
        Ok(bytes)
    }
}
