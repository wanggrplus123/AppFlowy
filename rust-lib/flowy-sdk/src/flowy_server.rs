use flowy_infra::uuid;
use flowy_user::{
    entities::{SignInParams, SignUpParams, UserDetail},
    errors::{ErrorBuilder, UserError, UserErrorCode},
    prelude::UserServer,
    sql_tables::User,
};

pub type ArcFlowyServer = std::sync::Arc<dyn FlowyServer>;

pub trait FlowyServer: UserServer {}

pub struct MockFlowyServer {}

impl FlowyServer for MockFlowyServer {}

impl UserServer for MockFlowyServer {
    fn sign_up(&self, params: SignUpParams) -> Result<User, UserError> {
        // let user_id = "9527".to_owned();
        let user_id = uuid();
        Ok(User::new(
            user_id,
            params.name,
            params.email,
            params.password,
        ))
    }

    fn sign_in(&self, params: SignInParams) -> Result<User, UserError> {
        let user_id = uuid();
        Ok(User::new(
            user_id,
            "".to_owned(),
            params.email,
            params.password,
        ))
    }

    fn get_user_info(&self, _user_id: &str) -> Result<UserDetail, UserError> {
        Err(ErrorBuilder::new(UserErrorCode::Unknown).build())
    }

    fn sign_out(&self, _user_id: &str) -> Result<(), UserError> {
        Err(ErrorBuilder::new(UserErrorCode::Unknown).build())
    }
}