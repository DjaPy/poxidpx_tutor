use crate::domain::entities::PokemonNumber;
use std::sync::Arc;
use crate::repositories::pokemon::Repository;
use std::convert::TryFrom;

pub struct Request {
    pub number: u16,
}

pub enum Error {
    BadRequest,
    NotFound,
    Unknown
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<(), Error> {
    match PokemonNumber::try_from(req.number) {
        Ok(_) => Ok(()),
        Err(FetchOneError::NotFound) => Err(Error::NotFound),
        Err(FetchOneError::Unknown) => Err(Error::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::pokemon::InMemoryRepository;

    impl Request {
        fn new(number: PokemonNumber) -> Self {
            Self {
                number: u16::from(number),
            }
        }
    }

    #[test]
    fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        };
    }
    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::bad());

        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        };
    }
    #[test]
    fn it_should_return_a_not_found_error_when_the_repo_does_not_contain_the_pokemon() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request::new(PokemonNumber::pikachu());

        let res = execute(repo, req);

        match res {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        };
    }
}