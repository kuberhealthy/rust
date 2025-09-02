use std::{env, error::Error};

use kuberhealthy_client::KuberhealthyClient;

fn main() -> Result<(), Box<dyn Error>> {
    let client = KuberhealthyClient::from_env()?;

    // Placeholder for your own check logic.
    // Pass --fail to simulate a failure.
    let ok = env::args().find(|a| a == "--fail").is_none();

    if ok {
        client.report_success()?;
    } else {
        client.report_failure(vec!["example failure"])?;
    }

    Ok(())
}
