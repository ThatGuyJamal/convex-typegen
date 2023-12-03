use convex::ConvexClient;

mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()>
{
    let mut client = match ConvexClient::new("https://zealous-bobcat-355.convex.cloud").await {
        Ok(client) => client,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let result = match client.query(schema::Users::Find.to_string(), maplit::btreemap! {
        "id".into() => "3sx2tezsnsawa05fzhyvevsq9kq80s8".into()
    }).await {
        Ok(result) => result,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    println!("Result: {:?}", result);

    Ok(())
}
