mod array_helper;
use fracgm::{registration, rotation, solver::GemanMcclureSolver};
use libc;

pub use array_helper::free_buf;

#[no_mangle]
pub extern "C" fn fracgm_rotation_solver(
    pc1: *const libc::c_double,
    pc1_n_rows: libc::size_t,
    pc1_n_cols: libc::size_t,
    pc2: *const libc::c_double,
    pc2_n_rows: libc::size_t,
    pc2_n_cols: libc::size_t,
    max_iteration: libc::size_t,
    tol: libc::c_double,
    noise_bound: libc::c_double,
    c: libc::c_double,
) -> array_helper::CBuffer {
    let solver = rotation::LinearSolver::new(
        max_iteration as usize,
        tol as f64,
        Some(noise_bound) as Option<f64>,
        Some(c) as Option<f64>,
    );

    let pc1 = array_helper::to_array2(pc1, pc1_n_rows, pc1_n_cols);
    let pc2 = array_helper::to_array2(pc2, pc2_n_rows, pc2_n_cols);

    let rot = GemanMcclureSolver::solve(&solver, &pc1, &pc2);

    array_helper::to_buffer(&rot)
}

#[no_mangle]
pub extern "C" fn fracgm_registration_solver(
    pc1: *const libc::c_double,
    pc1_n_rows: libc::size_t,
    pc1_n_cols: libc::size_t,
    pc2: *const libc::c_double,
    pc2_n_rows: libc::size_t,
    pc2_n_cols: libc::size_t,
    max_iteration: libc::size_t,
    tol: libc::c_double,
    noise_bound: libc::c_double,
    c: libc::c_double,
) -> array_helper::CBuffer {
    let solver = registration::LinearSolver::new(
        max_iteration as usize,
        tol as f64,
        Some(noise_bound) as Option<f64>,
        Some(c) as Option<f64>,
    );

    let pc1 = array_helper::to_array2(pc1, pc1_n_rows, pc1_n_cols);
    let pc2 = array_helper::to_array2(pc2, pc2_n_rows, pc2_n_cols);

    let solution = GemanMcclureSolver::solve(&solver, &pc1, &pc2);

    array_helper::to_buffer(&solution)
}
