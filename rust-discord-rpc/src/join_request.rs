#[derive(Default, Clone, Hash, PartialEq, Debug)]
pub struct JoinRequest {
    pub user_id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
}

#[derive(Clone, Hash, PartialEq, Debug)]
pub enum JoinRequestReply {
    No,
    Yes,
    Ignore,
}
