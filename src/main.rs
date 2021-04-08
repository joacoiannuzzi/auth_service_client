use service::auth_service_client::AuthServiceClient;
use service::AuthenticateReq;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://localhost:50051")
        .connect()
        .await?;
    // creating gRPC client from channel
    let mut client = AuthServiceClient::new(channel);

    let r = AuthenticateReq {
        mail: "name@gmail.com".to_string(),
        password: "mypassword".to_string(),
    };
    // creating a new Request
    let request = tonic::Request::new(r.clone());
    // sending request and waiting for response
    let response = client.authenticate(request).await?.into_inner();
    println!("RESPONSE for req={:?} {:?}", r, response);

    ////////////////////////
    let r = AuthenticateReq {
        mail: "name@gmail.com".to_string(),
        password: "notvalid".to_string(),
    };
    // creating a new Request
    let request = tonic::Request::new(r.clone());
    // sending request and waiting for response
    let response = client.authenticate(request).await?.into_inner();
    println!("RESPONSE for req={:?} {:?}", r, response);

    ////////////////////////
    let r = AuthenticateReq {
        mail: "notvalid@gmail.com".to_string(),
        password: "notvalid".to_string(),
    };
    // creating a new Request
    let request = tonic::Request::new(r.clone());
    // sending request and waiting for response
    let response = client.authenticate(request).await?.into_inner();
    println!("RESPONSE for req={:?} {:?}", r, response);
    Ok(())
}
