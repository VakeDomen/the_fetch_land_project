pub mod user_operations {
    use diesel::{prelude::*, insert_into};
    use diesel::result::Error;
    use crate::database::models::SqliteUser;
    use crate::database::schema::users::dsl::*;
    use crate::models::auth::AuthUserData;

    use super::sqlite_operations::establish_connection;

    pub fn get_user_by_google_id(gid: String) ->  Result<Option<SqliteUser>, Error> {
        let conn = establish_connection();
        let mut resp = users
            .filter(google_id.eq(gid.to_string()))
            .load::<SqliteUser>(&conn)?;
        Ok(resp.pop())
    }

    pub fn insert_user(google_user: AuthUserData) ->  Result<SqliteUser, Error> {
        let sqlite_user = SqliteUser::from(google_user);
        let conn = establish_connection();
        let _ = insert_into(users)
            .values(&sqlite_user)
            .execute(&conn)?;
        Ok(sqlite_user)
    }
}

pub mod sqlite_operations {
    use diesel::{SqliteConnection, Connection};
    use std::{env};
    pub(crate) fn establish_connection() -> SqliteConnection {
        SqliteConnection::establish(
            &env::var("DATABASE_URL").expect("No DATABASE_URL in .env")
        ).expect("Error connecting to database!")
    }
}