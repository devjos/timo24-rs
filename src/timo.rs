use reqwest::blocking::{Client, ClientBuilder};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct TimoUserConfig {
    firma: String,
    user: String,
    password: String,
    mitarbeiter: String,
}

#[derive(Debug)]
pub enum Zeitart {
    Kommen,
    Gehen,
    PauseStart,
    PauseEnde,
}
pub struct TimoClient {
    config: TimoUserConfig,
    client: Client,
}

impl TimoClient {
    pub fn new(config: TimoUserConfig) -> TimoClient {
        let client_builder = ClientBuilder::new()
            .http1_title_case_headers()
            .cookie_store(true);
        let client = client_builder.build().unwrap();
        TimoClient { config, client }
    }

    pub fn login(&self) {
        // GET also possible
        //    let url = format!(
        //         "https://836.timo24.de/timo/main_login.jsp?user={}&firma={}&password={}&token=",
        //         self.config.user, self.config.firma, self.config.password
        //     );
        //     let res = self.client.get(&url).send().unwrap();
        // println!("{}", res.text().unwrap());

        let params = [
            ("firma", self.config.firma.to_string()),
            ("user", self.config.user.to_string()),
            ("password", self.config.password.to_string()),
            ("t", Uuid::new_v4().to_string()),
            ("back", "https://www.timo24.de/timoadmin/login".to_string()),
        ];
        let res = self
            .client
            .post("https://836.timo24.de/timo/main_login.jsp")
            .form(&params)
            .send()
            .unwrap();

        println!("POST login returned status {}", res.status().as_u16());
        if !res.status().is_success() {
            panic!(
                "Could not login (status {}). Wrong credentials?",
                res.status().as_u16()
            );
        }
    }

    pub fn book_attendance(&self, attendance_type: Zeitart, date: &str, time: &str) {
        let art = match attendance_type {
            Zeitart::Kommen => "1",
            Zeitart::Gehen => "2",
            Zeitart::PauseStart => "3",
            Zeitart::PauseEnde => "4",
        };
        let attendance_params = [
            ("id", "-1"),
            ("mitarbeiter", &self.config.mitarbeiter),
            ("art", art),
            ("datum", date),
            ("zeit", time),
            ("description", ""),
            ("noTimezones", "true"),
        ];

        let res = self
            .client
            .put("https://836.timo24.de/timo/services/rest/wtoverview/saveattendancetime")
            .form(&attendance_params)
            .send()
            .unwrap();

        if res.status().is_success() {
            println!(
                "Booked attendance {:?} {} {} ✅",
                attendance_type, date, time
            );
        } else {
            println!(
                "Could not book attendance {:?} {} {} ❌ Received status {}",
                attendance_type,
                date,
                time,
                res.status().as_u16()
            );
        }
    }

    pub fn print_login_url(&self) {
        let url = format!(
            "https://836.timo24.de/timo/main_login.jsp?user={}&firma={}&password={}&token=",
            self.config.user, self.config.firma, self.config.password
        );
        println!("Link zu Timo24: {}", url);
    }
}
