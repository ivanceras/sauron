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
        let s1: Attribute<()> =
            style! {hello=> "world"; width=> px(10); height=> px(20);};

        assert_eq!(
            s1.to_pretty_string(),
            r#"style="hello:world;width:10px;height:20px;""#
        );

        let s2: Attribute<()> = style! {"background-color"=> "red";
        width=> px(10); height=> px(20);};
        println!("s2: {:#?}", s2);

        assert_eq!(
            s2.to_pretty_string(),
            r#"style="background-color:red;width:10px;height:20px;""#
        );

        struct Data {
            width: i32,
        }

        let data = Data { width: 101 };
        let padding_width = 200;

        let s3: Attribute<()> = style! {
            width=> px(data.width);
            "padding-left"=> px(padding_width);
            "padding-right"=> px(padding_width);
        };
        assert_eq!(
            s3.to_pretty_string(),
            r#"style="width:101px;padding-left:200px;padding-right:200px;""#
        );
    }
}
