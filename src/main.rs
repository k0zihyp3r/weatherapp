use iced::{
    Alignment, Application, Command, Element, Settings,
    widget::{Button, Column, Text, TextInput}, button, text_input,
};
use serde::Deserialize;
use reqwest;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<WeatherDesc>,
    main: WeatherParameters,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct WeatherDesc {
    description: String,
}

#[derive(Deserialize, Debug)]
struct WeatherParameters {
    temp: f64,
    pressure: f64,
    humidity: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn get_weather(city: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let resp = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = resp.json::<WeatherResponse>()?;
    Ok(response_json)
}

#[derive(Default)]
pub struct WeatherApp {
    city_field: String,
    weather_data: Option<WeatherResponse>,
    city_input_state: text_input::State,
    button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    GetWeather,
    CityChanged(String),
}

impl Application for WeatherApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (WeatherApp::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Weather App")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::GetWeather => {
                if !self.city_field.is_empty() {
                    let api_key = "967a1be7a23f1be26b72e6d3a172bc8e"; 
                    match get_weather(&self.city_field, api_key) {
                        Ok(weather) => self.weather_data = Some(weather),
                        Err(_) => {
                            self.weather_data = None;
                        }
                    }
                }
                Command::none()
            }
            Message::CityChanged(city) => {
                self.city_field = city;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let city_input = TextInput::new(
            &mut self.city_input_state,
            "Enter city name",
            &self.city_field,
            Message::CityChanged,
        )
        .padding(10)
        .size(30);

        let get_weather_button = Button::new(
            &mut self.button_state,
            Text::new("Get Weather")
        ).on_press(Message::GetWeather)
        .padding(10);

        let weather_display = match &self.weather_data {
            Some(weather) => {
                Column::new()
                    .push(Text::new(format!("City: {}", weather.name)))
                    .push(Text::new(format!("Description: {}", weather.weather[0].description)))
                    .push(Text::new(format!("Temperature: {:.2} °C", weather.main.temp)))
                    .push(Text::new(format!("Humidity: {:.2} %", weather.main.humidity)))
                    .push(Text::new(format!("Pressure: {:.2} hPa", weather.main.pressure)))
                    .push(Text::new(format!("Wind Speed: {:.2} m/s", weather.wind.speed)))
            }
            None => Column::new().push(Text::new("Enter a city and press 'Get Weather'")),
        };

        Column::new()
            .align_items(Alignment::Center)
            .padding(20)
            .push(city_input)
            .push(get_weather_button)
            .push(weather_display)
            .into()
    }
}

fn main() -> iced::Result {
    let settings = Settings {
        window: iced::window::Settings {
            size: (400, 400),
            ..Default::default()
        },
        ..Default::default()
    };

    WeatherApp::run(settings)
}