use open::that;

pub async fn launch_browser(port:i32)->Result<(),std::io::Error>{
   let url=format!("http://localhost:{port}/");
   that(&url)
}