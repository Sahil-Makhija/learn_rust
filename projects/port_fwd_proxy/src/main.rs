fn main() {
    println!("Hello, world!");

    println!(
        "Description : This is a proxy that helps forward requests/packets coming from a particular IP and forward them to a desired location.\nListens on all interfaces"
    );

    println!(
        "Command line arguments:\n\tSource IP(required),\n\tDestination IP (def:localhost),\n\tDestination Port(required)."
    );
}
