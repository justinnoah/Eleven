extern crate libeleven;
#[macro_use]
extern crate rustless;
extern crate serde_json;
extern crate url;

#[macro_export]
macro_rules! resp_body {
    ($resp:ident) => {
        {
            use ::rustless::backend::WriteBody;
            let mut vec = Vec::new();

            $resp.body.unwrap().write_body(&mut ::rustless::ResponseBody::new(&mut vec)).expect("Can't write");
            String::from_utf8(vec).unwrap()
        }
    }
}

#[macro_export]
macro_rules! sr {
    ($method:ident, $url:expr) => {
        ::rustless::SimpleRequest::new(::rustless::server::method::Method::$method, ::url::Url::parse($url).unwrap())
    };
    ($method:ident, $url:expr, $blk:expr) => {
        ::rustless::SimpleRequest::build(::rustless::server::method::Method::$method, ::url::Url::parse($url).unwrap(), $blk)
    };
}

#[macro_export]
macro_rules! call_app {
    ($app:ident, $method:ident, $url:expr) => {
        $app.call(&mut sr!($method, $url))
    };
    ($app:ident, $method:ident, $url:expr, $blk:expr) => {
        $app.call(&mut sr!($method, $url, $blk))
    };
}

#[macro_export]
macro_rules! app {
    ($builder:ident) => ({
        let app = ::rustless::Application::new($builder);
        app
    })
}

mod api_standards;
