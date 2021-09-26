mod pca;

use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod proto {
    tonic::include_proto!("bone_generator");
}

#[derive(Default)]
pub struct MyBoneGenerator {}

#[tonic::async_trait]
impl proto::bone_generator_server::BoneGenerator for MyBoneGenerator {
    async fn execute(
        &self,
        request: Request<proto::Members>,
    ) -> Result<Response<proto::Members>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let members = &request.get_ref().members;

        let parsed_request = members
            .iter()
            .map(|proto::Member { name, vertices }| {
                let points_cloud: Vec<pca::Vector3<f64>> = vertices
                    .iter()
                    .map(|vertex| pca::Vector3::new(vertex.x, vertex.y, vertex.z))
                    .collect();
                (name.to_string(), points_cloud)
            })
            .collect::<Vec<_>>();

        let parsed_reply = pca::compute_bones_generation(&parsed_request)
            .iter()
            .map(|(name, points_cloud)| {
                let vertices: Vec<proto::Vertex> = points_cloud
                    .iter()
                    .map(|point| proto::Vertex {
                        x: point.x,
                        y: point.y,
                        z: point.z,
                    })
                    .collect();
                proto::Member {
                    name: name.to_string(),
                    vertices,
                }
            })
            .collect();

        let reply = proto::Members {
            members: parsed_reply,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse().unwrap();
    let bone_generator = MyBoneGenerator::default();

    Server::builder()
        .add_service(proto::bone_generator_server::BoneGeneratorServer::new(
            bone_generator,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
