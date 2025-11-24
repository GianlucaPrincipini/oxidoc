mod storage {
    pub mod format {
        pub mod kv {
            pub mod collection;
            pub mod db;
        }
    }
    pub mod io {
        pub mod writer;
        pub mod reader;
    }
}
pub mod cli {
    pub mod cli;
}
