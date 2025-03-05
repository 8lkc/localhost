use {
    super::Middleware,
    crate::{
        debug,
        message::{
            Method,
            Request,
        },
        utils::{
            HttpErr,
            HttpResult,
        },
    },
};

impl<'a> Middleware<'a> {
    pub fn check(request: &'a Request) -> Self { Self { request } }

    pub fn method(&self, method: Method) -> HttpResult<&Self> {
        if self.request.method != method {
            Err(debug!(HttpErr::from(405)))
        }
        else {
            Ok(self)
        }
    }

    pub fn logger(&self) -> HttpResult<&Self> {
        debug!(self.request);
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
