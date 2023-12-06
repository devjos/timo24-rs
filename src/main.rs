mod date;
mod timo;

use crate::timo::Project::SprintMeeting;
use crate::timo::Zeitart::{Gehen, Kommen, PauseEnde, PauseStart};
use crate::timo::{TimoClient, TimoUserConfig};
use figment::providers::{Format, Json};
use figment::Figment;
use serde::Deserialize;
use std::thread;
use std::time::Duration;

const ONE_SECOND: Duration = Duration::from_secs(1);

#[derive(Deserialize)]
pub struct TimeConfig {
    kommen_time: String,
    pause_start_time: String,
    pause_ende_time: String,
    gehen_time: String,

    sprint_meeting_time: String,
}

fn main() {
    let user_config = read_timo_user_config();
    let time_config = read_time_config();

    let timo_client = TimoClient::new(user_config);
    timo_client.login();
    thread::sleep(ONE_SECOND);

    let date = date::today();

    timo_client.book_attendance(&Kommen, &date, &time_config.kommen_time);
    thread::sleep(ONE_SECOND);
    timo_client.book_attendance(&PauseStart, &date, &time_config.pause_start_time);
    thread::sleep(ONE_SECOND);
    timo_client.book_attendance(&PauseEnde, &date, &time_config.pause_ende_time);
    thread::sleep(ONE_SECOND);
    timo_client.book_attendance(&Gehen, &date, &time_config.gehen_time);

    timo_client.book_project(&SprintMeeting, &date, &time_config.sprint_meeting_time);

    timo_client.print_login_url();
}

fn read_timo_user_config() -> TimoUserConfig {
    let config: TimoUserConfig = Figment::new()
        .merge(Json::file("user-config.json"))
        .extract()
        .unwrap();
    config
}

fn read_time_config() -> TimeConfig {
    let config: TimeConfig = Figment::new()
        .merge(Json::file("time-config.json"))
        .extract()
        .unwrap();
    config
}
