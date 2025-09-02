use reqwest::blocking::Client;
use serde::Serialize;
use std::{env, error::Error};

/// Client for reporting check results back to Kuberhealthy.
pub struct KuberhealthyClient {
    reporting_url: String,
    run_uuid: String,
    http: Client,
}

#[derive(Serialize)]
struct Report {
    #[serde(rename = "Errors")]
    errors: Vec<String>,
    #[serde(rename = "OK")]
    ok: bool,
}

impl KuberhealthyClient {
    /// Create a client from the `KH_REPORTING_URL` and `KH_RUN_UUID` environment variables.
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(KuberhealthyClient {
            reporting_url: env::var("KH_REPORTING_URL")?,
            run_uuid: env::var("KH_RUN_UUID")?,
            http: Client::new(),
        })
    }

    /// Report a successful check to Kuberhealthy.
    pub fn report_success(&self) -> Result<(), reqwest::Error> {
        self.send(Report {
            errors: vec![],
            ok: true,
        })
    }

    /// Report a failed check to Kuberhealthy with the given error messages.
    pub fn report_failure<S: Into<String>>(&self, errors: Vec<S>) -> Result<(), reqwest::Error> {
        self.send(Report {
            errors: errors.into_iter().map(Into::into).collect(),
            ok: false,
        })
    }

    fn send(&self, report: Report) -> Result<(), reqwest::Error> {
        self.http
            .post(&self.reporting_url)
            .header("kh-run-uuid", &self.run_uuid)
            .json(&report)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}
