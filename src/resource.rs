use std::{borrow::Borrow, collections::HashMap, hash::Hash, rc::Rc};

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
};

pub type TextureManager<'a, T> = ResourceManager<'a, TextureCreator<T>, String, Texture<'a>>;

pub struct ResourceManager<'a, L, K, R>
where
    L: ResourceLoader<'a, R>,
    K: Hash + Eq,
{
    loader: &'a L,
    cache: HashMap<K, Rc<R>>,
}

impl<'a, K, L, R> ResourceManager<'a, L, K, R>
where
    L: ResourceLoader<'a, R>,
    K: Hash + Eq,
{
    pub fn new(loader: &'a L) -> Self {
        Self {
            loader,
            cache: HashMap::new(),
        }
    }

    pub fn load<A>(&mut self, arg: &A) -> Result<Rc<R>, String>
    where
        A: Hash + Eq + ?Sized,
        L: ResourceLoader<'a, R, Arg = A>,
        K: Borrow<A> + for<'b> From<&'b A>,
    {
        //                  Option<Rc<R>>
        self.cache.get(arg).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load(arg)?);
                self.cache.insert(arg.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }
}

pub trait ResourceLoader<'a, R> {
    type Arg: ?Sized;

    fn load(&'a self, arg: &Self::Arg) -> Result<R, String>;
}

impl<'a, T> ResourceLoader<'a, Texture<'a>> for TextureCreator<T> {
    type Arg = str;

    fn load(&'a self, arg: &Self::Arg) -> Result<Texture<'a>, String> {
        self.load_texture(arg)
    }
}
