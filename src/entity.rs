
pub trait Entity {
    fn indexarity(&self) -> usize;
}

#[macro_export]
macro_rules! add_entity {
    ($struct_name:ident { $($field_name:ident : $field_type:ty),* }) => {
        #[derive(Debug)]
        pub struct $struct_name {
            $($field_name : $field_type,)*
            ampl: *mut Ampl,
            name: String,
        }

        impl Entity for $struct_name {
            fn indexarity(&self) -> usize {
                let name = CString::new(&*self.name).unwrap();
                let mut indexarity: usize = 0;
                unsafe { ffi::AMPL_EntityGetIndexarity((*self.ampl).raw, name.as_ptr(), &mut indexarity as *mut usize) };
                indexarity
            }
        }
    }
}
