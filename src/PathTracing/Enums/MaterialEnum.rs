use crate::PathTracing::Materials::Diffuse::Matte;
use crate::PathTracing::Materials::Mirror::Mirror;

#[derive(Copy, Clone)]
pub enum MaterialEnum {
    Matte(Matte),
    Mirror(Mirror)
}