pub async fn server() -> Result<(), String> {
    let listener = tokio::net::TcpListener::bind("localhost:2323")
        .await
        .map_err(|_| "failed to bind")?;
    let (handle, _) = listener.accept().await.map_err(|_| "failed to accept")?;

    let (mut reader, mut writer) = handle.into_split();

    let server_reader =
        tokio::spawn(async move { tokio::io::copy(&mut reader, &mut tokio::io::stdout()).await });
    let server_writer =
        tokio::spawn(async move { tokio::io::copy(&mut tokio::io::stdin(), &mut writer).await });

    tokio::select!(
        _ = server_reader => {},
        _ = server_writer => {}
    );

    Ok(())
}
