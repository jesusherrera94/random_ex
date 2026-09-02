pub trait ConfigDefault {
    fn get_default() -> Self;
}

// These types need default values
pub struct ConnectionTimeout(pub u64);
pub struct MaxConnections(pub u32);
pub struct RetryAttempts(pub u8);
pub struct PostgresPort(pub u16);
pub struct MySQLPort(pub u16);
pub struct MongoPort(pub u16);
pub struct RedisPort(pub u16);

#[macro_export]
macro_rules! config_default_impl {
    ($struct:ident, $default:expr) => {
        impl ConfigDefault for $struct {
            fn get_default() -> Self {
                $struct($default)
            }
        }
    };
}

impl ConfigDefault for ConnectionTimeout {
    fn get_default() -> Self {
        ConnectionTimeout(10)
    }
}
impl ConfigDefault for MaxConnections {
    fn get_default() -> Self {
        MaxConnections(100)
    }
}
impl ConfigDefault for RetryAttempts {
    fn get_default() -> Self {
        RetryAttempts(3)
    }
}
impl ConfigDefault for PostgresPort {
    fn get_default() -> Self {
        PostgresPort(5432)
    }
}
impl ConfigDefault for MySQLPort {
    fn get_default() -> Self {
        MySQLPort(3306)
    }
}
impl ConfigDefault for MongoPort {
    fn get_default() -> Self {
        MongoPort(27017)
    }
}
impl ConfigDefault for RedisPort {
    fn get_default() -> Self {
        RedisPort(6379)
    }
}
// Example usage
pub fn main() {
    // let's say we have a new struct
    struct CustomPort(pub u16);

    // we implement the ConfigDefault trait for CustomPort
    config_default_impl!(CustomPort, 8080);

    // when running the `get_default` method, it should return the default value
    assert_eq!(<CustomPort as ConfigDefault>::get_default().0, 8080);
}
