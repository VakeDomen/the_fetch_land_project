pub mod sale_operations {
    use diesel::result::Error;
    use diesel::{prelude::*, insert_into};

    use crate::database::models::SqliteSale;
    use crate::database::schema::sales::dsl::*;

    use super::sqlite_operations::establish_connection;

    pub fn insert_sale(sqlite_sale: SqliteSale) ->  Result<SqliteSale, Error> {
        let conn = establish_connection();
        let _ = insert_into(sales)
            .values(&sqlite_sale)
            .execute(&conn)?;
        Ok(sqlite_sale)
    }

    pub fn get_latest_sales(num: i64) -> Result<Vec<SqliteSale>, Error> {
        let conn = establish_connection();
        let sqlite_sales = sales
            .order(created.desc())
            .limit(num)
            .load::<SqliteSale>(&conn)?;
        Ok(sqlite_sales)
    }

    pub fn get_sales_by_user(uid: String) -> Result<Vec<SqliteSale>, Error> {
        let conn = establish_connection();
        let resp = sales
            .filter(user_id.eq(uid))
            .load::<SqliteSale>(&conn)?;
        Ok(resp)
    }

    pub fn get_sales_by_card(cid: String) -> Result<Vec<SqliteSale>, Error> {
        let conn = establish_connection();
        let resp = sales
            .filter(sale_object_id.eq(cid))
            .filter(sale_type.eq("CARD"))
            .load::<SqliteSale>(&conn)?;
        Ok(resp)
    }

    pub fn delete_sale(uid: String, sid: String) -> Result<(), Error> {
        let conn = establish_connection();
        diesel::delete(sales
                .filter(id.eq(sid))
                .filter(user_id.eq(uid))
            )
            .execute(&conn)?;
        Ok(())
    }

    pub fn delete_all_user_sales(uid: String) -> Result<(), Error> {
        let conn = establish_connection();
        diesel::delete(sales.filter(user_id.eq(uid)))
            .execute(&conn)?;
        Ok(())
    }
}

pub mod user_operations {
    use diesel::{prelude::*, insert_into};
    use diesel::result::Error;
    use crate::api::user_update::UserPatchData;
    use crate::database::models::SqliteUser;
    use crate::database::schema::users::dsl::*;
    use crate::models::auth::AuthUserData;

    use super::sqlite_operations::establish_connection;

    pub fn get_user_by_google_id(gid: String) ->  Result<Option<SqliteUser>, Error> {
        let conn = establish_connection();
        let mut resp = users
            .filter(google_id.eq(gid))
            .load::<SqliteUser>(&conn)?;
        Ok(resp.pop())
    }


    pub fn get_user_by_id(user_id: String) -> Result<Option<SqliteUser>, Error> {
        let conn = establish_connection();
        let mut resp = users
            .filter(id.eq(user_id))
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

    pub fn update_user_info(uid: String, data: UserPatchData) -> Result<(), Error> {
        let conn = establish_connection();
        diesel::update(users.filter(id.eq(uid)))
            .set((name.eq(data.name), phone.eq(data.phone)))
            .execute(&conn)?;
        Ok(())
    }

    pub fn delete_user(uid: String) -> Result<(), Error> {
        let conn = establish_connection();
        diesel::delete(users.filter(id.eq(uid)))
            .execute(&conn)?;
        Ok(())
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