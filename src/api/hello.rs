#[macro_export]
macro_rules! hello {
    () => {
        hello_route::hello().and_then(hello_handler::hello).recover(hello_handler::handle_rejection)
    };
}
