use netsnap::NetSnap;


#[tokio::main]
async fn main() {
    let mut client = NetSnap::new("http://ofkr/")
        .config(9, 5, 500)
        .run()
        .await;

    // TODO: To use tests later, feel more free using bin for now

}
