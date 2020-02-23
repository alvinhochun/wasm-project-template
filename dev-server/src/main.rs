fn main() {
    let listen = "127.0.0.1:8000";
    println!("Dev server listening on {}", listen);
    rouille::start_server(listen, move |request| {
        use rouille::{match_assets, router};
        let response = match_assets(&request, "./www");
        if response.is_success() {
            return response;
        }
        let response = match_assets(&request, "./target/wasm-dist-dev");
        if response.is_success() {
            return response;
        }

        router!(request,
            (GET) ["/"] => {
                rouille::Response::redirect_302("/index.html")
            },

            _ => rouille::Response::empty_404(),
        )
    });
}
