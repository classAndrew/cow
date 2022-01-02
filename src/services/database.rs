use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use std::sync::Arc;
use serenity::{
    model::id::{
        UserId,
        GuildId,
        ChannelId, RoleId
    },
    prelude::TypeMapKey
};
use tiberius::{AuthMethod, Config};
use rust_decimal::{
    Decimal,
    prelude::FromPrimitive
};
use rust_decimal::prelude::ToPrimitive;

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
        // Default schema needs to be Cow
        config.database("Cow");
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

    pub async fn provide_exp(&self, server_id: GuildId, user_id: UserId) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(*server_id.as_u64()).unwrap();
        let user = Decimal::from_u64(*user_id.as_u64()).unwrap();
        let res = conn.query(
            "EXEC Ranking.ProvideExp @serverid = @P1, @userid = @P2",
            &[&server, &user])
            .await?
            .into_row()
            .await?;

        let mut out: i32 = -1;
        // Returns -1 (or less than 0): didn't level up
        // If positive, that's the new level they reached

        if let Some(item) = res {
            out = item.get(0).unwrap();
        }

        Ok(out)
    }

    pub async fn get_xp(&self, server_id: GuildId, user_id: UserId) -> Result<(i32, i32), Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(*server_id.as_u64()).unwrap();
        let user = Decimal::from_u64(*user_id.as_u64()).unwrap();
        let res = conn.query(
            "SELECT xp, level FROM [Ranking].[Level] WHERE server_id = @P1 AND [user_id] = @P2",
            &[&server, &user])
            .await?
            .into_row()
            .await?;

        let mut out: (i32, i32) = (0, 0);

        if let Some(item) = res {
            out = (item.get(0).unwrap(), item.get(1).unwrap());
        }

        Ok(out)
    }

    // True: disabled False: enabled
    // Because by default a channel should be enabled, right?
    pub async fn toggle_channel_xp(&self, server_id: GuildId, channel_id: ChannelId) -> Result<bool, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(*server_id.as_u64()).unwrap();
        let channel = Decimal::from_u64(*channel_id.as_u64()).unwrap();
        let res = conn.query(
            "EXEC [Ranking].[ToggleChannel] @serverid = @P1, @channelid = @P2",
            &[&server, &channel])
            .await?
            .into_row()
            .await?;

        let mut out: bool = false;

        if let Some(item) = res {
            out = item.get(0).unwrap();
        }

        Ok(out)
    }

    pub async fn channel_disabled(&self, server_id: GuildId, channel_id: ChannelId) -> Result<bool, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(*server_id.as_u64()).unwrap();
        let channel = Decimal::from_u64(*channel_id.as_u64()).unwrap();
        let res = conn.query(
            "SELECT CAST(1 AS BIT) FROM [Ranking].[DisabledChannel] WHERE server_id = @P1 AND channel_id = @P2",
            &[&server, &channel])
            .await?
            .into_row()
            .await?;

        let mut out: bool = false;

        if let Some(item) = res {
            out = item.get(0).unwrap();
        }

        Ok(out)
    }

    pub async fn top_members(&self, server_id: GuildId) -> Result<Vec<(u64, i32, i32)>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(*server_id.as_u64()).unwrap();
        let res = conn.query(
            "SELECT TOP 10 user_id, level, xp FROM [Ranking].[Level] WHERE server_id = @P1 ORDER BY level DESC, xp DESC",
            &[&server])
            .await?
            .into_first_result()
            .await?
            .into_iter()
            .map(|row| {
                let id: rust_decimal::Decimal = row.get(0).unwrap();
                let value: (u64, i32, i32) = (id.to_u64().unwrap(), row.get(1).unwrap(), row.get(2).unwrap());
                value
            })
            .collect::<Vec<_>>();

        Ok(res)
    }

    pub async fn get_roles(&self, server_id: GuildId) -> Result<Vec<(String, Option<u64>, i32)>, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(*server_id.as_u64()).unwrap();
        let res = conn.query(
            "SELECT role_name, role_id, min_level FROM [Ranking].[Role] WHERE server_id = @P1 ORDER BY min_level ASC",
            &[&server])
            .await?
            .into_first_result()
            .await?
            .into_iter()
            .map(|row| {
                let name: &str = row.get(0).unwrap();
                let mut id: Option<u64> = None;
                if let Some(row) = row.get(1) {
                    let id_dec: rust_decimal::Decimal = row;
                    id = id_dec.to_u64();
                }
                let value: (String, Option<u64>, i32) = (String::from(name), id, row.get(2).unwrap());
                value
            })
            .collect::<Vec<_>>();

        Ok(res)
    }

    // will also set role 
    pub async fn add_role(&self, server_id: GuildId, role_name: &String, role_id: RoleId, min_level: i32) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let server = Decimal::from_u64(*server_id.as_u64()).unwrap();
        let role = Decimal::from_u64(*role_id.as_u64()).unwrap();
        let res = conn.query(
            "EXEC [Ranking].[AddRole] @server_id = @P1, @role_name = @P2, @role_id = @P3, @min_level = @P4",
            &[&server, role_name, &role, &Decimal::from_i32(min_level).unwrap()])
            .await?
            .into_row()
            .await?;

        let mut out: bool = false;

        if let Some(item) = res {
            out = item.get(0).unwrap();
        }

        Ok(out)
    }
}