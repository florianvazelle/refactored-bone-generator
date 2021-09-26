use nalgebra::Matrix3;
pub use nalgebra::Vector3;
use ordered_float::OrderedFloat;

fn compute_barycenter(data: &[Vector3<f64>]) -> Vector3<f64> {
    let mut center = Vector3::new(0.0, 0.0, 0.0);

    for value in data {
        center += value;
    }

    let data_count = data.len() as f64;
    center /= data_count;

    center
}

fn transform_space(axis: &Vector3<f64>, data: &[Vector3<f64>]) -> Vec<Vector3<f64>> {
    data.iter().map(|value| value - axis).collect()
}

fn compute_covmatrix(data: &[Vector3<f64>]) -> Matrix3<f64> {
    let points_count = data.len() as f64; // nombre de points dans le nuage de points

    // On trouve la moyenne en x, y et z
    let mut average = Vector3::new(0.0, 0.0, 0.0);

    for point in data {
        average += point;
    }
    average /= points_count;

    // On initialise M : length x length (ici: 3x3)
    let mut cov_matrix = Matrix3::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    for i in 0..3 {
        for j in 0..3 {
            for point in data {
                cov_matrix[(i, j)] += (average[i] - point[i]) * (average[j] - point[j]);
                cov_matrix[(i, j)] /= points_count - 1.0;
            }
        }
    }

    cov_matrix
}

fn power_method(m: &Matrix3<f64>, num_simulations: Option<i32>) -> (f64, Vector3<f64>) {
    let num_simulations = num_simulations.unwrap_or(10);

    // 1) On choisit un vecteur initial
    let mut v = Vector3::new(0.0, 0.0, 1.0);
    let mut lambda = 0.0;

    // 2) Pour k de 0, 1, 2 ...
    for _ in 0..num_simulations {
        let v_normalized = v.normalize(); // On normalise V

        // a) On calcule M . Vk
        let m_dot_v = m * v_normalized;

        // b) On trouve la plus grande valeur absolu de M . Vk
        lambda = m_dot_v.amax();

        // c) On calcule V(k + 1)
        v = (1.0 / lambda) * m_dot_v;
    }

    // 3) On retourne le k plus grand lambda et V
    (lambda, v)
}

fn inv(point: &Vector3<f64>) -> Vector3<f64> {
    Vector3::new(-point[0], -point[1], -point[2])
}

pub fn compute_bones_generation(
    members: &[(String, Vec<Vector3<f64>>)],
) -> Vec<(String, Vec<Vector3<f64>>)> {
    let mut principal_components: Vec<(String, Vec<Vector3<f64>>)> =
        Vec::with_capacity(members.len());

    // Pour tout les membres
    for (key, points_cloud) in members.iter() {
        // ----- STEP 1 -----

        // a) On calcule le barycentre des données
        let barycenter = compute_barycenter(points_cloud);

        // b) On centre les données
        let center_points_cloud = transform_space(&barycenter, points_cloud); // to local space

        // c) On calcule la matrice de covariance
        let cov_matrix = compute_covmatrix(&center_points_cloud);

        // ----- STEP 2 -----

        // d) On calcule les valeurs propres (eigenvalue), et les vecteurs propres (eigenvector)
        let (eigenvalue, eigenvector) = power_method(&cov_matrix, Some(100)); // eigendecomposition of covariance matrix
        println!("eigenvalue: {} | eigenvector: {}", eigenvalue, eigenvector);

        // ----- STEP 3 -----

        // e) On projete les données centrées sur l'eigenvector
        let norm = eigenvector.norm(); // finding norm of the eigenvector
        let proj: Vec<Vector3<f64>> = center_points_cloud
            .iter()
            .map(|point| (point.dot(&eigenvector) / (norm * norm)) * eigenvector)
            .collect();

        // f) On récupère les deux extrémitées
        let max = *proj.iter().max_by_key(|n| OrderedFloat(n.norm())).unwrap();
        let min = *proj.iter().min_by_key(|n| OrderedFloat(n.norm())).unwrap();
        let center_pc = vec![max, min];

        // ----- STEP 4 -----

        // g) On repositionne les deux points
        let pc = transform_space(&inv(&barycenter), &center_pc); // to world space

        principal_components.push((key.to_string(), pc));
    }

    principal_components
}
