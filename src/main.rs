use netsnap::NetSnap;


#[tokio::main]
async fn main() {
    let mut client = NetSnap::new("github.com").port(80).run().await;

    // TODO: To use tests later, feel more free using bin for now

}
