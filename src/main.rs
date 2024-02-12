 mod server;

 use server::router;
fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
   
    let _ = router::start_server();

}
