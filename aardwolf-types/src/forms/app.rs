use crate::{apps::App, error::AardwolfFail, forms::traits::Validate, scope::Scope};

#[derive(Clone, Debug, Fail, Serialize)]
pub enum CreateAppError {
    #[fail(display = "validation error when creating app")]
    ValidationError,
}

impl AardwolfFail for CreateAppError {}

/// Represents the form that is POSTed to /api/v1/apps to create an application
#[derive(Deserialize)]
pub struct CreateApp {
    client_name: String,
    redirect_uris: String,
    scopes: Scope,
    website: Option<String>,
}

impl Validate<App, CreateAppError> for CreateApp {
    fn validate(self) -> Result<App, CreateAppError> {
        if self.client_name.is_empty() || self.client_name.len() > 256 {
            return Err(CreateAppError::ValidationError);
        }

        if self.redirect_uris.is_empty() {
            return Err(CreateAppError::ValidationError);
        }

        let CreateApp {
            client_name,
            redirect_uris,
            scopes,
            website,
        } = self;

        Ok(App {
            client_name,
            redirect_uris,
            scopes,
            website,
        })
    }
}
