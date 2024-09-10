use rust_utils::data::IData;
use crate::models::Post;

pub struct PostAPI;

#[cfg(debug_assertions)]
const HOST: &'static str = "http://127.0.0.1:9000";
#[cfg(not(debug_assertions))]
const HOST: &'static str = "https://zhouzhipeng.com";


impl IData for PostAPI {
    type Model = Post;

    // #[cfg(not(test))]
    fn get_host() -> &'static str {
        HOST
    }
    // #[cfg(test)]
    // fn get_host() -> &'static str {
    //     "https://zhouzhipeng.com"
    // }

    fn get_category() -> &'static str {
        "crab-post"
    }

    #[cfg(test)]
    fn get_auth_key() -> &'static str {
        env!("API_AUTH_KEY")
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_get_posts() ->anyhow::Result<()> {
        PostAPI::insert(&Post{
            title: "zfd".to_string(),
            content: "45sfsdf6".to_string(),
            author_id: "".to_string(),
            published: false,
        }).await?;

        let posts  = PostAPI::list_raw(10).await;
        println!("{:#?}", posts);

        Ok(())
    }
}