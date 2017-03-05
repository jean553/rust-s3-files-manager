extern crate rusoto;
extern crate hyper;
extern crate core;

use hyper::Client;

use rusoto::s3::S3Client;
use rusoto::s3::CreateBucketRequest;

use rusoto::DefaultCredentialsProvider;
use rusoto::Region;

use core::default::Default;

fn main() {

    let client = Client::new();
    let provider = DefaultCredentialsProvider::new().unwrap();
    let s3_client = S3Client::new(
        client, 
        provider, 
        Region::EuCentral1,
        "http://s3:5000"
    );

    let bucket_request = CreateBucketRequest{
        bucket: "bucket_name".to_owned(),
        ..Default::default()
    };
    s3_client.create_bucket(&bucket_request);
}
