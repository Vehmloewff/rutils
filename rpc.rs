use tokio::sync::{mpsc, oneshot};

pub fn rpc<Request, Response>(buffer: usize) -> (RpcCaller<Request, Response>, RpcHandler<Request, Response>) {
	let (sender, receiver) = mpsc::channel(buffer);

	return (RpcCaller(sender), RpcHandler(receiver));
}

pub struct IncomingRequest<Request, Response> {
	pub request: Request,
	pub responder: RpcResponder<Response>,
}

#[derive(Clone)]
pub struct RpcCaller<Request, Response>(mpsc::Sender<IncomingRequest<Request, Response>>);

impl<Request, Response> RpcCaller<Request, Response> {
	pub async fn call(&self, request: Request) -> Option<Response> {
		let (sender, receiver) = oneshot::channel();

		self.0
			.send(IncomingRequest {
				request,
				responder: RpcResponder(sender),
			})
			.await
			.ok()?;

		Some(receiver.await.ok()?)
	}
}

pub struct RpcHandler<Request, Response>(mpsc::Receiver<IncomingRequest<Request, Response>>);

impl<Request, Response> RpcHandler<Request, Response> {
	pub async fn next(&mut self) -> Option<IncomingRequest<Request, Response>> {
		self.0.recv().await
	}
}

pub struct RpcResponder<Response>(oneshot::Sender<Response>);

impl<Response> RpcResponder<Response> {
	pub fn respond(self, response: Response) {
		let _ = self.0.send(response);
	}
}

#[macro_export]
macro_rules! drive {
	($condition:expr => $variable:ident $action:expr ) => {
		loop {
			let $variable = match $condition {
				Some(value) => value,
				None => break,
			};

			$action;
		}
	};

	($condition:expr => _ { $($action:stmt)* }) => {
		loop {
			let _ = match $condition {
				Some(value) => value,
				None => break,
			};

			$action;
		}
	};
}
