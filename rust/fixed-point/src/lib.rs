#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7(127));
        assert_eq!(Q7::from(-10.), Q7(-128));
    }

    #[test]
    fn f32_to_q7() {
        assert_eq!(Q7::from(0.7), Q7(88));
        assert_eq!(Q7::from(-0.4), Q7(-50));
        assert_eq!(Q7::from(123.0), Q7(127));
    }

    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6875);

        let q2 = Q7::from(n1);
        let n2 = f32::from(q2);
        assert_eq!(n1, n2);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(f: f64) -> Self {
        if f >= 1.0 {
            Q7(127)
        } else if f <= -1.0 {
            Q7(-128)
        } else {
            Q7((f * 127.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(q: Q7) -> Self {
        (q.0 as f64) * 2f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(f: f32) -> Self {
        Q7::from(f as f64)
    }
}

impl From<Q7> for f32 {
    fn from(q: Q7) -> Self {
        f64::from(q) as f32
    }
}