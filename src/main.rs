use clap::Parser;
use clap::ValueHint;
use thirtyfour::prelude::*;
use url::Url;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, value_name = "URL", value_hint = ValueHint::Url, default_value = "http://localhost:4444")]
    webdriver_server: Url,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let caps = DesiredCapabilities::firefox();
    let server_url = args.webdriver_server;
    let driver = WebDriver::new(server_url, caps).await?;

    // Navigate to https://wikipedia.org.
    driver.goto("https://wikipedia.org").await?;

    // Find element.
    let elem_form = driver.find(By::Id("search-form")).await?;

    // Find element from element.
    let elem_text = elem_form.find(By::Id("searchInput")).await?;

    // Type in the search terms.
    elem_text.send_keys("selenium").await?;

    // Click the search button.
    let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
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
