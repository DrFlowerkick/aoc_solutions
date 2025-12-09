//!day_24.rs

use anyhow::{anyhow, Result};

// for part 2 I use an equation solver
use eqsolver::{multivariable::MultiVarNewton, nalgebra::{Matrix6, Vector6}};

struct HailStone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl From<&str> for HailStone {
    fn from(value: &str) -> Self {
        let (point_str, velocity_str) = value.split_once('@').unwrap();
        let mut point_iter = point_str
            .split(',')
            .map(|s| s.trim().parse::<f64>().expect("bad point input"));
        let mut velocity_iter = velocity_str
            .split(',')
            .map(|s| s.trim().parse::<f64>().expect("bad point input"));
        let result = Self {
            x: point_iter.next().expect("not enough point input"),
            y: point_iter.next().expect("not enough point input"),
            z: point_iter.next().expect("not enough point input"),
            vx: velocity_iter.next().expect("not enough velocity input"),
            vy: velocity_iter.next().expect("not enough velocity input"),
            vz: velocity_iter.next().expect("not enough velocity input"),
        };
        if point_iter.next().is_some() {
            panic!("too much point input");
        }
        if velocity_iter.next().is_some() {
            panic!("too much velocity input");
        }
        result
    }
}

impl HailStone {
    fn as_tuple(&self) -> (f64, f64, f64, f64, f64, f64) {
        (self.x, self.y, self.z, self.vx, self.vy, self.vz)
    }
    fn calc_position_on_path(&self, factor: f64) -> (f64, f64, f64) {
        (
            self.x + self.vx * factor,
            self.y + self.vy * factor,
            self.z + self.vz * factor,
        )
    }
    fn calc_intersection(&self, other: &Self, ignore_z: bool) -> Option<(f64, f64, f64)> {
        /*
        self:
        x = x1 + vx1 * t
        y = y1 + vy1 * t
        z = 0

        other:
        x = x2 + vx2 * s
        y = y2 + vy2 * s
        z = 0

        intersection, if x and y of self and other are equal
        x1 + vx1 * t = x2 + vx2 * s
        y1 + vy1 * t = y2 + vy2 * s

        --> t = (x2 - x1) / vx1 + vx2/vx1 * s
        --> y1 + vy1 * (x2 - x1) / vx1 + (vy1 * vx2 / vx1) * s = y2 + vy2 * s
        --> s * (vy2 - vy1*vx2/vx1) = y1 - y2 + vy1 * (x2 - x1) / vx1
        --> s = (y1 - y2 + (x2 - x1)*vy1/vx1) / (vy2 - vy1*vx2/vx1)

        do not test for special cases with vx, vy == 0, since all input is not zero

        special case parallel path (can never collide)
        vx1 * vy2 - vx2 * vy1 = 0

        An actual collision does happen, if t == s
        t or s is negativ, the intersection happened in the past -> return None

        if z is used, t and s are only valid, if
        t = (x2 - x1) / vx1 + vx2/vx1 * s
        is equal to
        t = (z2 - z1) / vz1 + vz2/vz1 * s
        -->
        */
        // check parallel paths
        if ignore_z && (self.vx * other.vy - self.vy * other.vx).abs() < f64::EPSILON {
            return None;
        }

        // calc parameters for x-y-plane
        let s = (self.y - other.y + (other.x - self.x) * self.vy / self.vx)
            / (other.vy - self.vy * other.vx / self.vx);
        let t = (other.x - self.x) / self.vx + s * other.vx / self.vx;

        // intersection in the past
        if s < 0.0 || t < 0.0 {
            return None;
        }

        if ignore_z {
            return Some(self.calc_position_on_path(t));
        }

        None
    }
    fn check_intersection_in_x_y_boundaries(&self, other: &Self, min: f64, max: f64) -> bool {
        match self.calc_intersection(other, true) {
            Some(intersection) => {
                intersection.0 >= min
                    && intersection.0 <= max
                    && intersection.1 >= min
                    && intersection.1 <= max
            }
            None => false,
        }
    }
}

fn solve_task_2(hailstones: &[HailStone]) -> Result<u64> {
    // part 2: a rock thrown from postion (xs, ys, zs) with velocity (vxs, vys, vzs) has to hit
    // all hailstones. This is obviously over-determined. The question is, how many hailstones do you need to solve it?
    // The rock has 6 unknown variables. To solve this this, we try with one hailstone.
    // looking only at x: at the time of crash tc we have with the first hailstone the following equation
    // xs + vxs*tc = x1 + vx1*tc
    // --> tc * (vxs - vx1) = x1 - xs
    // --> tc = (x1 - xs) / (vxs - vx1)
    // this mus be true also for y and z
    // --> tc = (x1 - xs) / (vxs - vx1) = (y1 - ys) / (vys - vy1) = (z1 - zs) / (vzs - vz1)
    // this results in two equations
    // (x1 - xs) / (vxs - vx1) = (y1 - ys) / (vys - vy1)
    // (y1 - ys) / (vys - vy1) = (z1 - zs) / (vzs - vz1)
    // now we need 4 more equations, which we get with two more hailstones
    // (x2 - xs) / (vxs - vx2) = (y2 - ys) / (vys - vy2)
    // (y2 - ys) / (vys - vy2) = (z2 - zs) / (vzs - vz2)
    // (x3 - xs) / (vxs - vx3) = (y3 - ys) / (vys - vy3)
    // (y3 - ys) / (vys - vy3) = (z3 - zs) / (vzs - vz3)
    // an alternativ equation, using x-z combination, is
    // (x1 - xs) / (vxs - vx1) = (z1 - zs) / (vzs - vz1)
    // (x1 - xs) * (vzs - vz1) - (z1 - zs) * (vxs - vx1) = 0
    // with Jacobi row
    // vz1 - vzs, 0, vxs - vx1, zs - z1, 0, x1 - xs
    // Therefore we need 3 hailstones. You can ignore all other hailstones. Because if you have a solution for this three hailstones, this solution
    // must fit either all other hailstones or there is not a solution at all.
    // To feed these equations in a equation solver, you must transform them to have "= 0" on one side
    // (x1 - xs) * (vys - vy1) - (y1 - ys) * (vxs - vx1) = 0
    // (y1 - ys) * (vzs - vz1) - (z1 - zs) * (vys - vy1) = 0
    // (x2 - xs) * (vys - vy2) - (y2 - ys) * (vxs - vx2) = 0
    // (y2 - ys) * (vzs - vz2) - (z2 - zs) * (vys - vy2) = 0
    // (x3 - xs) * (vys - vy3) - (y3 - ys) * (vxs - vx3) = 0
    // (y3 - ys) * (vzs - vz3) - (z3 - zs) * (vys - vy3) = 0

    // for eqsolver to work it does need the Jacobi Matrix of all 6 equations after all 6 unknown variables
    // see example: https://github.com/AzeezDa/eqsolver#multivariate-functions
    // Matrix in order of xs, ys, zs, vxs, vys, vzs and each row results from the corresponding equation
    // vy1 - vys, vxs - vx1, 0        , ys - y1, x1 - xs, 0
    // 0        , vz1 - vzs, vys - vy1, 0      , zs - z1, y1 - ys
    // vy2 - vys, vxs - vx2, 0        , ys - y2, x2 - xs, 0
    // 0        , vz2 - vzs, vys - vy2, 0      , zs - z2, y2 - ys
    // vy3 - vys, vxs - vx3, 0        , ys - y3, x3 - xs, 0
    // 0        , vz3 - vzs, vys - vy3, 0      , zs - z3, y3 - ys

    // alternativ approach to make it work with LU inversion of jacobi matrix
    // (x1 - xs) * (vys - vy1) - (y1 - ys) * (vxs - vx1) = 0
    // (x2 - xs) * (vys - vy2) - (y2 - ys) * (vxs - vx2) = 0
    // (x3 - xs) * (vzs - vz3) - (z3 - zs) * (vxs - vx3) = 0
    // (x1 - xs) * (vzs - vz1) - (z1 - zs) * (vxs - vx1) = 0
    // (y2 - ys) * (vzs - vz2) - (z2 - zs) * (vys - vy2) = 0
    // (y3 - ys) * (vzs - vz3) - (z3 - zs) * (vys - vy3) = 0

    // vy1 - vys, vxs - vx1, 0        , ys - y1, x1 - xs, 0
    // vy2 - vys, vxs - vx2, 0        , ys - y2, x2 - xs, 0
    // vz3 - vzs, 0        , vxs - vx3, zs - z3, 0      , x3 - xs
    // vz1 - vzs, 0        , vxs - vx1, zs - z1, 0      , x1 - xs
    // 0        , vz2 - vzs, vys - vy2, 0      , zs - z2, y2 - ys
    // 0        , vz3 - vzs, vys - vy3, 0      , zs - z3, y3 - ys

    // I'm not sure why it worked with these hailstone (before I tried with 0, 1, 2), but now I have the correct result :)
    let (x1, y1, z1, vx1, vy1, vz1) = hailstones[0].as_tuple();
    let (x2, y2, z2, vx2, vy2, vz2) = hailstones[3].as_tuple();
    let (x3, y3, z3, vx3, vy3, vz3) = hailstones[6].as_tuple();

    // Vector6: 0: xs, 1: ys, 2: zs, 3: vxs, 4: vys, 5: vzs
    let functions = |v: Vector6<f64>| {
        Vector6::new(
            // (x1 - xs) * (vys - vy1) - (y1 - ys) * (vxs - vx1) = 0
            (x1 - v[0]) * (v[4] - vy1) - (y1 - v[1]) * (v[3] - vx1),
            // (x2 - xs) * (vys - vy2) - (y2 - ys) * (vxs - vx2) = 0
            (x2 - v[0]) * (v[4] - vy2) - (y2 - v[1]) * (v[3] - vx2),
            // (x3 - xs) * (vzs - vz3) - (z3 - zs) * (vxs - vx3) = 0
            (x3 - v[0]) * (v[5] - vz3) - (z3 - v[2]) * (v[3] - vx3),
            // (x1 - xs) * (vzs - vz1) - (z1 - zs) * (vxs - vx1) = 0
            (x1 - v[0]) * (v[5] - vz1) - (z1 - v[2]) * (v[3] - vx1),
            // (y2 - ys) * (vzs - vz2) - (z2 - zs) * (vys - vy2) = 0
            (y2 - v[1]) * (v[5] - vz2) - (z2 - v[2]) * (v[4] - vy2),
            // (y3 - ys) * (vzs - vz3) - (z3 - zs) * (vys - vy3) = 0
            (y3 - v[1]) * (v[5] - vz3) - (z3 - v[2]) * (v[4] - vy3),
        )
    };

    // Jacobian of F
    let jacobi = |v: Vector6<f64>| {
        Matrix6::new(
            //  vy1 - vys  ,     vxs  - vx1,     0         , ys - y1  , x1 - xs  , 0
            vy1 - v[4],
            v[3] - vx1,
            0.,
            v[1] - y1,
            x1 - v[0],
            0.,
            //  vy2 - vys ,      vxs - vx2 , 0         , ys - y2  , x2 - xs  , 0
            vy2 - v[4],
            v[3] - vx2,
            0.,
            v[1] - y2,
            x2 - v[0],
            0.,
            // vz3 - vzs, 0        , vxs - vx3, zs - z3, 0      , x3 - xs
            vz3 - v[5],
            0.,
            v[3] - vx3,
            v[2] - z3,
            0.,
            x3 - v[0],
            // vz1 - vzs, 0        , vxs - vx1, zs - z1, 0      , x1 - xs
            vz1 - v[5],
            0.,
            v[3] - vx1,
            v[2] - z1,
            0.,
            x1 - v[0],
            // 0        , vz2 - vzs , vys - vy2 , 0        , zs - z2  , y2 - ys
            0.,
            vz2 - v[5],
            v[4] - vy2,
            0.,
            v[2] - z2,
            y2 - v[1],
            // 0        , vz3 - vzs , vys - vy3 , 0        , zs - z3  , y3 - ys
            0.,
            vz3 - v[5],
            v[4] - vy3,
            0.,
            v[2] - z3,
            y3 - v[1],
        )
    };

    let x_start = (x1 + x2 + x3) / 3.;
    let y_start = (y1 + y2 + y3) / 3.;
    let z_start = (z1 + z2 + z3) / 3.;
    let vx_start = (vx1 + vx2 + vx3) / 3.;
    let vy_start = (vy1 + vy2 + vy3) / 3.;
    let vz_start = (vz1 + vz2 + vz3) / 3.;

    let x0 = Vector6::new(x_start, y_start, z_start, vx_start, vy_start, vz_start);

    let solution: Vector6<f64> = match MultiVarNewton::new(functions, jacobi)
        .with_tol(1e-6)
        .with_itermax(500000)
        .solve(x0)
    {
        Ok(sol) => sol,
        Err(err) => return Err(anyhow!("err solution: {:?}", err)),
    };

    //println!("{:?}", solution);

    let solution = solution[0] + solution[1] + solution[2];
    Ok(solution as u64)
}

pub fn day_24() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_24.txt");
    let hailstones: Vec<HailStone> = input.lines().map(HailStone::from).collect();
    assert!(hailstones.iter().all(|h| h.vx.abs() > f64::EPSILON
        && h.vy.abs() > f64::EPSILON
        && h.vz.abs() > f64::EPSILON));

    let min = 200_000_000_000_000.0;
    let max = 400_000_000_000_000.0;

    let mut result_part1 = 0;
    for (i, hailstone_1) in hailstones.iter().enumerate() {
        for hailstone_2 in hailstones.iter().skip(i + 1) {
            if hailstone_1.check_intersection_in_x_y_boundaries(hailstone_2, min, max) {
                result_part1 += 1;
            }
        }
    }
    eprintln!("result day 24 part 1: {}", result_part1);
    assert_eq!(result_part1, 17_776);

    // task 2
    let result_part2 = solve_task_2(&hailstones)?;
    eprintln!("result day 24 part 2: {}", result_part2);
    assert_eq!(result_part2, 948_978_092_202_212);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part_1() -> Result<()> {
        let input = "19, 13, 30 @ -2,  1, -2\n\
                           18, 19, 22 @ -1, -1, -2\n\
                           20, 25, 34 @ -2, -2, -4\n\
                           12, 31, 28 @ -1, -2, -1\n\
                           20, 19, 15 @  1, -5, -3";
        let hailstones: Vec<HailStone> = input.lines().map(HailStone::from).collect();
        assert!(hailstones.iter().all(|h| h.vx.abs() > f64::EPSILON
            && h.vy.abs() > f64::EPSILON
            && h.vz.abs() > f64::EPSILON));

        let min = 7.0;
        let max = 27.0;

        let mut result_part1 = 0;
        for (i, hailstone_1) in hailstones.iter().enumerate() {
            for hailstone_2 in hailstones.iter().skip(i + 1) {
                if hailstone_1.check_intersection_in_x_y_boundaries(hailstone_2, min, max) {
                    result_part1 += 1;
                }
            }
        }
        eprintln!("result day 24 example part 1: {}", result_part1);
        assert_eq!(result_part1, 2);

        Ok(())
    }

    #[test]
    fn test_matrix_inversion() {
        let input = include_str!("../../../../aoc_input/aoc-2023/day_24.txt");
        let hailstones: Vec<HailStone> = input.lines().map(HailStone::from).collect();

        let (x1, y1, z1, vx1, vy1, vz1) = hailstones[0].as_tuple();
        let (x2, y2, z2, vx2, vy2, vz2) = hailstones[1].as_tuple();
        let (x3, y3, z3, vx3, vy3, vz3) = hailstones[2].as_tuple();

        // Jacobian of F
        let jacobi = |v: Vector6<f64>| {
            Matrix6::new(
                //  vy1 - vys  ,     vxs  - vx1,     0         , ys - y1  , x1 - xs  , 0
                vy1 - v[4],
                v[3] - vx1,
                0.,
                v[1] - y1,
                x1 - v[0],
                0.,
                //  vy2 - vys ,      vxs - vx2 , 0         , ys - y2  , x2 - xs  , 0
                vy2 - v[4],
                v[3] - vx2,
                0.,
                v[1] - y2,
                x2 - v[0],
                0.,
                // vz3 - vzs, 0        , vxs - vx3, zs - z3, 0      , x3 - xs
                vz3 - v[5],
                0.,
                v[3] - vx3,
                v[2] - z3,
                0.,
                x3 - v[0],
                // vz1 - vzs, 0        , vxs - vx1, zs - z1, 0      , x1 - xs
                vz1 - v[5],
                0.,
                v[3] - vx1,
                v[2] - z1,
                0.,
                x1 - v[0],
                // 0        , vz2 - vzs , vys - vy2 , 0        , zs - z2  , y2 - ys
                0.,
                vz2 - v[5],
                v[4] - vy2,
                0.,
                v[2] - z2,
                y2 - v[1],
                // 0        , vz3 - vzs , vys - vy3 , 0        , zs - z3  , y3 - ys
                0.,
                vz3 - v[5],
                v[4] - vy3,
                0.,
                v[2] - z3,
                y3 - v[1],
            )
        };

        let x_start = (x1 + x2 + x3) / 3.;
        let y_start = (y1 + y2 + y3) / 3.;
        let z_start = (z1 + z2 + z3) / 3.;
        let vx_start = (vx1 + vx2 + vx3) / 3.;
        let vy_start = (vy1 + vy2 + vy3) / 3.;
        let vz_start = (vz1 + vz2 + vz3) / 3.;

        let x0 = Vector6::new(x_start, y_start, z_start, vx_start, vy_start, vz_start);

        eprintln!("{:?}", jacobi(x0));
        eprintln!("{:?}", jacobi(x0).try_inverse());
        assert!(jacobi(x0).try_inverse().is_some());
    }
}
