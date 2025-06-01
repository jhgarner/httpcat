use std::io::Read;

use super::*;

pub fn send(mut req: Request, file_name: &str, send_from: impl Read) -> Result<Status> {
    Ok(if let Some(save_as) = req.receive_multipart(Vec::new())? {
        let save_as = String::from_utf8(save_as)?;
        req.send_header(format!(
            "Content-Disposition: attachment; filename=\"{save_as}\""
        ))?;
        req.send_body_from(send_from)?;
        Status::Complete
    } else {
        req.send_body(format!(include_str!("download.html"), file_name))?;
        Status::Waiting
    })
}
