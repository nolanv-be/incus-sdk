use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Status {
    OperationCreated,
    Started,
    Stopped,
    Running,
    Cancelling,
    Pending,
    Success,
    Failure,
    Cancelled,
    Starting,
    Stopping,
    Aborting,
    Freezing,
    Frozen,
    Thawed,
    Error,
    Ready,
}

impl Into<usize> for Status {
    fn into(self) -> usize {
        match self {
            Status::OperationCreated => 100,
            Status::Started => 101,
            Status::Stopped => 102,
            Status::Running => 103,
            Status::Cancelling => 104,
            Status::Pending => 105,
            Status::Success => 106,
            Status::Failure => 107,
            Status::Cancelled => 108,
            Status::Starting => 109,
            Status::Stopping => 110,
            Status::Aborting => 111,
            Status::Freezing => 112,
            Status::Frozen => 113,
            Status::Thawed => 200,
            Status::Error => 400,
            Status::Ready => 401,
        }
    }
}
