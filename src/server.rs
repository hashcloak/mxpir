use xpir::server::PirServer;
use xpir::{PirQuery, PirReply};

pub struct MultiPirServer<'a> {
    handles: Vec<PirServer<'a>>,
}

impl<'a> MultiPirServer<'a> {

    pub fn new<T>(collection: &[Vec<T>], alpha: u64, d: u64)->MultiPirServer<'a> {
        let mut handles = Vec::with_capacity(collection.len());

        for bucket in collection {
            let mut server = PirServer::with_params(bucket, alpha, d);
            handles.push(server);
        }

        MultiPirServer { handles }
    }

    pub fn gen_replies(&self, queries: &[PirQuery]) -> Vec<PirReply> {
        let len = queries.len();
        assert_eq!(len, self.handles.len());

        let mut answers = Vec::with_capacity(len);

        for (i, query) in queries.iter().enumerate() {
            answers.push(self.handles[i].gen_reply(query));
        }

        answers
    }
}
