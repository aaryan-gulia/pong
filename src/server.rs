pub async fn server(host: &String, port: &String) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|_| "failed to bind")?;
    let (handle, _) = listener.accept().await.map_err(|_| "failed to accept")?;
    let (mut reader_1, mut writer_1) = handle.into_split();
    let (handle, _) = listener.accept().await.map_err(|_| "failed to accept")?;
    let (mut reader_2, mut writer_2) = handle.into_split();
    let server_1 = tokio::spawn(async move { tokio::io::copy(&mut reader_1, &mut writer_2).await });
    let server_2 = tokio::spawn(async move { tokio::io::copy(&mut reader_2, &mut writer_1).await });

    tokio::select!(
        _ = server_1 => {},
        _ = server_2 => {}
    );

    Ok(())
}
