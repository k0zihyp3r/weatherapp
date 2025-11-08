use iced::{
    Alignment, Backward, Sandbox, Settings,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherResponse{
    weather: Vec<WeatherDesc>,
    main: WeatherParameters,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct WeatherDesc{
    description: String,

}

#[derive(Deserialize, Debug)]
struct WeatherParameters{
    temp: f64,
    pressure: f64,
    humidity: f64,
}

#[derive(Deserialize, Debug)]
struct Wind{
    speed:f64,
}

fn get_weather(city: &str, api_key: &str) ->
    Result<WeatherResponse, reqwest::Error>{
    let url: String = format!( "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);
    let resp = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = resp.json::<WeatherResponse>()?;
    Ok(response_json)

}

fn display_weather_info(response: &WeatherResponse){
    let description: &String = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;
}

struct RustUI{
    theme: Theme,
    city_field: CityField
}

struct CityField {city: String}
fn main() -> iced::Result{

    RustUI::run(Settings::default())
    
}
