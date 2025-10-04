use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::{UserRepository, UserRepositoryError};
use crate::infrastructure::database::schema::users;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

// Modelo Diesel para inserção
#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    id: Uuid,
    email: String,
    password_hash: String,
    name: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Modelo Diesel para leitura
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct UserModel {
    id: Uuid,
    email: String,
    password_hash: String,
    name: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        User {
            id: model.id,
            email: model.email,
            password_hash: model.password_hash,
            name: model.name,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<User> for NewUser {
    fn from(user: User) -> Self {
        NewUser {
            id: user.id,
            email: user.email,
            password_hash: user.password_hash,
            name: user.name,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

pub struct DieselUserRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselUserRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for DieselUserRepository {
    async fn create(&self, user: User) -> Result<User, UserRepositoryError> {
        use crate::infrastructure::database::schema::users::dsl::*;
        
        let mut conn = self.pool.get()
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        let new_user = NewUser::from(user);

        let result = tokio::task::spawn_blocking(move || {
            diesel::insert_into(users)
                .values(&new_user)
                .returning(UserModel::as_returning())
                .get_result::<UserModel>(&mut conn)
        })
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?
        .map_err(|e: diesel::result::Error| match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => UserRepositoryError::EmailAlreadyExists,
            _ => UserRepositoryError::DatabaseError(e.to_string()),
        })?;

        Ok(result.into())
    }

    async fn find_by_email(&self, user_email: &str) -> Result<Option<User>, UserRepositoryError> {
        use crate::infrastructure::database::schema::users::dsl::*;
        
        let mut conn = self.pool.get()
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        let user_email = user_email.to_string();

        let result = tokio::task::spawn_blocking(move || {
            users
                .filter(email.eq(user_email))
                .select(UserModel::as_select())
                .first::<UserModel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.map(|model| model.into()))
    }

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, UserRepositoryError> {
        use crate::infrastructure::database::schema::users::dsl::*;
        
        let mut conn = self.pool.get()
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        let result = tokio::task::spawn_blocking(move || {
            users
                .filter(id.eq(user_id))
                .select(UserModel::as_select())
                .first::<UserModel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.map(|model| model.into()))
    }

    async fn update(&self, user: User) -> Result<User, UserRepositoryError> {
        use crate::infrastructure::database::schema::users::dsl::*;
        
        let mut conn = self.pool.get()
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        let user_id = user.id;
        let user_email = user.email.clone();
        let user_password_hash = user.password_hash.clone();
        let user_name = user.name.clone();
        let user_is_active = user.is_active;
        let now = Utc::now();

        let result = tokio::task::spawn_blocking(move || {
            diesel::update(users.find(user_id))
                .set((
                    email.eq(user_email),
                    password_hash.eq(user_password_hash),
                    name.eq(user_name),
                    is_active.eq(user_is_active),
                    updated_at.eq(now),
                ))
                .returning(UserModel::as_returning())
                .get_result::<UserModel>(&mut conn)
        })
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?
        .map_err(|e: diesel::result::Error| match e {
            diesel::result::Error::NotFound => UserRepositoryError::NotFound,
            _ => UserRepositoryError::DatabaseError(e.to_string()),
        })?;

        Ok(result.into())
    }

    async fn delete(&self, user_id: Uuid) -> Result<(), UserRepositoryError> {
        use crate::infrastructure::database::schema::users::dsl::*;
        
        let mut conn = self.pool.get()
            .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        let deleted_count = tokio::task::spawn_blocking(move || {
            diesel::delete(users.find(user_id))
                .execute(&mut conn)
        })
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        if deleted_count == 0 {
            Err(UserRepositoryError::NotFound)
        } else {
            Ok(())
        }
    }
}
