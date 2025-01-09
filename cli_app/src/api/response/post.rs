#[derive(Serialize, ToSchema)]
pub struct SinglePostResponse {
    pub data: Post,
}

#[derive(Serialize, ToSchema)]
pub struct ListPostResponse {
    pub data: Vec<Post>,
}
