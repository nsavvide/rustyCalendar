use chrono::{Datelike, Timelike, Utc};
use google_calendar3::{CalendarHub, Error};
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator}; // Add chrono to Cargo.toml for date and time manipulation

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Read the service account key (JSON) you've downloaded from the Google Developers Console.
    let secret = read_service_account_key("secret.json")
        .await
        .expect("secret.json");

    // Create an authenticator.
    let auth = ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .unwrap();

    let hub = CalendarHub::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
        auth,
    );

    let now = Utc::now();
    let today = now.date(); // Get the current date (time is set to 00:00:00)
    let start_of_day = today.and_hms(0, 0, 0); // Start of the day
    let end_of_day = today.and_hms(23, 59, 59); // End of the day, one second before midnight

    // Call the API to list events for today from the primary calendar
    let result = hub
        .events()
        .list("niels.i.savvides@gmail.com") // You can replace 'primary' with any specific calendar id
        .time_min(&start_of_day.to_rfc3339())
        .time_max(&end_of_day.to_rfc3339())
        .single_events(true) // Set to true to expand recurring events into instances
        .order_by("startTime") // Orders events by start time
        .doit()
        .await;

    // Handle the result
    match result {
        Ok((_, event_list)) => {
            for event in event_list.items.unwrap_or_else(|| vec![]) {
                let start_time = event
                    .start
                    .unwrap_or_default()
                    .date_time
                    .unwrap_or_else(|| "All day".to_string());
                let end_time = event
                    .end
                    .unwrap_or_default()
                    .date_time
                    .unwrap_or_else(|| "Unknown end time".to_string());
                println!(
                    "Event: {}, Start: {}, End: {}",
                    event.summary.unwrap_or_default(),
                    start_time,
                    end_time
                );
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    Ok(())
}
