use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use std::sync::Arc;
use serenity::{
    model::id::{
        UserId,
        GuildId
    },
    prelude::TypeMapKey
};
use tiberius::{AuthMethod, Config};
use rust_decimal::{
    Decimal,
    prelude::FromPrimitive
};

pub struct Database {
    pool: Pool<ConnectionManager>
}

impl TypeMapKey for Database {
    type Value = Arc<Database>;
}

impl Database {
    pub async fn new(ip: &str, port: u16, usr: &str, pwd: &str) -> Result<Self, bb8_tiberius::Error> {
        // The password is stored in a file; using secure strings is probably not going to make much of a difference.
        let mut config = Config::new();

        config.host(ip);
        config.port(port);
        config.authentication(AuthMethod::sql_server(usr, pwd));
        config.trust_cert();

        let manager = ConnectionManager::build(config)?;
        let pool = Pool::builder().max_size(8).build(manager).await?;

        Ok(Database { pool })
    }

    pub async fn get_db_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let res = conn.simple_query("SELECT @@version")
            .await?
            .into_first_result()
            .await?
            .into_iter()
            .map(|row| {
                let val: &str = row.get(0).unwrap();
                String::from(val)
            })
            .reduce(|a, b| {format!("{}\n{}", a, b)})
            .unwrap();

        Ok(res)
    }

    pub async fn provide_exp(&self, server_id: GuildId, user_id: UserId) -> Result<tiberius::ExecuteResult, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(*server_id.as_u64()).unwrap();
        let user = Decimal::from_u64(*user_id.as_u64()).unwrap();
        let res = conn.execute(
            "EXEC Ranking.ProvideExp @serverid = @P1, @userid = @P2",
            &[&server, &user])
            .await?;

        Ok(res)
    }

    pub async fn get_exp(&self, server_id: GuildId, user_id: UserId) -> Result<(i32, i32), Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(*server_id.as_u64()).unwrap();
        let user = Decimal::from_u64(*user_id.as_u64()).unwrap();
        let ret = conn.query(
            "SELECT xp, level FROM [Ranking].[Level] WHERE server_id = @P1 AND [user_id] = @P2",
            &[&server, &user])
            .await?
            .into_row()
            .await?;

        let mut out: (i32, i32) = (0, 0);

        if let Some(item) = ret {
            out = (item.get(0).unwrap(), item.get(1).unwrap());
        }

        Ok(out)
    }
}