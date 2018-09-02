pub enum ExecState {
	Continue,
	Stop,
	Error(&'static str),
}
