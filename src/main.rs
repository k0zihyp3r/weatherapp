use std::io;
use serde::Deserialize;
use colored::*;

#[dervie(Deserialize, Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[dervie(Deserialize, Debug)]
struct WeatherDesc{
    description: String,

}

#[dervie(Deserialize, Debug)]
struct WeatherParameters{
    temp: f64,
    pressure: f64,
    humidity: f64,
}

#[dervie(Deserialize, Debug)]
struct Wind{
    speed:f64,
}

fn get_weather(city: &str, api_key: &str) ->
    Result<WeatherRespone, reqwest::Error>{
    let url: string = format!( "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api);
    let resp = reqwest::blocking::get(&url)?;
    let respone_json: WeatherResponse = Response.json::<WeatherResponse>()?;
    Ok(respone_json);

}
fn main() {

    
    
}
