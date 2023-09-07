use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello2/michael").await?.print().await?;

    // hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
