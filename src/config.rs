use chrono_tz::Tz;

pub const SERVER_PORT: u16 = 3000;

pub const DATABASE_URL: &str = "sqlite:data.db";

pub const LOCAL_CORS_DOMAIN: &str = "http://localhost:3000";

pub const PUBLIC_CORS_DOMAIN: &str = "http://localhost:3000";

pub const TIME_ZONE: Tz = chrono_tz::Asia::Ho_Chi_Minh;

pub const ADMIN_USERNAME: &str = "admin";

pub const ADMIN_PASSWORD: &str = "password";

pub const JWT_SECRET: &str = "secret";
