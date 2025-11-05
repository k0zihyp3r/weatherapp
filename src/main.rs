let url = format!( "https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units=metric" );
let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;


fn main() {
}
