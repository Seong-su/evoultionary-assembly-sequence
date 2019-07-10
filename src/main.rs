extern crate assimp;

use assimp as ai;

fn main() {
    // Log to stdout and a file `log.txt`
    ai::LogStream::file("log/log");
    ai::LogStream::set_verbose_logging(true);

    let importer = ai::Importer::new();

    // The file to import
    let scene = importer.read_file("data/Casing.step").unwrap();

    // Print all the vertices in all the meshes
    for mesh in scene.mesh_iter() {
        for vert in mesh.vertex_iter() {
            println!("{:?}", vert);
        }
    }
}