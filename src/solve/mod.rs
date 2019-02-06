use super::cube;

fn phase1_finished(cb: cube::RubikCube) -> bool {
    use super::cube::EdgeCubePos::*;
    const udslices: [cube::EdgeCubePos; 4] = [FL, BL, BR, FR];
    let sbst = &cb.0;
    sbst.0.iter().all(|x| x.o == 0) && sbst.1.iter().all(|x| x.o == 0)
        && udslices
            .iter()
            .all(|pos| udslices.contains(&sbst.1[*pos as usize].e))
}
