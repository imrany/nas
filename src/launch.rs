use chromiumoxide::browser::{
    Browser, 
    BrowserConfig
};
use anyhow::{
    Result,
};
use futures::StreamExt;
use async_std;

// #[async_std::main]
pub async fn launch_browser(url:&str)->Result<(), std::io::Error>{
    // create a `Browser` that spawns a `chromium` process running with UI (`with_head()`, headless is default) 
   // and the handler that drives the websocket etc.
   let (browser, mut handler) =
    Browser::launch(BrowserConfig::builder().with_head().build().unwrap()).await.unwrap();

    // spawn a new task that continuously polls the handler
    let handle = async_std::task::spawn(async move {
        loop {
            let _event = handler.next().await.unwrap();
        }
    });

    // create a new browser page and navigate to the url
    browser.new_page(url).await.unwrap();
    handle.await;
    Ok(())
}