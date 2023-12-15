use open::that;
use colored::Colorize;
use local_ip_address::local_ip;

pub async fn launch_browser(url:&str)->Result<(), std::io::Error>{
   let my_local_ip = local_ip().unwrap();
   match that(url) {
      Ok(()) => {
         println!(" Local: {}",format!("{url}").cyan());
         println!(" Network: {}",format!("http://{my_local_ip:?}:8080/").cyan());
      },
      Err(_) => println!(" {} An error occurred when opening {}", " ERROR ".on_red().color("white"),format!("{url}").cyan())
   }
   Ok(())
}