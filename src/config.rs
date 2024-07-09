use chrono_tz::Tz;

pub const SERVER_PORT: u16 = 3000;

pub const LOCAL_CORS_DOMAIN: &str = "http://localhost:3000";

pub const PUBLIC_CORS_DOMAIN: &str = "http://127.0.0.1:3001";

pub const TIME_ZONE: Tz = chrono_tz::Asia::Ho_Chi_Minh;

pub const ADMIN_USERNAME: &str = "admin";

pub const ADMIN_PASSWORD: &str = "password";

pub const ADMIN_PRIVATE_KEY: &str = "5NPUotvpDFnhti1v8sS7izg5LUHVVf2zVdDro8uvzntYYq2X4CRdMGhmqxxsrpbcGFbe4N84cDHn2hSnhWun3UV3";

pub const SOLANA_ENDPOINT: &str = "https://api.devnet.solana.com";

pub const JWT_SECRET: &str = "secret";
