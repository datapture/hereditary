
mod geom
{

    #[derive(Clone)]
    pub struct Vec2
    {
        pub x:f64,
        pub y:f64
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub enum Colors
    {
        TRANSPARENT,
        RED,
        GREEN,
        BLUE
    }

    impl Vec2
    {
        pub fn new(x:f64, y:f64) -> Self
        {
            Self{x,y}
        }

        pub fn len_sqr(&self) -> f64
        {
            self.x*self.x + self.y*self.y
        }

        pub fn lenght(&self) -> f64
        {
            (self.x*self.x + self.y*self.y).sqrt()
        }

        pub fn absolute(self) -> Self
        {
            Self{x:self.x.abs(), y:self.y.abs()}
        }
    }

    impl From<(f64, f64)> for Vec2
    {
        fn from(value: (f64, f64)) -> Self {
            Vec2{x:value.0, y:value.1}
        }
    }

    impl core::ops::Add for Vec2
    {
        type Output = Vec2;

        fn add(self, rhs: Self) -> Self::Output {
            Vec2{x:self.x + rhs.x, y:self.y + rhs.y}
        }
    }

    impl core::ops::Sub for Vec2
    {
        type Output = Vec2;

        fn sub(self, rhs: Self) -> Self::Output {
            Vec2{x:self.x - rhs.x, y:self.y - rhs.y}
        }
    }

    impl core::ops::Neg for Vec2
    {
        type Output = Vec2;

        fn neg(self) -> Self::Output {
            Vec2{x:-self.x, y:-self.y}
        }
    }

    impl core::ops::Mul<f64> for Vec2
    {
        type Output = Vec2;

        fn mul(self, rhs: f64) -> Self::Output {
            Vec2{x: rhs*self.x, y: rhs*self.y}
        }
    }

    impl core::ops::Mul<Vec2> for f64
    {
        type Output = Vec2;

        fn mul(self, rhs: Vec2) -> Self::Output {
            Vec2{x: self*rhs.x, y: self*rhs.y}
        }
    }

}

mod ifaces
{
    #[hereditary::trait_info]
    pub trait Polytope
    {
        fn num_vertices(&self) -> usize;
        fn get_vertex(&self, index:usize) -> Option<crate::geom::Vec2>;
    }

    #[hereditary::trait_info]
    pub trait Bound
    {
        fn get_radius(&self) -> f64;        
        fn sizes_wh(&self) -> (f64,f64);
        
        fn get_width(&self) -> f64
        {
            self.sizes_wh().0
        }

        fn get_height(&self) -> f64
        {
            self.sizes_wh().1
        }
    }

    #[hereditary::trait_info]
    pub trait Measures
    {
        fn center(&self) -> crate::geom::Vec2;
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }

    #[hereditary::trait_info]
    pub trait Intersects
    {
        fn point_collides(&self, point:&crate::geom::Vec2) -> bool;
    }

    #[hereditary::trait_info]
    pub trait Material
    {
        fn get_color(&self) -> crate::geom::Colors;
        fn is_solid(&self) -> bool;
        fn cast_shadow(&self, light_point:&crate::geom::Vec2) -> crate::geom::Vec2;
        fn reflect_light(&self, light_point:&crate::geom::Vec2) -> crate::geom::Vec2;
    }

}

mod shapes
{
    use crate::{geom::Vec2, ifaces::Measures};

    pub struct ColoredCircle
    {
        pub center:Vec2,
        pub radius:f64,
        pub color: crate::geom::Colors
    }

    impl ColoredCircle
    {
        pub fn create(x:f64, y:f64, radius:f64, color:crate::geom::Colors) -> Self
        {
            Self{center:Vec2{x,y}, radius, color}
        }            
    }

    pub struct Triangle
    {
        pub vertices:[Vec2;3]
    }

    impl Triangle
    {
        pub fn create(
            x0:f64, y0:f64,
            x1:f64, y1:f64,
            x2:f64, y2:f64
        ) -> Self
        {
            Self{vertices:[
                Vec2{x:x0,y:y0},
                Vec2{x:x1,y:y1},
                Vec2{x:x2,y:y2},
            ]}
        }
    }

    pub struct Rectangle
    {
        pub width:f64,
        pub height:f64
    }

    impl Rectangle
    {
        pub fn create(width:f64, height:f64) -> Self
        {
            Self{width, height}
        }
    }

    // Circle traits
    impl crate::ifaces::Bound for ColoredCircle
    {
        fn get_radius(&self) -> f64 {
            self.radius
        }

        fn sizes_wh(&self) -> (f64,f64)
        {
            let diameter = self.radius*2f64;
            (diameter, diameter)
        }
    }

    impl crate::ifaces::Measures for ColoredCircle
    {
        fn center(&self) -> crate::geom::Vec2 {
            self.center.clone()
        }

        fn area(&self) -> f64 {
            self.radius*self.radius*std::f64::consts::PI
        }

        fn perimeter(&self) -> f64 {
            self.radius*2.0_f64*std::f64::consts::PI
        }
    }

    impl crate::ifaces::Material for ColoredCircle
    {
        fn get_color(&self) -> crate::geom::Colors {
            self.color.clone()
        }

        fn is_solid(&self) -> bool {            
            match self.color {
                crate::geom::Colors::TRANSPARENT => false,
                _ => true
            }
        }

        fn cast_shadow(&self, light_point:&crate::geom::Vec2) -> crate::geom::Vec2 {
            let vdir = self.center.clone() - light_point.clone();
            vdir + self.center.clone()
        }

        fn reflect_light(&self, light_point:&crate::geom::Vec2) -> crate::geom::Vec2 {
            let vdir = light_point.clone() - self.center.clone();
            vdir + light_point.clone()
        }
    }

    impl crate::ifaces::Intersects for ColoredCircle
    {
        fn point_collides(&self, point:&crate::geom::Vec2) -> bool {
            let vdiff = point.clone() - self.center.clone();
            if vdiff.len_sqr() < (self.radius*self.radius) {true} else{false}
        }
    }

    // Triangle traits
    impl crate::ifaces::Measures for Triangle{
        fn center(&self) -> crate::geom::Vec2 {
            let vsum = self.vertices.iter().fold(
                Vec2{x:0f64, y:0f64},
                |ar, new| ar + new.clone() 
            );

            vsum*(1f64/3f64)
        }

        fn area(&self) -> f64 {
            let p0 = self.vertices[0].clone();
            let v1 = self.vertices[1].clone() - p0.clone();
            let v2 = self.vertices[2].clone() - p0;
            // cross product
            (v1.x*v2.y - v1.y*v2.x)*0.5f64
        }

        fn perimeter(&self) -> f64 {
            let p0 = self.vertices[0].clone();
            let p1 = self.vertices[1].clone();
            let p2 = self.vertices[2].clone();
            let v1 = p1.clone() - p0.clone();
            let v2 = p2.clone() - p1.clone();
            let v3 = p0 - p2;
            v1.lenght() + v2.lenght() + v3.lenght()
        }
    }

    
    impl crate::ifaces::Bound for Triangle
    {
        fn get_radius(&self) -> f64 {
            let vcenter = self.center();
            
            let maxrad = self.vertices.iter().fold(0f64, |ar, nvec| -> f64 {
                let vdiff = nvec.clone() - vcenter.clone();
                let sqdis = vdiff.len_sqr();
                sqdis.max(ar)
            });

            maxrad.sqrt()
        }

        fn sizes_wh(&self) -> (f64,f64)
        {
            let vcenter = self.center();
            
            let maxvec = self.vertices.iter().fold((0f64,0f64).into(), |ar, nvec| -> Vec2 {
                let vdiff = (nvec.clone() - vcenter.clone()).absolute();
                Vec2{x:vdiff.x.max(ar.x), y:vdiff.y.max(ar.y)}
            })*2.0f64;

            (maxvec.x, maxvec.y)
        }

    }

    impl crate::ifaces::Polytope for Triangle
    {
        fn num_vertices(&self) -> usize {
            3
        }

        fn get_vertex(&self, index:usize) -> Option<crate::geom::Vec2> {
            self.vertices.get(index).map(|p|p.clone())
        }
    }

    // Rectangle Traits
    impl crate::ifaces::Measures for Rectangle
    {
        fn center(&self) -> crate::geom::Vec2 {
            Vec2{x:self.width*0.5f64, y:self.height*0.5f64}
        }

        fn area(&self) -> f64 {
            self.width*self.height
        }

        fn perimeter(&self) -> f64 {
            (self.width + self.height)*2f64
        }
    }

    impl crate::ifaces::Bound for Rectangle
    {
        fn get_radius(&self) -> f64 {
            self.height.max(self.width)*0.5
        }

        fn sizes_wh(&self) -> (f64,f64) {
            (self.width, self.height)
        }

        fn get_width(&self) -> f64 {
            self.width
        }

        fn get_height(&self) -> f64 {
            self.height
        }
    }

    impl crate::ifaces::Polytope for Rectangle
    {
        fn num_vertices(&self) -> usize {
            4
        }

        fn get_vertex(&self, index:usize) -> Option<crate::geom::Vec2> {
            match index {
                0 => Some(Vec2::new(0f64, 0f64)),                
                1 => Some(Vec2::new(self.width, 0f64)),
                2 => Some(Vec2::new(self.width, self.height)),
                3 => Some(Vec2::new(0f64, self.height)),
                _ => None
            }
        }
    }
    
}

#[derive(hereditary::Forwarding)]
struct Kimera
{
    #[forward_derive(ifaces::Polytope)]
    trishape:shapes::Triangle,
    #[forward_derive(ifaces::Material,ifaces::Intersects)]
    circle:shapes::ColoredCircle,
    boundbox:shapes::Rectangle
}


use ifaces::Measures;
use ifaces::Bound;
use ifaces::Intersects;
use ifaces::Material;

impl Kimera
{
    fn create(
        color:geom::Colors,
        x0:f64, y0:f64,
        x1:f64, y1:f64,
        x2:f64, y2:f64
    ) -> Self
    {
        let tri = shapes::Triangle::create(x0, y0, x1, y1, x2, y2);
        let vcenter = tri.center();
        let (width, height) = tri.sizes_wh();
        let rect = shapes::Rectangle::create(width, height);
        let circ = shapes::ColoredCircle::create(vcenter.x, vcenter.y, rect.get_radius(), color);
        Self{trishape:tri, circle:circ, boundbox:rect}
    }
}

#[hereditary::forward_trait(boundbox)]
impl ifaces::Bound for Kimera
{
    fn get_radius(&self) -> f64 {
        self.circle.radius
    }

    /*
    Should generate these methods like this:
    fn sizes_wh(&self) -> (f64, f64) {self.boundbox.sizes_wh()}
    fn get_width(&self) -> f64 {self.boundbox.get_width()}
    fn get_height(&self) -> f64 {self.boundbox.get_height()}
    */
}

#[hereditary::forward_trait(trishape)]
impl ifaces::Measures for Kimera
{
    fn center(&self) -> crate::geom::Vec2 {
        self.circle.center()
    }
}

fn main() {
    
    let kimeratriangle = Kimera::create(geom::Colors::GREEN, -0.2, -0.2, 0.0, 3.0, 2.0,1.5);    
    println!("Kimera shape!");
    let vcenter = kimeratriangle.center();
    println!("-> Center:({},{})", vcenter.x, vcenter.y);
    println!("-> Radius:{}",kimeratriangle.get_radius());
    println!("-> Perimeter:{}",kimeratriangle.perimeter());
    println!("-> Width:{}",kimeratriangle.get_width());
    println!("-> Is Solid:{}",kimeratriangle.is_solid());

    let testpoint = geom::Vec2::new(1f64, 1f64);

    println!("-> Intersects Point(1,1):{}",kimeratriangle.point_collides(&testpoint));
}
