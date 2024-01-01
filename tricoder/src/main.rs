use tricoder::subdomains::enumerate;

fn main() -> Result<(), anyhow::Error> {
    let client = reqwest::blocking::Client::new();
    let result = enumerate(&client, "google.com")?;

    println!("[+] {:#?}", result);

    Ok(())
}
