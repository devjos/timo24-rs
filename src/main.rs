mod date;
mod timo;

use crate::timo::Project::SprintMeeting;
use crate::timo::Zeitart::{Gehen, Kommen, PauseEnde, PauseStart};
use crate::timo::{TimoClient, TimoUserConfig};
use figment::providers::{Format, Json};
use figment::Figment;
use std::thread;
use std::time::Duration;

const ONE_SECOND: Duration = Duration::from_secs(1);

fn main() {
    let config = read_timo_user_config();

    let timo_client = TimoClient::new(config);
    timo_client.login();
    thread::sleep(ONE_SECOND);

    let date = date::today();

    timo_client.book_attendance(&Kommen, &date, "08:00");
    thread::sleep(ONE_SECOND);
    timo_client.book_attendance(&PauseStart, &date, "12:00");
    thread::sleep(ONE_SECOND);
    timo_client.book_attendance(&PauseEnde, &date, "12:30");
    thread::sleep(ONE_SECOND);
    timo_client.book_attendance(&Gehen, &date, "16:30");

    timo_client.book_project(&SprintMeeting, &date, "0:30");

    timo_client.print_login_url();
}

fn read_timo_user_config() -> TimoUserConfig {
    let config: TimoUserConfig = Figment::new()
        .merge(Json::file("config.json"))
        .extract()
        .unwrap();
    config
}
