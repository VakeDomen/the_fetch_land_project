pub mod sale_operations {
    use diesel::dsl::count;
    use diesel::result::Error;
    use diesel::{prelude::*, insert_into};

    use crate::api::user_sale_edit::SaleEditPatchData;
    use crate::database::models::SqliteSale;
    use crate::database::schema::sales::dsl::*;

    use super::sqlite_operations::establish_connection;

    pub fn get_num_of_sales() -> Result<i64, Error> {
        let conn = establish_connection();
        sales
            .select(count(id))
            .first(&conn)
    }

    pub fn get_sales_paged_by_price(page_size: i64, page_offset: i64) -> Result<Vec<SqliteSale>, Error> {
        let conn = establish_connection();
        let offset = (page_offset) * page_size;
        sales
            .order(price)
            .limit(page_size)
            .offset(offset)
            .load::<SqliteSale>(&conn)
    }

    pub fn get_sales_paged_by_created(page_size: i64, page_offset: i64) -> Result<Vec<SqliteSale>, Error> {
        let conn = establish_connection();
        let offset = (page_offset) * page_size;
        sales
            .order(created.desc())
            .limit(page_size)
            .offset(offset)
            .load::<SqliteSale>(&conn)
    }

    pub fn insert_sale(sqlite_sale: SqliteSale) ->  Result<SqliteSale, Error> {
        let conn = establish_connection();
        let _ = insert_into(sales)
            .values(&sqlite_sale)
            .execute(&conn)?;
        Ok(sqlite_sale)
    }

    pub fn get_latest_sales(num: i64) -> Result<Vec<SqliteSale>, Error> {
        let conn = establish_connection();
        sales
            .order(created.desc())
            .limit(num)
            .load::<SqliteSale>(&conn)
    }

    pub fn get_sales_by_user(uid: String) -> Result<Vec<SqliteSale>, Error> {
        let conn = establish_connection();
        sales
            .filter(user_id.eq(uid))
            .load::<SqliteSale>(&conn)
    }

    pub fn get_sales_by_card(cid: String) -> Result<Vec<SqliteSale>, Error> {
        let conn = establish_connection();
        sales
            .filter(sale_object_id.eq(cid))
            .filter(sale_type.eq("CARD"))
            .load::<SqliteSale>(&conn)
    }

    pub fn update_sale(sid: String, owner_id: String, data: SaleEditPatchData) -> Result<(), Error> {
        let conn = establish_connection();
        diesel::update(
                sales
                    .filter(id.eq(sid))
                    .filter(user_id.eq(owner_id)
                )
            )
            .set((
                description.eq(data.description),
                price.eq(data.price),
                amount.eq(data.amount),
                contact_type.eq(data.contact_type),
                location.eq(data.location),
                web_address.eq(data.web_address)))
            .execute(&conn)?;
        Ok(())
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

pub mod subscription_operations {
    use diesel::result::Error;
    use diesel::{prelude::*, insert_into};

    use crate::database::models::SqliteSubscription;
    use crate::database::schema::subscriptions::dsl::*;

    use super::sqlite_operations::establish_connection;

    pub fn insert_subscription(sqlite_sub: SqliteSubscription) ->  Result<SqliteSubscription, Error> {
        let conn = establish_connection();
        let _ = insert_into(subscriptions)
            .values(&sqlite_sub)
            .execute(&conn)?;
        Ok(sqlite_sub)
    }

    pub fn get_subscriptions_by_user(uid: String) -> Result<Vec<SqliteSubscription>, Error> {
        let conn = establish_connection();
        subscriptions
            .filter(user_id.eq(uid))
            .load::<SqliteSubscription>(&conn)
    }

    pub fn get_subscribtions_by_card(cid: String) -> Result<Vec<SqliteSubscription>, Error> {
        let conn = establish_connection();
        subscriptions
            .filter(sale_object_id.eq(cid))
            .filter(sale_type.eq("CARD"))
            .load::<SqliteSubscription>(&conn)
    }

    pub fn delete_subscription(uid: String, sid: String) -> Result<(), Error> {
        let conn = establish_connection();
        diesel::delete(subscriptions
                .filter(id.eq(sid))
                .filter(user_id.eq(uid))
            )
            .execute(&conn)?;
        Ok(())
    }

    pub fn delete_all_user_subscriptions(uid: String) -> Result<(), Error> {
        let conn = establish_connection();
        diesel::delete(subscriptions.filter(user_id.eq(uid)))
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