use redis;


struct GrantToken {
    expiring_epoch: u64,
    state_key: String,

    grant_token: String
}

struct Token {
    renewal_epoch: u64,

    token: String,
    renewal_token: String,

    is_locked: bool,
    lock_epoch: u64,

    user_id: String
}




impl Token {

}



pub fn getToken(user_id: String) -> Result<Token> {
    todo!();
}

pub fn subscribeForTokenRequest(channel_name: String) -> Result<()> {
    todo!();
}


