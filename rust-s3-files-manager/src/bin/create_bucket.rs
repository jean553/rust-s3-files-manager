extern crate rusoto;
extern crate hyper;

use hyper::Client;

use rusoto::s3::S3Client;
use rusoto::DefaultCredentialsProvider;
use rusoto::Region;

fn main() {

    let client = Client::new();
    let provider = DefaultCredentialsProvider::new().unwrap();
    let s3_client = S3Client::new(
        client, 
        provider, 
        Region::EuCentral1
    );
}
