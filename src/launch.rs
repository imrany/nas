use open::that;

pub async fn launch_browser(url:&str)->Result<(), std::io::Error>{
   match that(url) {
    Ok(()) => println!("Serving on {url}"),
    Err(e) => println!("An error occurred when opening {url} : {e}")
   }
   Ok(())
}