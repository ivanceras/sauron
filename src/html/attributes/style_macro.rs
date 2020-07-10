/// a utility function for convenient styling of elements
#[macro_export]
macro_rules! style {
    (
        $($name:tt => $value:expr;)*
    ) => {
        $crate::html::attributes::styles_values([$((stringify!($name).trim_matches('"'), Into::<$crate::Value>::into($value))),*])
    };

}

#[cfg(test)]
mod tests {
    use crate::html::{
        units::px,
        *,
    };

    #[test]
    fn test_style_values() {
        let mut b1 = String::new();
        let s1: Attribute<()> =
            style! {hello=> "world"; width=> px(10); height=> px(20);};
        s1.render(&mut b1).unwrap();

        assert_eq!(b1, r#"style="hello:world;width:10px;height:20px;""#);

        let s2: Attribute<()> = style! {"background-color"=> "red";
        width=> px(10); height=> px(20);};

        let mut b2 = String::new();
        s2.render(&mut b2).unwrap();
        println!("s2: {:#?}", s2);

        assert_eq!(
            b2,
            r#"style="background-color:red;width:10px;height:20px;""#
        );

        struct Data {
            width: i32,
        }

        let data = Data { width: 101 };
        let padding_width = 200;

        let mut b3 = String::new();

        let s3: Attribute<()> = style! {
            width=> px(data.width);
            "padding-left"=> px(padding_width);
            "padding-right"=> px(padding_width);
        };
        s3.render(&mut b3).unwrap();
        assert_eq!(
            b3,
            r#"style="width:101px;padding-left:200px;padding-right:200px;""#
        );
    }
}
