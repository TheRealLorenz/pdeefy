use headless_chrome::Browser;

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
pub struct Renderer(Browser);

impl Renderer {
    pub fn new(browser: Browser) -> Self {
        Self(browser)
    }

    pub fn html_to_bytes(&self, html: &str) -> Result<Vec<u8>, anyhow::Error> {
        let tab = self.0.new_tab()?;
        tab.set_content(html)?;

        let bytes = tab.print_to_pdf(None)?;
        Ok(bytes)
    }

    pub fn url_to_bytes(&self, url: &str) -> Result<Vec<u8>, anyhow::Error> {
        let tab = self.0.new_tab()?;
        tab.navigate_to(url)?.wait_until_navigated()?;

        let bytes = tab.print_to_pdf(None)?;
        Ok(bytes)
    }
}
