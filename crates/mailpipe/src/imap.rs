use tokio::net::TcpStream;
use tokio_native_tls::native_tls;

/// Configuration for establishing a TLS-encrypted IMAP connection.
///
/// Passwords are not stored here; they are passed transiently to [`ImapConnector::connect`].
pub struct ImapConnector {
    /// The remote server hostname (e.g. `imap.example.com`).
    ///
    /// Do not include a protocol scheme or port number.
    pub server: String,

    /// The account email address (e.g. `user@example.com`).
    pub email: String,

    /// The remote server port. Defaults to `993` (standard IMAPS).
    pub port: u16,
}

impl ImapConnector {
    pub fn new(server: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            server: server.into(),
            email: email.into(),
            port: 993,
        }
    }

    /// Establishes a TLS connection to the IMAP server and authenticates.
    ///
    /// Returns an active [`async_imap::Session`] on success.
    ///
    /// # Arguments
    ///
    /// * `pass` - The password or app-specific password for the account.
    ///
    /// # Errors
    ///
    /// Returns a [`String`] describing the failure if TLS setup, the TCP
    /// connection, the TLS handshake, or IMAP login fails.
    pub async fn connect(
        &self,
        pass: &str,
    ) -> Result<async_imap::Session<tokio_native_tls::TlsStream<TcpStream>>, String> {
        let tls_connector = native_tls::TlsConnector::new()
            .map_err(|e| format!("Failed to initialize TLS: {}", e))?;

        let tokio_connector = tokio_native_tls::TlsConnector::from(tls_connector);

        let addr = format!("{}:{}", self.server, self.port);
        let tcp_stream = TcpStream::connect(&addr)
            .await
            .map_err(|e| format!("TCP connection failed: {}", e))?;

        let tls_stream = tokio_connector
            .connect(&self.server, tcp_stream)
            .await
            .map_err(|e| format!("TLS handshake failed: {}", e))?;

        println!("Logging in as {}...", self.email);

        let client = async_imap::Client::new(tls_stream);

        let session = client
            .login(&self.email, pass)
            .await
            .map_err(|(e, _unauth_client)| format!("IMAP login failed: {}", e))?;

        Ok(session)
    }
}
