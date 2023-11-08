use usecase::register_function::register_url_usecase::RegisterSingleUrlUseCase
use anyhow::Error;

pub struct SingleFeedUrl(pub String);
pub struct RegisterUrlController;

impl RegisterSingleUrlUseCase for RegisterUrlController {
    fn register_single_feed_url(&self, url: String) -> Result<String, Error> {
        todo!()
    }
}

fn new_register_url_controller() ->  {
    RegisterUrlController
}



#[cfg(test)]
mod tests {
    use super::*;
    use port::register::register_feed_url_port::RegisterUrlPort;

    #[test]
    fn test_register_single_feed_url() {
        let register_url_gateway = RegisterUrlController;
        let url = "http://lorem-rss.herokuapp.com/feed".to_string();
        let result = register_url_gateway.register_single_feed_url(url);


        assert!(result.is_ok());
    }
}
