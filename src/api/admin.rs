//! Koleksi query yang digunakan untuk operasi pada rest API Admin
#![allow(missing_docs)]

use diesel::prelude::*;
use validator::Validate;

use crate::{
    api,
    api::types::*,
    api::{
        error::{param_error, unauthorized},
        ApiResult,
    },
    dao::AdminDao,
    error::Error,
    models,
    prelude::*,
    ID,
};

/// New Admin query
#[derive(Serialize, Deserialize, Validate)]
pub struct NewAdmin {
    #[validate(length(
        min = 3,
        max = 500,
        message = "Invalid name length, min 3 and max 500 characters"
    ))]
    pub name: String,
    #[validate(email(message = "Email not valid, please enter valid email address"))]
    pub email: String,
    #[validate(phone(message = "Invalid phone number: {}"))]
    pub phone_num: String,
    #[validate(length(
        min = 6,
        max = 500,
        message = "Invalid password length, min 6 and max 500 characters"
    ))]
    pub password: String,
    #[validate(length(
        min = 6,
        max = 500,
        message = "Invalid password length, min 6 and max 500 characters"
    ))]
    pub confirm_password: String,

    pub accesses: Vec<String>,
}

/// Activate Admin query
#[derive(Serialize, Deserialize, Validate)]
pub struct ActivateAdmin {
    #[validate(phone(message = "Invalid phone number: {}"))]
    pub phone_num: String,
    #[validate(length(min = 6, message = "Password min 6 characters"))]
    pub password: String,
    #[validate(length(min = 1, message = "Token can't be empty"))]
    pub token: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdatePassword {
    pub id: ID,
    pub password: String,
    pub password_confm: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateAccesses {
    pub id: ID,
    pub accesses: Vec<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateMeta {
    pub id: ID,
    pub meta: Vec<String>,
}

/// Holder untuk implementasi API endpoint publik untuk admin.
pub struct PublicApi;

#[api_group("Admin", "public", base = "/admin/v1", accessor = "admin")]
impl PublicApi {
    /// Rest API endpoint untuk menambahkan admin baru.
    #[api_endpoint(path = "/add", mutable, auth = "required")]
    pub fn add_admin(query: NewAdmin) -> ApiResult<models::Admin> {
        query.validate()?;

        let conn = state.db();
        let dao = AdminDao::new(&conn);

        if query.password != query.confirm_password {
            return param_error("Confirmation password didn't match");
        }

        if current_admin.id != 1 {
            return unauthorized();
        }

        let mut labels = vec![];

        for access in query.accesses {
            labels.push(format!("access.{}", access));
        }

        labels.sort();
        labels.dedup();

        dao.create(
            &query.name,
            &query.email,
            &query.phone_num,
            &query.password,
            &labels,
        )
        .map_err(From::from)
        .map(ApiResult::success)
    }

    /// Update accesses.
    #[api_endpoint(path = "/update_accesses", auth = "required", mutable, accessor = "admin")]
    pub fn update_accesses(query: UpdateAccesses) -> ApiResult<()> {
        use crate::schema::admins::{self, dsl};
        let conn = state.db();

        if current_admin.id != 1 {
            return unauthorized();
        }

        let admin = AdminDao::new(&conn).get_by_id(query.id)?;

        let mut meta = admin.meta.clone();

        meta = meta.into_iter().filter(|a| !a.starts_with("access.")).collect();

        for acc in query.accesses {
            meta.push(format!("access.{}", acc));
        }

        diesel::update(dsl::admins.filter(dsl::id.eq(query.id)))
            .set(dsl::meta.eq(meta))
            .execute(&conn)
            .map_err(Error::from)?;

        Ok(ApiResult::success(()))
    }

    /// Update meta.
    #[api_endpoint(path = "/update_meta", auth = "required", mutable, accessor = "admin")]
    pub fn update_meta(query: UpdateMeta) -> ApiResult<()> {
        use crate::schema::admins::{self, dsl};
        let conn = state.db();

        if current_admin.id != 1 {
            return unauthorized();
        }

        let admin = AdminDao::new(&conn).get_by_id(query.id)?;

        diesel::update(dsl::admins.filter(dsl::id.eq(query.id)))
            .set(dsl::meta.eq(query.meta))
            .execute(&conn)
            .map_err(Error::from)?;

        Ok(ApiResult::success(()))
    }

    /// Mendapatkan daftar admin
    #[api_endpoint(path = "/list", auth = "required", accessor = "admin")]
    pub fn list_admin(query: QueryEntries) -> ApiResult<EntriesResult<Admin>> {
        query.validate()?;

        let conn = state.db();
        let dao = AdminDao::new(&conn);

        if current_admin.id > 1 {
            return unauthorized();
        }

        let entries = dao.get_admins(query.offset, query.limit)?;

        // filter out admin and system user from listing
        let entries: Vec<models::Admin> = entries.into_iter().filter(|a| a.id > 1).collect();

        let count = dao.count()?;
        Ok(ApiResult::success(EntriesResult {
            count,
            entries: entries.into_iter().map(|a| a.to_api_type(&conn)).collect(),
        }))
    }

    /// Mendapatkan jumlah admin secara keseluruhan.
    #[api_endpoint(path = "/count", auth = "required")]
    pub fn admin_count(state: &AppState, query: ()) -> ApiResult<i64> {
        let conn = state.db();
        let dao = AdminDao::new(&conn);

        dao.count().map(ApiResult::success).map_err(From::from)
    }

    /// Mendapatkan data admin berdasarkan ID.
    #[api_endpoint(path = "/detail", auth = "required", accessor = "admin")]
    pub fn admin_detail(query: IdQuery) -> ApiResult<Admin> {
        let conn = state.db();
        let dao = AdminDao::new(&conn);

        dao.get_by_id(query.id)
            .map(|a| ApiResult::success(a.to_api_type(&conn)))
            .map_err(From::from)
    }

    /// Delete admin.
    #[api_endpoint(path = "/delete", auth = "required", mutable = "true")]
    pub fn delete_admin(query: IdQuery) -> ApiResult<()> {
        let conn = state.db();
        let dao = AdminDao::new(&conn);

        dao.delete_by_id(query.id)?;

        Ok(ApiResult::success(()))
    }

    /// Mendapatkan informasi current admin.
    #[api_endpoint(path = "/me/info", auth = "required", accessor = "admin")]
    pub fn me_info(state: &AppState, query: (), req: &ApiHttpRequest) -> ApiResult<models::Admin> {
        Ok(ApiResult::success(current_admin))
    }

    /// Request code untuk reset password.
    #[api_endpoint(path = "/reset_password/request", auth = "none", mutable)]
    pub fn reset_password(query: ResetPassword) -> ApiResult<()> {
        query.validate()?;

        let conn = state.db();
        let dao = AdminDao::new(&conn);
        let admin = dao.get_by_email(&query.email)?;

        dao.reset_password(admin.id, admin.name, admin.email)?;

        Ok(ApiResult::success(()))
    }

    /// Verifikasi token untuk reset password.
    #[api_endpoint(path = "/reset_password/verify", auth = "none", mutable)]
    pub fn reset_password_verify(query: ResetPassword) -> ApiResult<()> {
        query.validate()?;

        let conn = state.db();
        let dao = AdminDao::new(&conn);
        let admin = dao.get_by_email(&query.email)?;

        if let Some(token) = query.token {
            dao.verify_reset_password(admin.id, &token)?;
        } else {
            param_error("Parameter token can't be empty.")?;
        }

        Ok(ApiResult::success(()))
    }

    /// Mengubah password dengan password yang baru berdasarkan reset password code.
    #[api_endpoint(path = "/reset_password", auth = "none", mutable)]
    pub fn set_new_password(query: ResetPassword) -> ApiResult<()> {
        query.validate()?;

        let conn = state.db();
        let dao = AdminDao::new(&conn);
        let admin = dao.get_by_email(&query.email)?;

        match (query.token, query.password) {
            (Some(token), Some(password)) => {
                dao.verify_reset_password(admin.id, &token)?;
                dao.set_password(admin.id, &password)?;
                dao.remove_reset_password(admin.id)?;
            }
            _ => param_error("Parameter token or password can't be empty.")?,
        }

        Ok(ApiResult::success(()))
    }

    /// Update password.
    #[api_endpoint(path = "/update_password", auth = "required", mutable, accessor = "admin")]
    pub fn update_password(query: UpdatePassword) -> ApiResult<()> {
        let conn = state.db();

        let dao = AdminDao::new(&conn);

        if current_admin.id != 1 {
            return unauthorized();
        }

        dao.set_password(query.id, &query.password)?;

        Ok(ApiResult::success(()))
    }
}

/// Holder untuk implementasi API endpoint privat.
pub struct PrivateApi;

#[api_group("Admin", "private", base = "/admin/v1", accessor = "admin")]
impl PrivateApi {}
