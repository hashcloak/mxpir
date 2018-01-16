use xpir::client::PirClient;
use xpir::{PirReply, PirQuery};

pub struct MultiPirClient<'a> {
    handles: Vec<PirClient<'a>>,
}

impl<'a> MultiPirClient<'a> {

    pub fn new(buckets: &[(u64, u64)], alpha: u64, d: u64) -> MultiPirClient<'a> {
        let mut handles = Vec::with_capacity(buckets.len());

        for &(ele_num, ele_size) in buckets {
            handles.push(PirClient::with_params(ele_size, ele_num, alpha, d));
        }

        MultiPirClient { handles }
    }

    pub fn gen_query(&self, indexes: &[u64]) -> Vec<PirQuery> {
        let len = indexes.len();
        assert_eq!(len, self.handles.len());

        let mut queries = Vec::with_capacity(len);

        for (i, index) in indexes.iter().enumerate() {
            queries.push(self.handles[i].gen_query(*index));
        }

        queries
    }

    pub fn decode_replies<T: Clone>(&self, replies: &[PirReply]) -> Vec<T> {
        let len = replies.len();
        assert_eq!(len, self.handles.len());

        let mut results = Vec::with_capacity(len);

        for (i, handle) in self.handles.iter().enumerate() {
            results.push(handle.decode_reply(&replies[i]));
        }

        results
    }
}
