use crate::app_context::DownloadProgressSnapshot;

#[cfg(test)]
pub fn update_download_progress(client_id: i32, file: &tdlib_rs::types::File) {
    crate::app_context::app_context()
        .download_progress
        .update_download_progress(client_id, file);
}

pub fn get_download_progress(client_id: i32, file_id: i32) -> Option<DownloadProgressSnapshot> {
    crate::app_context::app_context()
        .download_progress
        .get_download_progress(client_id, file_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_progress_is_isolated_by_client_id() {
        let file_id = 42;
        update_download_progress(10, &test_file(file_id, 100, 1000, false, true));
        update_download_progress(20, &test_file(file_id, 700, 1000, false, true));

        let first = get_download_progress(10, file_id).expect("first client progress");
        let second = get_download_progress(20, file_id).expect("second client progress");

        assert_eq!(first.downloaded_size, 100);
        assert_eq!(second.downloaded_size, 700);
    }

    fn test_file(
        id: i32,
        downloaded_size: i64,
        size: i64,
        is_downloading_completed: bool,
        is_downloading_active: bool,
    ) -> tdlib_rs::types::File {
        tdlib_rs::types::File {
            id,
            size,
            expected_size: 0,
            local: tdlib_rs::types::LocalFile {
                path: String::new(),
                can_be_downloaded: true,
                can_be_deleted: true,
                is_downloading_active,
                is_downloading_completed,
                download_offset: 0,
                downloaded_prefix_size: 0,
                downloaded_size,
            },
            remote: tdlib_rs::types::RemoteFile {
                id: String::new(),
                unique_id: String::new(),
                is_uploading_active: false,
                is_uploading_completed: false,
                uploaded_size: 0,
            },
        }
    }
}
