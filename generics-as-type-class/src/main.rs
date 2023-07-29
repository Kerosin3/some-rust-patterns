use proto_trait::Bootp;
use proto_trait::Nfs;
use proto_trait::ProtoKind;
fn main() {
    let btp = proto_trait::Bootp();
    let proto_bootp = FileDownloader {
        filename: "file_to_download_bootp".to_string(),
        protocol: btp,
    };
    println!("{}", proto_bootp.get_file_name());
    println!("auth info : {:?}", proto_bootp.get_auth_info());
    println!("NFS DONE!-------------");

    let nts = proto_trait::Nfs {
        auth: nfs::AuthInfo("alex-username".to_string()),
        mount_point: "some path".to_string(),
    };
    let proto_nfs = FileDownloader {
        filename: "filename_for_nfs".to_string(),
        protocol: nts,
    };
    println!("{}", proto_nfs.get_file_name());
    println!(
        "nfs additional info: {:?} {:?}",
        proto_nfs.mount_point(),
        proto_nfs.get_auth_info()
    );
}

struct FileDownloader<P: ProtoKind> {
    filename: String,
    protocol: P,
}
// common methods
impl<P: ProtoKind> FileDownloader<P> {
    fn get_file_name(&self) -> String {
        self.filename.to_owned()
    }
    // get auth info
    fn get_auth_info(&self) -> P::AuthInfo {
        self.protocol.auth_info()
    }
}
// impl for nfs
impl FileDownloader<Nfs> {
    fn mount_point(&self) -> String {
        self.protocol.mount_point()
    }
}

mod nfs {
    #[derive(Clone, Debug)]
    pub struct AuthInfo(pub String);
}

mod bootp {
    #[derive(Clone, Debug)]
    pub struct AuthInfo(); // no auth info
}

mod proto_trait {
    use super::{bootp, nfs};

    // return string from nfs and nothing when bootp
    pub trait ProtoKind {
        type AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo;
    }

    pub struct Nfs {
        pub auth: nfs::AuthInfo,
        pub mount_point: String,
    }
    impl Nfs {
        pub fn new(auth_i: nfs::AuthInfo) -> Self {
            Self {
                auth: auth_i,
                mount_point: "default".to_string(),
            }
        }
        pub fn mount_point(&self) -> String {
            self.mount_point.to_owned()
        }
    }
    impl ProtoKind for Nfs {
        type AuthInfo = nfs::AuthInfo;

        fn auth_info(&self) -> Self::AuthInfo {
            self.auth.clone()
        }
    }

    pub struct Bootp();
    impl ProtoKind for Bootp {
        type AuthInfo = bootp::AuthInfo;

        fn auth_info(&self) -> Self::AuthInfo {
            bootp::AuthInfo()
        }
    }
}
