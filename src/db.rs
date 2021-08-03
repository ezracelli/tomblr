use std::ops::{Deref, DerefMut};

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub struct Pool(r2d2::Pool<ConnectionManager<PgConnection>>);

impl Pool {
    pub fn new<S: Into<String>>(database_url: S) -> Result<Self, r2d2::PoolError> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::new(manager)?;
        Ok(Self(pool))
    }
}

impl Deref for Pool {
    type Target = r2d2::Pool<ConnectionManager<PgConnection>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
