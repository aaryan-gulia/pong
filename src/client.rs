pub async fn client(host: &String, port: &String) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);
    let client = tokio::net::TcpStream::connect(addr)
        .await
        .map_err(|_| "Failed to connect")?;
    let (mut reader, mut writer) = client.into_split();

    let client_read =
        tokio::spawn(async move { tokio::io::copy(&mut reader, &mut tokio::io::stdout()).await });

    let client_write =
        tokio::spawn(async move { tokio::io::copy(&mut tokio::io::stdin(), &mut writer).await });

    tokio::select!(
        _ = client_read => {},
        _ = client_write => {}
    );

    Ok(())
}
