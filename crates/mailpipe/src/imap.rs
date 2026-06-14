use tokio::net::TcpStream;
use tokio_native_tls::native_tls;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt}; 

/// Connector details for an IMAP connection
/// The password for authentication purposes is not stored in this struct, but rather a temporary sring slice
pub struct ImapConnector {
    /// The remote server hostname (eg. "imap.example.com")
    /// Do not include protocol schemes or port numbers
    pub server: String,

    /// The email address (eg. "user@example.com")
    pub email: String,
    
    /// The remote server port (defaults to 993)
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

    /// Connects to IMAP server and attempts to authenticate
    /// 
    /// # Arguments
    /// * `pass` - The temporary password or app password used to log in
    pub async fn connect(
        &self, 
        pass: &str
    ) -> Result<async_imap::Session<Compat<tokio_native_tls::TlsStream<TcpStream>>>, String> {
        
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

        let compat_stream = tls_stream.compat();

        println!("Logging in as {}...", self.email);

        let client = async_imap::Client::new(compat_stream);
        
        let session = client
            .login(&self.email, pass)
            .await
            .map_err(|(e, _unauth_client)| format!("IMAP login failed: {}", e))?;

        Ok(session)
    }
}
