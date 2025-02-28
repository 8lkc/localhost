use {
    super::Middleware,
    crate::{
        utils::{
            HttpErr,
            HttpResult,
        },
        Method,
        Request,
    },
};

impl<'a> Middleware<'a> {
    pub fn check(request: &'a Request) -> Self {
        Self { request }
    }

    pub fn method(&self, method: Method) -> HttpResult<&Self> {
        if self.request.method != method {
            Err(HttpErr::from(405))
        }
        else {
            Ok(self)
        }
    }

    pub fn logger(&self) -> HttpResult<&Self> {
        println!("{:?}", self.request);
        Ok(self)
    }

    // pub fn session(&self) -> HttpResult<&Self> {
    //     if self.request.session.is_none() {
    //         Err(HttpErr::from(401))
    //     }
    //     else {
    //         Ok(self)
    //     }
    // }
}
