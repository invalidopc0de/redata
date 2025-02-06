use prusto::auth::Auth;
use rocket::figment::Figment;
use rocket_db_pools::{Pool, Config, Error};
use prusto::ClientBuilder;

pub struct TrinoPool {
    host: String,
    port: u16,
    user: String,
    auth: Auth,
}

#[rocket::async_trait]
impl Pool for TrinoPool {
    type Connection = prusto::Client;
    type Error = Error<prusto::Error>;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        println!("Initializing TrinoPool");
        //let config: Config = figment.extract()?;

        let auth = Auth::new_basic(
            "user".to_string(),
            Some("password".to_string()),
        );

        Ok(TrinoPool {
            host: "trino".to_string(),
            port: 8080,
            user: "user".to_string(),
            auth,
        })
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        // Not sure if this is how connections are supposed to work, but
        // going to try it for now
        let client = ClientBuilder::new(
                self.user.to_string(),
                self.host.to_string()
            )
            .port(self.port)
            .catalog("metadata-db")
            .auth(self.auth.clone())
            .build()
            .unwrap();
        Ok(client)
    }

    async fn close(&self) {
        // Do nothing
    }
}
