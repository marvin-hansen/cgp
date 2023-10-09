#[macro_export]
macro_rules! delegate_components {
    ( $target:ident $( < $( $param:ident ),* $(,)? > )? ; ) => {

    };
    (   $target:ident $( < $( $param:ident ),* $(,)? > )?;
        [ ] : $forwarded:ty
        $( , $( $rest:tt )* )?
    ) => {
        $crate::delegate_components!(
            $target $( < $( $param ),* > )* ;
            $( $( $rest )*  )?
        );
    };
    ( $target:ident $( < $( $param:ident ),* $(,)? > )? ;
        [ $name:ty $(, $($names:tt)* )?] : $forwarded:ty
        $( , $( $rest:tt )* )?
    ) => {
        $crate::delegate_component!(
            $target $( < $( $param ),* > )*;
            $name : $forwarded
        );

        $crate::delegate_components!(
            $target $( < $( $param ),* > )* ;
            [ $( $( $names )* )? ] : $forwarded
            $( , $( $rest )*  )?
        );
    };
    (   $target:ident $( < $( $param:ident ),* $(,)? > )? ;
        $name:ty : $forwarded:ty
        $( , $( $rest:tt )* )?
    ) => {
        $crate::delegate_component!(
            $target $( < $( $param ),* > )*;
            $name : $forwarded
        );

        $crate::delegate_components!(
            $target $( < $( $param ),* > )* ;
            $( $( $rest )*  )?
        );
    };
}
