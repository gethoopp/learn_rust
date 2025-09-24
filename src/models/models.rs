pub struct client_websockets  {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message,warp::Error>>>

}



#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterRequest {
    user_id: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]

pub struct RegisterResponse {
    url: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Event{
    topic: String,
    user_id: Option<usize>
    message: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TopicResponse{
    topic: Vec<String>
}


