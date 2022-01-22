use crate::model;

struct OnePoint {
    vertices_index: usize,
    texcoords_index: usize,
    normals_index: usize,
}

struct OneFace(Vec<OnePoint>);

struct ModelObject {
    name: String,
    faces: Vec<OneFace>,
}

pub fn parse_obj(text: &str) -> Vec<model::Model> {
    let mut vertices = Vec::new();
    let mut texcoords = Vec::new();
    let mut normals = Vec::new();
    let mut objects = Vec::new();
    let mut cur_object = None;

    for line in text.lines() {
        if line.starts_with("o ") {
            if cur_object.is_some() {
                objects.push(cur_object.take().unwrap());
            }

            let parts: Vec<&str> = line.split(" ").collect();
            cur_object = Some(ModelObject {
                name: parts[1].to_string(),
                faces: Vec::new(),
            });
        } else if line.starts_with("v ") {
            let parts: Vec<&str> = line.split(" ").collect();
            let x = parts[1].parse::<f32>().unwrap();
            let y = parts[2].parse::<f32>().unwrap();
            let z = parts[3].parse::<f32>().unwrap();
            vertices.push((x, y, z));
        } else if line.starts_with("vt ") {
            let parts: Vec<&str> = line.split(" ").collect();
            let u = parts[1].parse::<f32>().unwrap();
            let v = parts[2].parse::<f32>().unwrap();
            texcoords.push((u, v));
        } else if line.starts_with("vn ") {
            let parts: Vec<&str> = line.split(" ").collect();
            let x = parts[1].parse::<f32>().unwrap();
            let y = parts[2].parse::<f32>().unwrap();
            let z = parts[3].parse::<f32>().unwrap();
            normals.push((x, y, z));
        } else if line.starts_with("f ") {
            if cur_object.is_some() {
                let mut face = Vec::new();
                let parts: Vec<&str> = line.split(" ").collect();
                for part in parts.iter().skip(1) {
                    let item: Vec<usize> = part
                        .split("/")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();
                    face.push(OnePoint {
                        vertices_index: item[0],
                        texcoords_index: item[1],
                        normals_index: item[2],
                    });
                }
                let mut obj = cur_object.unwrap();
                obj.faces.push(OneFace(face));
                cur_object = Some(obj);
            }
        } else {
        }
    }

    if cur_object.is_some() {
        objects.push(cur_object.take().unwrap());
    }

    convert_objects(objects, vertices, texcoords, normals)
}

fn convert_objects(
    objects: std::vec::Vec<ModelObject>,
    vertices: std::vec::Vec<(f32, f32, f32)>,
    texcoords: std::vec::Vec<(f32, f32)>,
    normals: std::vec::Vec<(f32, f32, f32)>,
) -> Vec<model::Model> {
    let mut models = Vec::new();

    for object in objects {
        let face_count = object.faces.len();
        let mut obj_vertices = Vec::new();
        let mut obj_texcoords = Vec::new();
        let mut obj_normals = Vec::new();
        let mut obj_indices = Vec::new();
        let mut base_index = 0;

        for i in 0..face_count {
            let OneFace(face) = &object.faces[i];
            for point in face.iter() {
                let vertex = vertices[point.vertices_index - 1];
                obj_vertices.push(vertex.0);
                obj_vertices.push(vertex.1);
                obj_vertices.push(vertex.2);

                let uv = texcoords[point.texcoords_index - 1];
                obj_texcoords.push(uv.0);
                obj_texcoords.push(uv.1);

                let normal = normals[point.normals_index - 1];
                obj_normals.push(normal.0);
                obj_normals.push(normal.1);
                obj_normals.push(normal.2);
            }

            let face_point_count = face.len(); // 这个面中，点的数量
            let triangle_count = face_point_count - 2; // 这个面中，三角形的数量
            for t in 0..triangle_count {
                obj_indices.push(base_index);
                obj_indices.push(base_index + t + 1);
                obj_indices.push(base_index + t + 2);
            }
            base_index += face_point_count;
        }

        models.push(model::Model {
            name: object.name,
            vertices: obj_vertices,
            texcoords: obj_texcoords,
            normals: obj_normals,
            colors: Vec::new(),
            indices: obj_indices.into_iter().map(|item| item as u16).collect(),
        });
    }

    models
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_obj() {
        let cube_obj = include_str!("../models/cube/cube1.obj");
        let _cube_models = parse_obj(cube_obj);
    }
}
