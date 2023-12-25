use open::that;
use local_ip_address::local_ip;

#[cfg(target_os="linux")]
use colored::Colorize;

pub async fn launch_browser(url:&str)->Result<(), std::io::Error>{
   match that(url) {
      Ok(()) => {
         let my_local_ip = local_ip();
         #[cfg(target_os="linux")]
         println!(" Local: {}",format!("{url}").cyan());

         #[cfg(target_os="windows")]
         println!(" Local: {url}");
         
         match my_local_ip {
            Ok(ip) => {
               #[cfg(target_os="linux")]
               println!(" Network: {}",format!("http://{ip}:8080/").cyan());

               #[cfg(target_os="windows")]
               println!(" Network: {}",format!("http://{ip}:8080/"));
            },
            Err(e) => {
               #[cfg(target_os="linux")]
               println!(" {} {}.", format!(" WARNING ").on_yellow().color("white"),e );

               #[cfg(target_os="windows")]
               println!(" {} {}.", format!(" WARNING "),e );
            }
         }
      },
      Err(_) => {
         #[cfg(target_os="linux")]
         println!(" {} An error occurred when opening {}", " ERROR ".on_red().color("white"),format!("{url}").cyan());

         #[cfg(target_os="windows")]
         println!(" ERROR An error occurred when opening {url}");
      }
   }
   Ok(())
}