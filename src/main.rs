use iced::{
    Alignment, Application, Command, Element, Settings,
    widget::{container, Button, Column, Text, TextInput, Image, Row},
    button, text_input,
};
use serde::Deserialize;
use reqwest;
use rand::prelude::IndexedRandom;

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
    icon: String,
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

struct OrangeButton;
impl button::StyleSheet for OrangeButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(1.0, 0.5, 0.0))),
            border_radius: 15.0,
            text_color: iced::Color::WHITE,
            ..Default::default()
        }
    }
}

struct DarkBg;
impl container::StyleSheet for DarkBg {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.15, 0.15, 0.15))),
            text_color: Some(iced::Color::WHITE),
            ..Default::default()
        }
    }
}


fn get_weather(city: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let resp = reqwest::blocking::get(&url)?;
    resp.json::<WeatherResponse>()
}

#[derive(Default)]
pub struct WeatherApp {
    city_field: String,
    weather_data: Option<WeatherResponse>,
    city_input_state: text_input::State,
    get_weather_button_state: button::State,
    fetch_random_weather_button_state: button::State,
    random_cities_weather: Vec<Option<WeatherResponse>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GetWeather,
    CityChanged(String),
    FetchRandomCitiesWeather,
}

impl Application for WeatherApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            WeatherApp::default(),
            Command::perform(async {}, |_| Message::FetchRandomCitiesWeather),
        )
    }

    fn title(&self) -> String {
        String::from("Weather App")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let api_key = "967a1be7a23f1be26b72e6d3a172bc8e";
        match message {
            Message::GetWeather => {
                if !self.city_field.is_empty() {
                    self.weather_data = get_weather(&self.city_field, api_key).ok();
                }
                Command::none()
            }
            Message::CityChanged(city) => {
                self.city_field = city;
                Command::none()
            }
            Message::FetchRandomCitiesWeather => {
                let cities = vec!["London", "Paris", "New York", "Tokyo", "Sydney", "Stockholm", "Gotland", "Warsaw", "Krakow", "Bialystok", "Moscow", "Kyiv", "Minsk", "Los Angeles", "Addis Abeba"];
                let mut rng = rand::rng();
                
                self.random_cities_weather = cities
                    .choose_multiple(&mut rng, 5)
                    .map(|city| get_weather(city, api_key).ok())
                    .collect();
                
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let header_image = Image::new("assets/images/openwth.png");

        let city_input = TextInput::new(
            &mut self.city_input_state,
            "Enter city name",
            &self.city_field,
            Message::CityChanged,
        )
        .padding(10)
        .size(20)
        .width(iced::Length::Units(400));

        let get_weather_button = Button::new(
            &mut self.get_weather_button_state,
            Text::new("Get Weather")
        )
        .on_press(Message::GetWeather)
        .padding(10)
        .style(OrangeButton);

        let fetch_random_weather_button = Button::new(
            &mut self.fetch_random_weather_button_state,
            Text::new("Refresh")
        )
        .on_press(Message::FetchRandomCitiesWeather)
        .padding(10)
        .style(OrangeButton);

        let weather_display = match &self.weather_data {
            Some(weather) => {
                let icon_path = format!("assets/images/{}_t.png", weather.weather[0].icon);
                Column::new()
                    .align_items(Alignment::Center)
                    .push(Text::new(format!("City: {}", weather.name)).color(iced::Color::WHITE))
                    .push(Text::new(format!("Description: {}", weather.weather[0].description)).color(iced::Color::WHITE))
                    .push(Image::new(icon_path)) 
                    .push(Text::new(format!("Temperature: {:.2} °C", weather.main.temp)).color(iced::Color::WHITE))
                    .push(Text::new(format!("Humidity: {:.2} %", weather.main.humidity)).color(iced::Color::WHITE))
                    .push(Text::new(format!("Pressure: {:.2} hPa", weather.main.pressure)).color(iced::Color::WHITE))
                    .push(Text::new(format!("Wind Speed: {:.2} m/s", weather.wind.speed)).color(iced::Color::WHITE))
            }
            None => Column::new().push(Text::new("Enter a city and press 'Get Weather'").color(iced::Color::WHITE)),
        };

        let mut random_cities_row = Row::new().spacing(15).align_items(Alignment::Center);
        for weather_opt in &self.random_cities_weather {
            if let Some(weather) = weather_opt {
                let icon_path = format!("assets/images/{}_t.png", weather.weather[0].icon);
                random_cities_row = random_cities_row.push(
                    Column::new()
                        .align_items(Alignment::Center)
                        .push(Text::new(format!("{}", weather.name)).color(iced::Color::WHITE))
                        .push(Text::new(format!("{:.1}°C", weather.main.temp)).color(iced::Color::WHITE))
                        .push(Image::new(icon_path).width(iced::Length::Units(50))),
                );
            }
        }

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(header_image)
            .push(city_input)
            .push(get_weather_button)
            .push(weather_display)
            .push(random_cities_row)
            .push(fetch_random_weather_button);

        container::Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .style(DarkBg)
            .into()
    }
}

fn main() -> iced::Result {
    WeatherApp::run(Settings {
        window: iced::window::Settings {
            size: (1080, 800), 
            ..Default::default()
        },
        ..Default::default()
    })
}