#[macro_export]
macro_rules! strip_async {
    (   @ins( )
        @out( $( $out:tt )* )
    ) => {
        $( $out )*
    };
    (   @ins( async $( $ins:tt )* )
        @out( $( $out:tt )* )
    ) => {
        $crate::strip_async!{
            @ins( $( $ins )* )
            @out( $( $out )* )
        }
    };
    (   @ins( .await $( $ins:tt )* )
        @out( $( $out:tt )* )
    ) => {
        $crate::strip_async!{
            @ins( $( $ins )* )
            @out( $( $out )* )
        }
    };
    (   @ins( $token:tt $( $ins:tt )* )
        @out( $( $out:tt )* )
    ) => {
        $crate::strip_async!{
            @ins( $( $ins )* )
            @out( $( $out )* $token )
        }
    };
    (
        $( ins:tt )*
    ) => {

        $crate::strip_async!{
            @ins( $( $ins )* )
            @out(  )
        }
    };
}
