#[derive(Debug, Eq, PartialEq)]
pub enum IncusResponseStatus {
    OperationCreated = 100,
    Started = 101,
    Stopped = 102,
    Running = 103,
    Cancelling = 104,
    Pending = 105,
    Starting = 106,
    Stopping = 107,
    Aborting = 108,
    Freezing = 109,
    Frozen = 110,
    Thawed = 111,
    ErrorIncus = 112,
    Ready = 113,

    Success = 200,

    Failure = 400,
    Cancelled = 401,
}
impl TryFrom<u64> for IncusResponseStatus {
    type Error = ();

    fn try_from(code: u64) -> Result<Self, Self::Error> {
        match code {
            100 => Ok(IncusResponseStatus::OperationCreated),
            101 => Ok(IncusResponseStatus::Started),
            102 => Ok(IncusResponseStatus::Stopped),
            103 => Ok(IncusResponseStatus::Running),
            104 => Ok(IncusResponseStatus::Cancelling),
            105 => Ok(IncusResponseStatus::Pending),
            106 => Ok(IncusResponseStatus::Starting),
            107 => Ok(IncusResponseStatus::Stopping),
            108 => Ok(IncusResponseStatus::Aborting),
            109 => Ok(IncusResponseStatus::Freezing),
            110 => Ok(IncusResponseStatus::Frozen),
            111 => Ok(IncusResponseStatus::Thawed),
            112 => Ok(IncusResponseStatus::ErrorIncus),
            113 => Ok(IncusResponseStatus::Ready),
            200 => Ok(IncusResponseStatus::Success),
            400 => Ok(IncusResponseStatus::Failure),
            401 => Ok(IncusResponseStatus::Cancelled),
            _ => Err(()),
        }
    }
}
