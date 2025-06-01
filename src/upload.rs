use std::io::Write;

use super::*;

pub fn save(mut req: Request, save_to: impl Write) -> Result<Status> {
    Ok(if req.receive_multipart(save_to)?.is_some() {
        req.send_body(include_str!("upload_complete.html"))?;
        Status::Complete
    } else {
        req.send_body(include_str!("upload.html"))?;
        Status::Waiting
    })
}
