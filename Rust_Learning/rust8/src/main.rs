use chrono::{Local ,Utc};

fn main() {
    let now  = Utc::now();
    println!("Utc time is : {}" , now);
    let formateddatetime = now.format("%Y-%m-%d %H:%M:%S");
    println!("Utc Date and Time Formate wice print is : {}" , formateddatetime);
    println!("Local time is : {}" , Local::now());
}
