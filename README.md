# webdriver-experiment

Experimenting with WebDriver automation in Rust

* Enable `networkingMode = mirrored` in `%USERPROFILE%/.wslconfig`

  https://learn.microsoft.com/en-us/windows/wsl/wsl-config#configuration-settings-for-wslconfig
* Download a matching version for your Firefox and run `geckodriver.exe`

  https://github.com/mozilla/geckodriver/releases

* User WebDriver with URL http://localhost:4444

```terminal
$ cargo run
```

Internet Explorer also had a driver, there's something for Servo. All classic, not BiDi.

Dealing with matching drivers and evergreen browsers is a pain, at least there's some automated tooling:

https://www.selenium.dev/documentation/selenium_manager/

https://github.com/SeleniumHQ/selenium/tree/trunk/rust

This usually does not supporting running the driver+browser on the WSL host though.


There's no such thing as a default driver URL, as they use different ports.

https://github.com/mozilla/geckodriver/releases

https://firefox-source-docs.mozilla.org/testing/geckodriver/Flags.html#p-port-port-port


https://developer.apple.com/documentation/webkit/testing-with-webdriver-in-safari

https://sites.google.com/chromium.org/driver/

https://sites.google.com/chromium.org/driver/getting-started


https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/

https://learn.microsoft.com/en-us/microsoft-edge/webdriver/


Note however that the drivers usually create new profiles or are isolated in general,
so one can't comfortably use existing session cookies for webapps that don't have an API.

There's tools to extract cookies from installed browsers, but those behave malware adjacent:

https://lib.rs/crates/rookie

https://lib.rs/crates/decrypt-cookies

https://lib.rs/crates/cookie-scoop
