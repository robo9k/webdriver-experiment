use std::fs::File;
use std::io::Read as _;
use std::path::PathBuf;

use clap::Parser;
use clap::ValueHint;
use cookiestxt_rs::Cookie;
use cookiestxt_rs::Cookies;
use thirtyfour::By;
use thirtyfour::DesiredCapabilities;
use thirtyfour::WebDriver;
use url::Url;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, value_name = "URL", value_hint = ValueHint::Url, default_value = "http://localhost:4444")]
    webdriver_server: Url,

    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    cookies: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let caps = DesiredCapabilities::firefox();
    let server_url = args.webdriver_server;
    let driver = WebDriver::new(server_url, caps).await?;

    let cookies = if let Some(cookies) = args.cookies {
        let mut cookies_file = File::open(cookies)?;
        let mut cookie_txt = String::new();
        cookies_file.read_to_string(&mut cookie_txt)?;

        Cookies::try_from(cookie_txt.as_str())?
    } else {
        Cookies::default()
    };
    let cookies: Vec<Cookie> = cookies.into();

    driver.goto("https://en.wikipedia.org").await?;
    for c in cookies {
        let cookie = thirtyfour::Cookie {
            name: c.name,
            value: c.value,
            path: Some(c.path),
            domain: Some(c.domain),
            secure: Some(c.https_only),
            http_only: Some(c.http_only),
            expiry: Some(c.expires as i64),
            same_site: None, // FIXME: c.include_subdomains ?
        };
        driver.add_cookie(cookie).await?
    }

    // Navigate to https://en.wikipedia.org.
    driver.goto("https://en.wikipedia.org").await?;

    // Find element.
    let elem_form = driver.find(By::Id("searchform")).await?;

    // Find element from element.
    let elem_text = elem_form.find(By::Name("search")).await?;

    // Type in the search terms.
    elem_text.send_keys("selenium").await?;

    // Click the search button.
    let elem_button = elem_form.find(By::Tag("button")).await?;
    elem_button.click().await?;

    // Always explicitly close the browser. This prevents the executor from being blocked
    driver.quit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
