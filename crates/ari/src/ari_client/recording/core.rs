use crate::*;

impl AriClient {
    pub async fn live_recording_get(&self, _recording_name: &str) -> AriClientResult<LiveRecording> {
        unimplemented!()
    }

    pub async fn live_recording_discard(&self, _recording_name: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    // TODO: explore if it's possible to return a StoredRecording
    pub async fn live_recording_stop(&self, _recording_name: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn live_recording_pause(&self, _recording_name: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn live_recording_resume(&self, _recording_name: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn live_recording_mute(&self, _recording_name: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn live_recording_unmute(&self, _recording_name: &str) -> AriClientResult<()> {
        unimplemented!()
    }
}

impl AriClient {
    pub async fn stored_recording_list(&self, _recording_name: &str) -> AriClientResult<Vec<StoredRecording>> {
        unimplemented!()
    }

    pub async fn stored_recording_get(&self, _recording_name: &str) -> AriClientResult<StoredRecording> {
        unimplemented!()
    }

    pub async fn stored_recording_delete(&self, _recording_name: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn stored_recording_download(&self, _recording_name: &str) -> AriClientResult<&[u8]> {
        unimplemented!()
    }
}
