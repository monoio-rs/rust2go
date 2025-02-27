pub mod binding {
    #![allow(warnings)]
    monoio_rust2go::r2g_include_binding!();
}

#[derive(monoio_rust2go::R2G, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u8,
}

#[derive(monoio_rust2go::R2G, Clone)]
pub struct LoginRequest {
    pub user: User,
    pub password: String,
}

#[derive(monoio_rust2go::R2G, Clone)]
pub struct LoginResponse {
    pub succ: bool,
    pub message: String,
    pub token: Vec<u8>,
}

#[derive(monoio_rust2go::R2G, Clone)]
pub struct LogoutRequest {
    pub token: Vec<u8>,
    pub user_ids: Vec<u32>,
}

#[derive(monoio_rust2go::R2G, Clone)]
pub struct FriendsListRequest {
    pub token: Vec<u8>,
    pub user_ids: Vec<u32>,
}

#[derive(monoio_rust2go::R2G, Clone)]
pub struct FriendsListResponse {
    pub users: Vec<User>,
}

#[derive(monoio_rust2go::R2G, Clone)]
pub struct PMFriendRequest {
    pub user_id: u32,
    pub token: Vec<u8>,
    pub message: String,
}

#[derive(monoio_rust2go::R2G, Clone)]
pub struct PMFriendResponse {
    pub succ: bool,
    pub message: String,
}

#[monoio_rust2go::r2g]
pub trait TestCall {
    fn ping(n: usize) -> usize;
    fn login(req: &LoginRequest) -> LoginResponse;
    fn logout(req: &User);
    async fn add_friends(req: &FriendsListRequest) -> FriendsListResponse;
    #[drop_safe]
    async fn delete_friends(req: FriendsListRequest) -> FriendsListResponse;
    #[drop_safe_ret]
    async fn pm_friend(req: PMFriendRequest) -> PMFriendResponse;
}
