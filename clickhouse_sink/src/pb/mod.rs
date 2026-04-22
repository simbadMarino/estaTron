// @generated
pub mod google {
    // @@protoc_insertion_point(attribute:google.api)
    pub mod api {
        include!("google.api.rs");
        // @@protoc_insertion_point(google.api)
    }
}
// @@protoc_insertion_point(attribute:protocol)
pub mod protocol {
    include!("protocol.rs");
    // @@protoc_insertion_point(protocol)
}
pub mod sf {
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod index {
            // @@protoc_insertion_point(attribute:sf.substreams.index.v1)
            pub mod v1 {
                include!("sf.substreams.index.v1.rs");
                // @@protoc_insertion_point(sf.substreams.index.v1)
            }
        }
        pub mod rpc {
            // @@protoc_insertion_point(attribute:sf.substreams.rpc.v2)
            pub mod v2 {
                include!("sf.substreams.rpc.v2.rs");
                // @@protoc_insertion_point(sf.substreams.rpc.v2)
            }
        }
        pub mod sink {
            pub mod database {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.database.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.database.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.database.v1)
                }
            }
            pub mod service {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.service.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.service.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.service.v1)
                }
            }
            pub mod sql {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.sql.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.sql.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.sql.v1)
                }
            }
        }
        pub mod tron {
            // @@protoc_insertion_point(attribute:sf.substreams.tron.v1)
            pub mod v1 {
                include!("sf.substreams.tron.v1.rs");
                // @@protoc_insertion_point(sf.substreams.tron.v1)
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
    pub mod tron {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.tron.type.v1)
            pub mod v1 {
                include!("sf.tron.type.v1.rs");
                // @@protoc_insertion_point(sf.tron.type.v1)
            }
        }
    }
}
