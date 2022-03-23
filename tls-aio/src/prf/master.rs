use futures_util::{SinkExt, StreamExt};
use mpc_aio::garble::{Evaluator, Generator};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};
struct PrfMaster;

impl PrfMaster {
    pub fn new() -> Self {
        Self
    }

    pub async fn run<S: AsyncWrite + AsyncRead + Unpin>(
        &mut self,
        stream: &mut WebSocketStream<S>,
    ) -> Result<(), ()> {
        todo!()
    }
}
