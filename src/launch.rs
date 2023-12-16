use open::that;
use colored::Colorize;
use local_ip_address::local_ip;

pub async fn launch_browser(url:&str)->Result<(), std::io::Error>{
   match that(url) {
      Ok(()) => {
         let my_local_ip = local_ip();
         println!(" Local: {}",format!("{url}").cyan());
         match my_local_ip {
            Ok(ip) => {
               println!(" Network: {}",format!("http://{ip}:8080/").cyan());
            },
            Err(e) => println!(" {} {}.", format!(" WARNING ").on_yellow().color("white"),e )
         }
      },
      Err(_) => println!(" {} An error occurred when opening {}", " ERROR ".on_red().color("white"),format!("{url}").cyan())
   }
   Ok(())
}