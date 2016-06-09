// use super::Vector::Vector;
// use std::io;
// mod Vector;
const ZERO_F64: f64 = 0.0;

pub trait Vector {
    fn add_vectors(&mut self, a: &Self, b: &Self) -> &Self;
    fn add(&mut self, a: &Self) -> &Self;
    fn sub_vectors(&mut self, a: &Self, b: &Self) -> &Self;
    fn sub(&mut self, v: &Self) -> &Self;
    fn multiplyScalarVector<'a, 'b>(v: &'a mut Self, s: &'b f64) -> &'a Self;
    fn negate(&mut self) -> &Self;
    fn dot(&self, v: &Self) -> f64;
    fn lengthSq(&self) -> f64;
    fn length(&self) -> f64;
    fn lengthManhattan(&self) -> f64;
    fn distanceToSquared(&self, v: &Self) -> f64;
    fn crossVectors(&mut self, a: &Self, b: &Self) -> &Self;
    fn cross(&mut self, v: &Self) -> &Self;
    fn equals(&mut self, v: &Self) -> bool;
    fn min(&mut self, v: &Self) -> &Self;
    fn max(&mut self, v: &Self) -> &Self;
    fn floor(&mut self) -> &Self;
    fn ceil(&mut self) -> &Self;
    fn round(&mut self) -> &Self;
    fn clampScalar(&mut self, min: &f64, max: &f64) -> &Self;
    fn roundToZero(&mut self) -> &Self;
    fn divide(&mut self, v: &f64) -> &Self;
    fn lerp(&mut self, v: &Self, alpha: &f64) -> &Self;
    fn addScalar(&mut self, s: &f64) -> &Self;
    fn multiplyVectors(&mut self, a: &Self, b: &Self) -> &Self;

    fn setLength(&mut self, length: f64) -> &Self {
        let thisLength = self.length();
        self.multiplyScalar(&(length / thisLength))
    }

    fn normalize(&mut self) -> &Self {
        let length = self.length();
        self.divideScalar(&length)
    }

    fn distanceTo(&self, v: &Self) -> f64 {
        (self.distanceToSquared(v)).sqrt()
    }

    fn multiplyScalar(&mut self, s: &f64) -> &Self {
        Self::multiplyScalarVector(self, s)
    }

    fn divideScalar(&mut self, s: &f64) -> &Self {
        Self::multiplyScalarVector(self, &(1.0 / s))
    }

    fn clamp(&mut self, min: &Self, max: &Self) -> &Self {
        self.min(min);
        self.max(max);
        self
    }

    fn lerpVectors(&mut self, v1: &Self, v2: &Self, alpha: &f64) -> &Self {
        self.sub_vectors(v1, v2);
        self.multiplyScalar(alpha);
        self.add(v1)
    }
    // .reflect (normal) this
    // .multiply (v) this
    // .toArray ( array )
    // .projectOnVector (Vector3) this
    //
    // .setFromMatrixPosition ( m ) this
    // .setFromMatrixScale ( m ) this
    // .applyMatrix3 (m) this
    // .applyMatrix4 (m) this
    // .project ( camera )
    //
    // .projectOnPlane (planeNormal) this
    // .setComponent (index, value) this
    // .getComponent (index)
    // .applyAxisAngle (axis, angle) this
    // .transformDirection (m) this
    // .angleTo (v)
    // .setFromMatrixColumn (index, matrix) this
    // .fromArray (array) this
    // .applyProjection (m) this
    // .applyEuler (euler) this
    // .applyQuaternion (quaternion) this
    // .unproject ( camera )
    //
    fn copy(&mut self, from: &Self) -> &Self;
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(dead_code)]
impl Vector3 {
    pub fn set(&mut self, x: f64, y: f64, z: f64) -> &Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Vector for Vector3 {
    fn add_vectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.x + b.x;
        self.y = a.y + b.y;
        self.z = a.z + b.z;
        self
    }

    fn add(&mut self, a: &Self) -> &Self {
        self.x = a.x + self.x;
        self.y = a.y + self.y;
        self.z = a.z + self.z;
        self
    }

    fn sub_vectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.x - b.x;
        self.y = a.y - b.y;
        self.z = a.z - b.z;
        self
    }

    fn sub(&mut self, a: &Self) -> &Self {
        self.x = a.x - self.x;
        self.y = a.y - self.y;
        self.z = a.z - self.z;
        self
    }

    fn copy(&mut self, from: &Self) -> &Self {
        self.x = from.x.clone();
        self.y = from.y.clone();
        self.z = from.z.clone();
        self
    }

    fn multiplyScalarVector<'a, 'b>(v: &'a mut Self, s: &'b f64) -> &'a Self {
        v.x = v.x * s;
        v.y = v.y * s;
        v.z = v.y * s;
        v
    }

    fn negate(&mut self) -> &Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    fn dot(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    fn lengthSq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn lengthManhattan(&self) -> f64 {
        (self.x).abs() + (self.y).abs() + (self.z).abs()
    }

    fn distanceToSquared(&self, v: &Self) -> f64 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        let dz = self.z - v.z;

        dx * dx + dy * dy + dz * dz
    }

    fn crossVectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.y * b.z - a.z * b.y;
        self.y = a.z * b.x - a.x * b.z;
        self.z = a.x * b.y - a.y * b.x;

        self
    }

    fn cross(&mut self, v: &Self) -> &Self {
        self.x = self.y * v.z - self.z * v.y;
        self.y = self.z * v.x - self.x * v.z;
        self.z = self.x * v.y - self.y * v.x;

        self
    }

    fn equals(&mut self, v: &Self) -> bool {
        self.x == v.x && self.y == v.y && self.z == v.z
    }

    fn min(&mut self, v: &Self) -> &Self {
        self.x = self.x.min(v.x);
        self.y = self.y.min(v.y);
        self.z = self.z.min(v.z);

        self
    }

    fn max(&mut self, v: &Self) -> &Self {
        self.x = self.x.max(v.x);
        self.y = self.y.max(v.y);
        self.z = self.z.max(v.z);

        self
    }

    fn floor(&mut self) -> &Self {
        self.x.floor();
        self.y.floor();
        self.z.floor();

        self
    }

    fn ceil(&mut self) -> &Self {
        self.x.ceil();
        self.y.ceil();
        self.z.ceil();

        self
    }

    fn round(&mut self) -> &Self {
        self.x.round();
        self.y.round();
        self.z.round();

        self
    }

    fn clampScalar(&mut self, minVal: &f64, maxVal: &f64) -> &Self {
        // TODO optimize
        let minOwn = minVal.clone();
        let maxOwn = maxVal.clone();

        let min = Vector3 {
            x: minOwn,
            y: minOwn,
            z: minOwn,
        };

        let max = Vector3 {
            x: maxOwn,
            y: maxOwn,
            z: maxOwn,
        };

        self.clamp(&min, &max)
    }

    fn roundToZero(&mut self) -> &Self {

        if self.x < ZERO_F64 {
            self.x = self.x.ceil();
        } else {
            self.x = self.x.floor();
        }

        if self.y < ZERO_F64 {
            self.y = self.y.ceil();
        } else {
            self.y = self.y.floor();
        }

        if self.z < ZERO_F64 {
            self.z = self.z.ceil();
        } else {
            self.z = self.z.floor();
        }

        self
    }

    fn divide(&mut self, v: &f64) -> &Self {
        self.x = self.x / v;
        self.y = self.y / v;
        self.z = self.z / v;

        self
    }

    fn lerp(&mut self, v: &Self, alpha: &f64) -> &Self {
        self.x += (v.x - self.x) * alpha;
        self.y += (v.y - self.y) * alpha;
        self.z += (v.z - self.z) * alpha;

        self
    }

    fn addScalar(&mut self, s: &f64) -> &Self {
        self.x = self.x + s;
        self.y = self.y + s;
        self.z = self.z + s;

        self
    }

    fn multiplyVectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.x * b.x;
        self.y = a.y * b.y;
        self.z = a.z * b.z;

        self
    }
}









/// ///////////////////////////////////////////////////////////////////////////////////
/// ///////////////////////////////////////////////////////////////////////////////////
/// ///////////////////////////////////////////////////////////////////////////////////
/// ///////////////////////////////////////////////////////////////////////////////////
/// ///////////////////////////////////////////////////////////////////////////////////
/// ///////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
#[allow(dead_code)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Vector2 {
    pub fn set(&mut self, x: f64, y: f64) -> &Self {
        self.x = x;
        self.y = y;
        self
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vector for Vector2 {
    fn add_vectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.x + b.x;
        self.y = a.y + b.y;
        return self;
    }

    fn add(&mut self, a: &Self) -> &Self {
        self.x = a.x + self.x;
        self.y = a.y + self.y;
        return self;
    }

    fn sub_vectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.x - b.x;
        self.y = a.y - b.y;
        self
    }

    fn sub(&mut self, a: &Self) -> &Self {
        self.x = a.x - self.x;
        self.y = a.y - self.y;
        self
    }

    fn copy(&mut self, from: &Self) -> &Self {
        self.x = from.x.clone();
        self.y = from.y.clone();
        self
    }

    fn multiplyScalarVector<'a, 'b>(v: &'a mut Self, s: &'b f64) -> &'a Self {
        v.x = v.x * s;
        v.y = v.y * s;
        v
    }

    fn negate(&mut self) -> &Self {
        self.x = -self.x;
        self.y = -self.y;
        self
    }

    fn dot(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y
    }

    fn lengthSq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn lengthManhattan(&self) -> f64 {
        (self.x).abs() + (self.y).abs()
    }

    fn distanceToSquared(&self, v: &Self) -> f64 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;

        dx * dx + dy * dy
    }

    fn crossVectors(&mut self, a: &Self, b: &Self) -> &Self {
        let zero = 0.0;

        self.x = a.y * zero - zero * b.y;
        self.y = zero * b.x - a.x * zero;

        self
    }

    fn cross(&mut self, v: &Self) -> &Self {
        let zero = 0.0;
        self.x = self.y * zero - zero * v.y;
        self.y = zero * v.x - self.x * zero;

        self
    }

    fn equals(&mut self, v: &Self) -> bool {
        self.x == v.x && self.y == v.y
    }

    fn min(&mut self, v: &Self) -> &Self {
        self.x = self.x.min(v.x);
        self.y = self.y.min(v.y);

        self
    }

    fn max(&mut self, v: &Self) -> &Self {
        self.x = self.x.max(v.x);
        self.y = self.y.max(v.y);

        self
    }

    fn floor(&mut self) -> &Self {
        self.x.floor();
        self.y.floor();

        self
    }

    fn ceil(&mut self) -> &Self {
        self.x.ceil();
        self.y.ceil();

        self
    }

    fn round(&mut self) -> &Self {
        self.x.round();
        self.y.round();

        self
    }

    fn clampScalar(&mut self, minVal: &f64, maxVal: &f64) -> &Self {
        let minOwn = minVal.clone();
        let maxOwn = maxVal.clone();

        let min = Vector2 {
            x: minOwn,
            y: minOwn,
        };

        let max = Vector2 {
            x: maxOwn,
            y: maxOwn,
        };

        self.clamp(&min, &max)
    }

    fn roundToZero(&mut self) -> &Self {

        if self.x < ZERO_F64 {
            self.x = self.x.ceil();
        } else {
            self.x = self.x.floor();
        }

        if self.y < ZERO_F64 {
            self.y = self.y.ceil();
        } else {
            self.y = self.y.floor();
        }

        self
    }

    fn divide(&mut self, v: &f64) -> &Self {
        self.x = self.x / v;
        self.y = self.y / v;

        self
    }

    fn lerp(&mut self, v: &Self, alpha: &f64) -> &Self {
        self.x += (v.x - self.x) * alpha;
        self.y += (v.y - self.y) * alpha;

        self
    }

    fn addScalar(&mut self, s: &f64) -> &Self {
        self.x = self.x + s;
        self.y = self.y + s;

        self
    }

    fn multiplyVectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.x * b.x;
        self.y = a.y * b.y;
        
        self
    }

}
